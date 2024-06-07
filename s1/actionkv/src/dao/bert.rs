use chrono::prelude::*;
use futures::stream::{StreamExt, TryStreamExt};
use mongodb::bson::serde_helpers::{
    bson_datetime_as_rfc3339_string, chrono_datetime_as_bson_datetime,
};
use mongodb::{
    bson::{self, doc, oid::ObjectId, Bson},
    options::{self, IndexOptions},
    results::{InsertOneResult, UpdateResult}, //modify here
    Client,
    Collection,
    IndexModel,
};
use std::time::Duration;
use tokio::sync::OnceCell;
// 需要引入这个trait
use crate::config::{self, mongo::MONGO_CLIENT};
use crate::dao;

use crate::pb::{
    entity,
    svr::{
        bert::{BertReq, BertResp},
        ApiError,
    },
};
use crate::utils::{
    self,
    mongo::{local_date_format, serialize_object_id_option_as_hex_string},
};
use serde::{Deserialize, Serialize, Serializer};
use serde_json::to_string;
use std::hash::Hasher;
use std::result::Result;
use std::str::FromStr;
use std::vec;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BertEntity {
    #[serde(
        serialize_with = "serialize_object_id_option_as_hex_string",
        rename = "_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<ObjectId>,
    pub name: String,
    pub url: String,
    #[serde(with = "chrono_datetime_as_bson_datetime")] //只能支持utc
    pub created_at: DateTime<Utc>,
    // 修改时间
    // #[serde(serialize_with = "serialize_with_local_string")]
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
    pub deleted_at: i64,
}

impl entity::Namer for BertEntity {
    fn name(&self) -> &'static str {
        dao::ENTITY_BERT
    }
}

impl Default for BertEntity {
    fn default() -> Self {
        // let local: DateTime<Local> = Local::now();
        BertEntity {
            id: None,
            name: "".to_owned(),
            url: "".to_owned(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: 0,
        }
    }
}

impl From<BertReq> for BertEntity {
    fn from(value: BertReq) -> Self {
        BertEntity {
            id: None,
            name: value.name,
            url: value.url,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: 0,
        }
    }
}

impl Into<BertResp> for BertEntity {
    fn into(self) -> BertResp {
        let id_str = match self.id {
            Some(o) => o.to_hex(),
            None => "".to_owned(),
        };
        BertResp {
            id: id_str,
            name: self.name.clone(),
            url: self.url.clone(),
            created_at: utils::format_chrono_utc_to_local(&self.created_at),
            updated_at: utils::format_chrono_utc_to_local(&self.updated_at),
            deleted_at: self.deleted_at,
        }
    }
}

impl PartialEq<BertEntity> for BertEntity {
    fn eq(&self, other: &BertEntity) -> bool {
        self.name == other.name
    }
}

#[derive(Clone)]
pub struct BertRepo {
    pub col: Collection<BertEntity>,
}

impl BertRepo {
    pub async fn create_index() -> Result<(), ApiError> {
        let _configure = &config::cc::GLOBAL_CONFIG.lock().unwrap();
        let col: Collection<BertEntity> = utils::mongo::get_collection::<BertEntity>(
            &MONGO_CLIENT,
            &_configure.mongo.database,
            &_configure.table.bert,
        );

        let uniqueOpt = IndexOptions::builder()
            // .name()
            .unique(true)
            .background(true)
            .build();
        let opt = IndexOptions::builder()
            .unique(false)
            .background(true)
            .build();
        let mut indices = Vec::with_capacity(2);
        // note 没有指定名字，默认生成，导致问题是修改比较困难
        indices.push(
            IndexModel::builder()
                .keys(doc! {
                    "updated_at":-1,"deleted_at":-1,
                })
                .options(opt)
                .build(),
        );

        indices.push(
            IndexModel::builder()
                .keys(doc! {
                    "name":1,"deleted_at":-1,
                })
                .options(uniqueOpt)
                .build(),
        );
        let o = options::CreateIndexOptions::builder()
            .max_time(Duration::from_secs(600))
            .build();
        col.create_indexes(indices, o).await?;
        Ok(())
    }

    pub fn new() -> Self {
        let _configure = &config::cc::GLOBAL_CONFIG.lock().unwrap();
        let col = utils::mongo::get_collection::<BertEntity>(
            &MONGO_CLIENT,
            &_configure.mongo.database,
            &_configure.table.bert,
        );
        BertRepo { col }
    }

    pub async fn insert(&self, _bert: &BertEntity) -> Result<BertEntity, ApiError> //mongodb::error::Result<InsertOneResult>
    {
        // let opt = options::InsertOneOptions::build();
        let ret = self.col.insert_one(_bert, None).await?;
        info!("dao insert _bert {:?}", _bert);
        info!("inserted id = {:?}", &ret.inserted_id);
        // let mut entity = app.into();
        let _oid = match ret.inserted_id {
            Bson::ObjectId(_id) => Some(_id),
            _ => None,
        };
        let mut _bert2 = _bert.clone();
        _bert2.id = _oid;
        Ok(_bert2)
    }

    pub async fn get(&self, _id: impl AsRef<str>) -> Result<BertEntity, ApiError> {
        let mongo_id = ObjectId::parse_str(_id)?;
        let opt = options::FindOneOptions::builder()
            .show_record_id(true)
            .build();
        let ret = self.col.find_one(doc! {"_id":mongo_id}, opt).await?;
        Ok(ret.unwrap_or_default())
    }

    pub async fn get_by_name(&self, name: impl AsRef<str>) -> Result<BertEntity, ApiError> {
        let opt = options::FindOneOptions::builder()
            .show_record_id(true)
            .build();

        let ret = self
            .col
            .find_one(doc! {"name":name.as_ref(),"deleted_at":0}, opt)
            .await?;
        Ok(ret.unwrap_or_default())
    }

    pub async fn list(&self, skip: u64, limit: i64) -> Result<Vec<BertEntity>, ApiError> {
        let opt = options::FindOptions::builder()
            .sort(doc! {"updated_at":-1})
            .skip(Some(skip))
            .limit(Some(limit))
            .build();

        let mut cursor = self.col.find(doc! {"deleted_at":0}, opt).await?;
        let mut v = Vec::new();
        while let Some(doc) = cursor.next().await {
            if doc.is_ok() {
                v.push(doc.unwrap_or_default());
            }
        }
        Ok(v)
    }
    pub async fn delete_app_by_id(&self, id: impl AsRef<str>) -> Result<BertEntity, ApiError> {
        let opt = options::FindOneAndDeleteOptions::builder().build();
        let oid = ObjectId::parse_str(id)?;
        let ret = self.col.find_one_and_delete(doc! {"_id": oid}, opt).await?;
        // ret.ok().expect("");
        Ok(ret.unwrap_or_default())
    }
}

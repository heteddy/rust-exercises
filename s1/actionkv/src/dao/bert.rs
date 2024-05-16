use chrono::prelude::*;
use futures::stream::{StreamExt, TryStreamExt};
use mongodb::bson::serde_helpers::{
    bson_datetime_as_rfc3339_string,
    chrono_datetime_as_bson_datetime,
    // hex_string_as_object_id,
    // serialize_object_id_as_hex_string,
};
use mongodb::{
    bson::{self, doc, oid::ObjectId, Bson},
    options::{self, IndexOptions},
    results::{InsertOneResult, UpdateResult}, //modify here
    Client,
    // Client,
    Collection,
    IndexModel,
};
use std::time::Duration;
use tokio::sync::OnceCell;
// 需要引入这个trait
use serde::{Deserialize, Serialize, Serializer};
// 这个是derive 宏
use crate::config::{self, mongo::MONGO_CLIENT};
use crate::dao;
use crate::pb::svr::bert::{BertReq, BertResp};
use crate::pb::svr::ApiError;
use crate::utils::{
    self,
    mongo::{local_date_format, serialize_object_id_option_as_hex_string},
};
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
                .options(opt.clone())
                .build(),
        );

        indices.push(
            IndexModel::builder()
                .keys(doc! {
                    "name":1,"deleted_at":-1,
                })
                .options(opt.clone())
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
}

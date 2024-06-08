// use chrono::{DateTime, Utc};
use crate::utils;
use chrono::prelude::*;
use futures::stream::StreamExt;
use mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::{self, IndexOptions},
    Collection, IndexModel,
};
use std::time::Duration;
// 需要引入这个trait
use serde::{Deserialize, Serialize};
// 这个是derive 宏
use crate::config::{self, mongo::MONGO_CLIENT};
use crate::dao;
use crate::pb::{self, svr::ApiError};
use crate::utils::mongo::serialize_object_id_option_as_hex_string;
use std::result::Result;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerEntity {
    #[serde(
        serialize_with = "serialize_object_id_option_as_hex_string",
        rename = "_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<ObjectId>,
    pub name: String,
    pub http_addr: String,
    pub grpc_addr: String,
    #[serde(with = "chrono_datetime_as_bson_datetime")] //只能支持utc
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
    pub deleted_at: i64,
}

impl PartialEq<ServerEntity> for ServerEntity {
    fn eq(&self, other: &ServerEntity) -> bool {
        self.name == other.name
    }
}

impl pb::entity::Namer for ServerEntity {
    fn name(&self) -> &'static str {
        dao::ENTITY_SERVER
    }
}

// 实现from和into接口

#[derive(Clone)]
pub struct ServerRepo {
    pub col: Collection<ServerEntity>,
}

impl ServerRepo {
    pub async fn create_index() -> Result<(), ApiError> {
        let _configure = &config::cc::GLOBAL_CONFIG.lock().unwrap();
        let col = utils::mongo::get_collection::<ServerEntity>(
            &MONGO_CLIENT,
            &_configure.mongo.database,
            &_configure.table.app,
        );
        let uniqueOpt = IndexOptions::builder()
            .unique(true)
            .background(true)
            .build();
        let opt = IndexOptions::builder()
            .unique(false)
            .background(true)
            .build();

        let mut indices = Vec::with_capacity(3);

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
                    "name":-1,"deleted_at":-1,
                })
                .options(uniqueOpt)
                .build(),
        );
        let o = options::CreateIndexOptions::builder()
            .max_time(Duration::from_secs(60))
            .build();
        col.create_indexes(indices, o).await?;
        Ok(())
    }

    pub fn new() -> Self {
        let config_file = config::cc::GLOBAL_CONFIG.lock().unwrap();
        let col = utils::mongo::get_collection(
            &MONGO_CLIENT,
            &config_file.mongo.database,
            &config_file.table.preprocess,
        );
        ServerRepo { col }
    }

    pub async fn list(&self, skip: u64, limit: i64) -> Result<Vec<ServerEntity>, ApiError> {
        let opt = options::FindOptions::builder()
            .sort(doc! {"updated_at":-1,"deleted_at":1})
            .limit(Some(limit))
            .skip(Some(skip))
            .build();
        let filters = doc! {"deleted_at":0};
        let mut cursor = self.col.find(filters, opt).await?;
        let mut v = Vec::new();
        while let Some(doc) = cursor.next().await {
            if doc.is_ok() {
                v.push(doc.unwrap_or_default());
            }
        }
        Ok(v)
    }

    pub async fn get_by_name(&self, name: impl AsRef<str>) -> Result<ServerEntity, ApiError> {
        let opt = options::FindOneOptions::builder()
            .show_record_id(true)
            .build();

        let ret = self.col.find_one(doc! {"name":name.as_ref()}, opt).await?;
        Ok(ret.unwrap_or_default())
    }

    pub async fn get(&self, _id: impl AsRef<str>) -> Result<ServerEntity, ApiError> {
        let opt = options::FindOneOptions::builder()
            .show_record_id(true)
            .build();
        let _id = ObjectId::parse_str(_id.as_ref())?;

        let ret = self.col.find_one(doc! {"name":_id}, opt).await?;
        Ok(ret.unwrap_or_default())
    }

    pub async fn update(
        &self,
        _id: impl AsRef<str>,
        mut e: ServerEntity,
    ) -> Result<ServerEntity, ApiError> {
        let opt = options::FindOneAndUpdateOptions::builder()
            .upsert(false)
            .build();
        let _id = ObjectId::parse_str(_id.as_ref())?;
        let ret = self
            .col
            .find_one_and_update(
                doc! {
                    "_id":_id,
                },
                doc! {
                    "http_addr": e.http_addr.clone(),
                    "grpc_addr": e.grpc_addr.clone(),
                    "updated_at": Utc::now(),
                },
                opt,
            )
            .await?;
        e.id = Some(_id);
        Ok(e)
    }

    pub async fn delete(&self, _id: impl AsRef<str>) -> Result<ServerEntity, ApiError> {
        let opt = options::FindOneAndDeleteOptions::builder().build();
        let _id = ObjectId::parse_str(_id)?;
        let ret = self
            .col
            .find_one_and_delete(
                doc! {
                    "_id":_id,
                },
                opt,
            )
            .await?;
        Ok(ret.unwrap_or_default())
    }
}

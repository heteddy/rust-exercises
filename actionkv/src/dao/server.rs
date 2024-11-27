use crate::pb::{
    entity,
    svr::{
        server::{ServerReq, ServerResp},
        ApiError,
    },
};
use crate::utils;
use chrono::prelude::*;
use futures::stream::StreamExt;
use mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::{self, IndexOptions},
    Collection, IndexModel,
};
use std::time::Duration;
// 需要引入这个trait
use serde::{Deserialize, Serialize};
// 这个是derive 宏
use crate::config::{self, mongo::MONGO_CLIENT};
use crate::dao::base::{Entity, EntityDao};
use crate::utils::mongo::serialize_object_id_option_as_hex_string;
use std::result::Result;

pub const ENTITY_SERVER: &'static str = "server_entity";

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
    pub api_key: String,
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

impl entity::Namer for ServerEntity {
    fn name(&self) -> &'static str {
        ENTITY_SERVER
    }
}

impl From<ServerReq> for ServerEntity {
    fn from(value: ServerReq) -> Self {
        ServerEntity {
            id: None,
            name: value.name,
            http_addr: value.http_addr,
            grpc_addr: value.grpc_addr,
            api_key: value.api_key,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: 0,
        }
    }
}

impl Into<ServerResp> for ServerEntity {
    fn into(self) -> ServerResp {
        let id_str = match self.id {
            Some(_id) => _id.to_hex(),
            None => String::new(),
        };

        ServerResp {
            id: id_str,
            name: self.name,
            http_addr: self.http_addr,
            grpc_addr: self.grpc_addr,
            api_key: self.api_key,
            created_at: utils::format_chrono_utc_to_local(&self.created_at),
            updated_at: utils::format_chrono_utc_to_local(&self.updated_at),
            deleted_at: self.deleted_at,
        }
    }
}
impl Entity for ServerEntity {
    fn update(&mut self, id: Option<ObjectId>, updated_at: DateTime<Utc>) {
        self.id = id;
        self.updated_at = updated_at;
    }
    fn updating_doc(&self, rhs: &Self) -> Document {
        doc! {
            "http_addr": rhs.http_addr.clone(),
            "grpc_addr": rhs.grpc_addr.clone(),
            "updated_at": rhs.updated_at,
        }
    }
}

// 实现from和into接口

#[derive(Clone)]
pub struct ServerDao {
    collection: Collection<ServerEntity>,
}

impl EntityDao<ServerEntity> for ServerDao {
    fn new() -> Self {
        let config_file = config::cc::GLOBAL_CONFIG.lock().unwrap();
        let col = utils::mongo::get_collection(
            &MONGO_CLIENT,
            &config_file.mongo.database,
            &config_file.table.server,
        );
        ServerDao { collection: col }
    }

    fn col(&self) -> Collection<ServerEntity> {
        self.collection.clone()
    }
    fn indices(&self) -> Vec<IndexModel> {
        let uniqueOpt = IndexOptions::builder()
            // .name()
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
        indices
    }
}

// impl ServerDao {
//     pub fn new() -> Self {
//         let config_file = config::cc::GLOBAL_CONFIG.lock().unwrap();
//         let col = utils::mongo::get_collection(
//             &MONGO_CLIENT,
//             &config_file.mongo.database,
//             &config_file.table.server,
//         );
//         ServerDao { collection: col }
//     }
// }

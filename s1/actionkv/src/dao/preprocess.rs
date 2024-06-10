use crate::dao::base::{Entity, EntityDao};
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
use crate::pb;
use crate::pb::svr::preprocess::PreprocessResp;
use crate::pb::svr::{preprocess::PreprocessReq, ApiError};
use crate::utils::mongo::serialize_object_id_option_as_hex_string;
use crate::{dao, utils};
use std::result::Result;

pub const ENTITY_PREPROCESS: &'static str = "preprocess_entity";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PreprocessEntity {
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
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
    pub deleted_at: i64,
}

impl From<PreprocessReq> for PreprocessEntity {
    fn from(value: PreprocessReq) -> Self {
        Self {
            id: None,
            name: value.name,
            url: value.url,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: 0,
        }
    }
}
impl Into<PreprocessResp> for PreprocessEntity {
    fn into(self) -> PreprocessResp {
        let id_str = match self.id {
            Some(_id) => _id.to_hex(),
            None => String::new(),
        };

        PreprocessResp {
            id: id_str,
            name: self.name,
            url: self.url,
            created_at: utils::format_chrono_utc_to_local(&self.created_at),
            updated_at: utils::format_chrono_utc_to_local(&self.updated_at),
            deleted_at: self.deleted_at,
        }
    }
}

impl PartialEq<PreprocessEntity> for PreprocessEntity {
    fn eq(&self, other: &PreprocessEntity) -> bool {
        self.name == other.name
    }
}

impl pb::entity::Namer for PreprocessEntity {
    fn name(&self) -> &'static str {
        ENTITY_PREPROCESS
    }
}

impl Entity for PreprocessEntity {
    fn update(&mut self, id: Option<ObjectId>, updated_at: DateTime<Utc>) {
        self.id = id;
        self.updated_at = updated_at;
    }
    fn updating_doc(&self, rhs: &Self) -> Document {
        doc! {
            "url": rhs.url.clone(),
            "updated_at": rhs.updated_at,
        }
    }
}
#[derive(Clone)]
pub struct PreprocessDao {
    collection: Collection<PreprocessEntity>,
}

impl EntityDao<PreprocessEntity> for PreprocessDao {
    fn new() -> Self {
        let config_file = config::cc::GLOBAL_CONFIG.lock().unwrap();
        let col = utils::mongo::get_collection(
            &MONGO_CLIENT,
            &config_file.mongo.database,
            &config_file.table.preprocess,
        );
        PreprocessDao { collection: col }
    }
    fn col(&self) -> Collection<PreprocessEntity> {
        self.collection.clone()
    }
    fn indices(&self) -> Vec<IndexModel> {
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
        indices
    }
}

// impl PreprocessDao {
//     pub fn new() -> Self {
//         let config_file = config::cc::GLOBAL_CONFIG.lock().unwrap();
//         let col = utils::mongo::get_collection(
//             &MONGO_CLIENT,
//             &config_file.mongo.database,
//             &config_file.table.preprocess,
//         );
//         PreprocessDao { collection: col }
//     }
// }

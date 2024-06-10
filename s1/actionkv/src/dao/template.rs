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
use crate::pb::{
    entity,
    svr::{
        template::{TemplateReq, TemplateResp},
        ApiError,
    },
};
use crate::utils::format_chrono_utc_to_local;
use crate::utils::mongo::serialize_object_id_option_as_hex_string;
use std::result::Result;

pub const ENTITY_TEMPLATE: &'static str = "template_entity";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TemplateEntity {
    #[serde(
        serialize_with = "serialize_object_id_option_as_hex_string",
        rename = "_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<ObjectId>,
    pub name: String,
    pub body: String, // 定义一个
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
    pub deleted_at: i64,
}

impl PartialEq<TemplateEntity> for TemplateEntity {
    fn eq(&self, other: &TemplateEntity) -> bool {
        self.name == other.name
    }
}

impl entity::Namer for TemplateEntity {
    fn name(&self) -> &'static str {
        ENTITY_TEMPLATE
    }
}

impl From<TemplateReq> for TemplateEntity {
    fn from(value: TemplateReq) -> Self {
        Self {
            id: None,
            name: value.name,
            body: value.body,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: 0,
        }
    }
}

impl Into<TemplateResp> for TemplateEntity {
    fn into(self) -> TemplateResp {
        let id_str = match self.id {
            Some(id) => id.to_hex(),
            None => String::new(),
        };
        TemplateResp {
            id: id_str,
            name: self.name,
            body: self.body,
            created_at: utils::format_chrono_utc_to_local(&self.created_at),
            updated_at: utils::format_chrono_utc_to_local(&self.updated_at),
            deleted_at: self.deleted_at,
        }
    }
}

impl Entity for TemplateEntity {
    fn update(&mut self, id: Option<ObjectId>, updated_at: DateTime<Utc>) {
        self.id = id;
        self.updated_at = updated_at;
    }
    fn updating_doc(&self, rhs: &Self) -> Document {
        doc! {
            "body": rhs.body.clone(),
            "updated_at": rhs.updated_at.clone(),
        }
    }
}
#[derive(Clone)]
pub struct TemplateDao {
    collection: Collection<TemplateEntity>,
}

impl EntityDao<TemplateEntity> for TemplateDao {
    fn new() -> Self {
        let config_file = config::cc::GLOBAL_CONFIG.lock().unwrap();
        let col = utils::mongo::get_collection(
            &MONGO_CLIENT,
            &config_file.mongo.database,
            &config_file.table.template,
        );
        TemplateDao { collection: col }
    }
    fn col(&self) -> Collection<TemplateEntity> {
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

// impl TemplateDao {
//     pub fn new() -> Self {
//         let config_file = config::cc::GLOBAL_CONFIG.lock().unwrap();
//         let col = utils::mongo::get_collection(
//             &MONGO_CLIENT,
//             &config_file.mongo.database,
//             &config_file.table.template,
//         );
//         TemplateDao { collection: col }
//     }
// }

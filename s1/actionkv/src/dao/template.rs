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
use crate::pb::svr::template::{TemplateReq, TemplateResp};
use crate::pb::{self, svr::ApiError};
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

impl pb::entity::Namer for TemplateEntity {
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




pub struct TemplateDao {

}



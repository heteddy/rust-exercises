// use chrono::{DateTime, Utc};
use chrono::prelude::*;
use futures::stream::{StreamExt, TryStreamExt};
use mongodb::bson::serde_helpers::{
    bson_datetime_as_rfc3339_string,
    chrono_datetime_as_bson_datetime,
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
use serde::{Deserialize, Serialize, Serializer};
// 这个是derive 宏
use crate::config::{self, mongo::MONGO_CLIENT};
use crate::dao;
use crate::pb;
use crate::utils::mongo::{local_date_format, serialize_object_id_option_as_hex_string};
use serde_json::to_string;
use std::hash::Hasher;
use std::result::Result;
use std::str::FromStr;
use std::vec;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

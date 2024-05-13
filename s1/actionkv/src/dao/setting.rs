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
use crate::pb;
use crate::utils::mongo::{bson_datetime_as_string, serialize_object_id_option_as_hex_string};
use serde_json::to_string;
use std::hash::Hasher;
use std::result::Result;
use std::str::FromStr;
use std::vec;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    pub id: Option<ObjectId>,
    pub app_id: String,
    pub name: String,
    pub active: String,
    pub inactive: String,
    pub replica: u32,
    pub shards: u32,
    pub vector_size: u32,
    pub bert: String,
    pub server: String,
    pub preprocess: String,
    #[serde(with = "bson_datetime_as_string")]
    pub created_at: bson::DateTime,
    #[serde(with = "bson_datetime_as_string")]
    pub updated_at: bson::DateTime,
    pub deleted_at: u64,
}

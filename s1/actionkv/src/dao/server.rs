// use chrono::{DateTime, Utc};
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
use serde_derive::{Deserialize as DeserializeMacro, Serialize as SerializeMacro};
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
    id: Option<ObjectId>,
    name: String,
    http_addr: String,
    grpc_addr: String,
    #[serde(with = "bson_datetime_as_string")]
    created_at: bson::DateTime,
    #[serde(with = "bson_datetime_as_string")]
    updated_at: bson::DateTime,
    deleted_at: u64,
}

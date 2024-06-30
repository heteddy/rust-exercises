use serde::{Deserialize, Serialize};
use serde_json::{value, Map};
use std::collections::HashMap;
use validator::Validate;

#[derive(Debug, Clone, Default, Validate, Deserialize, Serialize)]
#[serde(default)]
pub struct InboundDataReq {
    pub id: String,
    body: HashMap<String, value::Value>,
    version: String,
}

impl InboundDataReq {}

#[derive(Debug, Clone, Default, Validate, Deserialize, Serialize)]
pub struct SearchReq {
    request_id: String,
    param: HashMap<String, value::Value>,
    tpl: String,
    size: i32,
}

pub struct SearchRet {
    Count: i32,
    Data: Vec<HashMap<String, value::Value>>,
}

// 创建一个collection
#[derive(Debug, Clone, Default, Validate, Deserialize, Serialize)]
pub struct CollectionReq {
    pub request_id: String,
    pub name: String,
}

// 列出所有的collection
#[derive(Debug, Clone, Default, Validate, Deserialize, Serialize)]
pub struct ListCollectionReq {
    pub request_id: String,
    pub server: String, // 服务器名称
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CollectionInfoReq {
    pub request_id: String,
    pub server: String, // 服务器名称
}


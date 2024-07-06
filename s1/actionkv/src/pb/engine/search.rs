use crate::pb::engine::qdrant::points::{PointStruct, UpsertPoints};
use serde::{Deserialize, Serialize};
use serde_json::{value, Map};
use std::collections::HashMap;
use validator::Validate;

#[derive(Debug, Clone, Default, Validate, Deserialize, Serialize)]
#[serde(default)]
pub struct InboundDataReq {
    pub id: String,
    pub payload: HashMap<String, value::Value>,
    pub vector: Vec<f32>,
    pub version: String,
}

impl Into<PointStruct> for InboundDataReq {
    fn into(self) -> PointStruct {
        PointStruct {
            id: Some(self.id),
            payload: self.payload,
            vector: self.vector,
        }
    }
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

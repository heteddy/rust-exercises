use crate::pb::engine::qdrant::points::{PointStruct, UpsertPoints};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;
use validator::Validate;

#[derive(Debug, Clone, Default, Validate, Deserialize, Serialize)]
#[serde(default)]
pub struct InboundDataReq {
    #[validate(length(equal = 32, message = "数据id必须是32位"))]
    pub id: String,
    pub payload: HashMap<String, Value>,
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
    #[validate(length(min = 5, message = "request id必须至少5位"))]
    pub request_id: String,
    pub params: HashMap<String, Value>,
    pub template: String,
    pub limit: i32,
}

pub struct SearchRet {
    Count: i32,
    Data: Vec<HashMap<String, Value>>,
}

// 创建一个collection
#[derive(Debug, Clone, Default, Validate, Deserialize, Serialize)]
pub struct CollectionReq {
    #[validate(length(min = 5, message = "request id必须至少5位"))]
    pub request_id: String,
    pub name: String,
}

// 列出所有的collection
#[derive(Debug, Clone, Default, Validate, Deserialize, Serialize)]
pub struct ListCollectionReq {
    #[validate(length(min = 5, message = "request id必须至少5位"))]
    pub request_id: String,
    pub server: String, // 服务器名称
}

#[derive(Debug, Clone, Default, Validate, Deserialize, Serialize)]
pub struct CollectionInfoReq {
    #[validate(length(min = 5, message = "request id必须至少5位"))]
    pub request_id: String,
    pub server: String, // 服务器名称
}

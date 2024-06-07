use serde::{Deserialize, Serialize, Serializer};
use validator::Validate;

#[derive(Debug, Clone, Default, Validate, Deserialize, Serialize)]
#[serde(default)]
pub struct ServerReq {
    pub name: String,
    pub http_addr: String,
    pub grpc_addr: String,
}

// 使用默认的default实现

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ServerResp {
    pub id: String,
    pub name: String,
    pub http_addr: String,
    pub grpc_addr: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: i64,
}

impl PartialEq<ServerResp> for ServerResp {
    fn eq(&self, other: &ServerResp) -> bool {
        self.name == other.name
    }
}

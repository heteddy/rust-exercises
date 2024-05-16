use crate::dao::bert::BertEntity;
use serde::{Deserialize, Serialize, Serializer};
use validator::Validate;
// use axum_valid::Valid;

#[derive(Debug, Clone, Validate, Deserialize, Serialize)]
#[serde(default)]
pub struct BertReq {
    #[validate(length(min = 3, message = "app_id 至少要大于3"))]
    pub name: String,
    pub url: String,
}

impl Default for BertReq {
    fn default() -> Self {
        BertReq {
            name: String::new(),
            url: String::new(),
        }
    }
}

// 在dao中定义转换方式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BertResp {
    pub id: String,
    pub name: String,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: i64,
}

impl Default for BertResp {
    fn default() -> Self {
        BertResp {
            id: String::default(),
            name: String::default(),
            url: String::default(),
            created_at: String::default(),
            updated_at: String::default(),
            deleted_at: 0,
        }
    }
}

impl PartialEq<BertResp> for BertResp {
    fn eq(&self, other: &BertResp) -> bool {
        self.name == other.name
    }
}

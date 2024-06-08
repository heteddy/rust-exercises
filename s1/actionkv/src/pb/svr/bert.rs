use serde::{Deserialize, Serialize};
use validator::Validate;
// use axum_valid::Valid;

#[derive(Debug, Clone, Default, Validate, Deserialize, Serialize)]
#[serde(default)]
pub struct BertReq {
    #[validate(length(min = 3, message = "app_id 至少要大于3"))]
    pub name: String,
    pub url: String,
}

// 在dao中定义转换方式
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BertResp {
    pub id: String,
    pub name: String,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: i64,
}

impl PartialEq<BertResp> for BertResp {
    fn eq(&self, other: &BertResp) -> bool {
        self.name == other.name
    }
}

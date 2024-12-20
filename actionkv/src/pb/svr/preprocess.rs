use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Default, Validate, Deserialize, Serialize)]
#[serde(default)]
pub struct PreprocessReq {
    #[validate(length(min = 3, message = "name长度至少大于3"))]
    pub name: String,
    pub url: String,
}

// 使用默认的default实现

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PreprocessResp {
    pub id: String,
    pub name: String,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: i64,
}

impl PartialEq<PreprocessResp> for PreprocessResp {
    fn eq(&self, other: &PreprocessResp) -> bool {
        self.name == other.name
    }
}

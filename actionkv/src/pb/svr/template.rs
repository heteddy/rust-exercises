use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Default, Validate, Deserialize, Serialize)]
#[serde(default)]
pub struct TemplateReq {
    pub name: String,
    pub body: String,
}

// 使用默认的default实现

#[derive(Debug, Clone, Default, Validate, Serialize, Deserialize)]
#[serde(default)]
pub struct TemplateResp {
    pub id: String,
    #[validate(length(min = 3, message = "name长度至少大于3"))]
    pub name: String,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: i64,
}

impl PartialEq<TemplateResp> for TemplateResp {
    fn eq(&self, other: &TemplateResp) -> bool {
        self.name == other.name
    }
}

use serde::{Deserialize, Serialize, Serializer};
use validator::Validate;
// use axum_valid::Valid;

#[derive(Debug, Clone, Default, Validate, Deserialize, Serialize)]
#[serde(default)]
pub struct AppReq {
    #[validate(length(min = 3, message = "app_id 至少要大于3"))]
    pub app_id: String,
    pub app_secret: String,
    // 租户名称，
    pub tenant: String,
    // 联系人
    pub liaison: String,
    //子系统名称
    #[validate(length(min = 3, message = "子系统 名称至少要大于3"))]
    pub system: String,
}

// 在dao中定义转换方式
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct AppResp {
    pub id: String,
    pub app_id: String,
    pub app_secret: String,
    // 租户名称，
    pub tenant: String,
    // 联系人
    pub liaison: String,
    //子系统名称
    pub system: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: i64,
}

// 这个有什么用途？
impl PartialEq<AppResp> for AppResp {
    fn eq(&self, other: &AppResp) -> bool {
        self.app_id == other.app_id
    }
}

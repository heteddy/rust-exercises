use axum::{extract::Json};
use serde_derive::{Deserialize, Serialize};
use validator::Validate;
use axum_valid::Valid;

#[derive(Debug, Clone, Validate, Deserialize, Serialize)]
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


impl std::default::Default for AppReq {
    fn default() -> Self {
        AppReq {
            app_id: String::new(),
            app_secret: String::new(),
            tenant: String::new(),
            liaison: String::new(),
            system: String::new(),
        }
    }
}
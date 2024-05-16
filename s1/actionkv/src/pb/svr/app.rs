// use axum::{extract::Json};
use crate::dao::app::AppEntity;
use serde::{Deserialize, Serialize, Serializer};
use validator::Validate;
// use axum_valid::Valid;

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

impl Default for AppReq {
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

// 在dao中定义转换方式
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Default for AppResp {
    fn default() -> Self {
        AppResp {
            id: String::default(),
            app_id: String::default(),
            app_secret: String::default(),
            tenant: String::default(),
            liaison: String::default(),
            system: String::default(),
            created_at: String::default(),
            updated_at: String::default(),
            deleted_at: 0,
        }
    }
}

impl PartialEq<AppResp> for AppResp {
    fn eq(&self, other: &AppResp) -> bool {
        self.app_id == other.app_id
    }
}

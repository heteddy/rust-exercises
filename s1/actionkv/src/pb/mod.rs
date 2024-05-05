use axum::{http::StatusCode, response::Response};
use serde::{de::DeserializeOwned, Serialize, Deserialize}; // 这个serialize是trait， serde_derive是宏
// use serde_derive::{Deserialize,Serialize};

pub mod app;
pub mod data;

pub const CODE_SUCCESS: StatusCode = StatusCode::OK;
pub const CODE_FAIL: StatusCode = StatusCode::BAD_REQUEST;

// 要重新定义一个结构
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RespV0<T> {
    pub code: Option<u16>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> RespV0<T>
    where
        T: Serialize + DeserializeOwned + Clone,
{
    pub fn from_result(arg: &T) -> Self {
        Self {
            code: Some(CODE_SUCCESS.as_u16()),
            msg: Some("操作成功".to_string()),
            data: Some(arg.clone()),
        }
    }

    pub fn from_error(arg: &str) -> Self {
        Self {
            code: Some(CODE_FAIL.as_u16()),
            msg: Some(arg.to_string()),
            data: None,
        }
    }

    pub fn from_error_info(code: StatusCode, info: &str) -> Self {
        Self {
            code: Some(code.as_u16()),
            msg: Some(info.to_string()),
            data: None,
        }
    }
}

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
// use serde_derive::{Deserialize,Serialize};
use axum::Json;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

// 这个serialize是trait， serde_derive是宏
pub mod app;
pub mod data;
pub mod error;

pub const CODE_SUCCESS: StatusCode = StatusCode::OK;


// 要重新定义一个结构
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiResponse<T> {
    pub code: Option<u16>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> ApiResponse<T>
    where
        T: Serialize + DeserializeOwned + Clone,
{
    pub fn from_result(arg: T) -> Self {  // 直接move T
        Self {
            code: Some(CODE_SUCCESS.as_u16()),
            msg: Some("操作成功".to_string()),
            data: Some(arg), // 是不是可以改成arc
        }
    }

    // pub fn from_error(err: error::ApiError) -> Self {
    //     Self {
    //         code: Some(CODE_INTERNAL_ERROR.as_u16()),
    //         msg: Some(err.to_string()),
    //         data: None,
    //     }
    // }

    // pub fn from_error_info(code: StatusCode, info: &str) -> Self {
    //     Self {
    //         code: Some(code.as_u16()),
    //         msg: Some(info.to_string()),
    //         data: None,
    //     }
    // }
}

impl<T> IntoResponse for ApiResponse<T>
    where
        T: Serialize + DeserializeOwned + Clone,
{
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}



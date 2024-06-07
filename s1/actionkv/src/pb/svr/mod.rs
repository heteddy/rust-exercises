use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::error::Error;
// use serde_derive::{Deserialize,Serialize};
use axum::Json;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use std::fmt::{self, Display};
use std::io::Error as IOErr;

// 这个serialize是trait， serde_derive是宏
pub mod app;
pub mod bert;
pub mod data;
pub mod error;
pub mod index;
pub mod preprocess;
pub mod server;

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
    pub fn from_result(arg: T) -> Self {
        // 直接move T
        Self {
            code: Some(CODE_SUCCESS.as_u16()),
            msg: Some("操作成功".to_string()),
            data: Some(arg), // 是不是可以改成arc
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug)]
pub struct InternalError(String);

impl Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl Error for InternalError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // 泛型错误，没有记录其内部原因。
        None
    }
}

#[derive(Debug)] // 没有实现clone和Serialize
pub enum ApiError {
    IOError(IOErr),
    DBError(mongodb::error::Error),
    InternalServerError(InternalError),
    BsonError(bson::oid::Error),
}

// impl ApiErrorTrait for ApiError {}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl Error for ApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            // 这里self是引用
            Self::IOError(e) => Some(e),
            Self::DBError(e) => Some(e),
            Self::InternalServerError(e) => Some(e),
            Self::BsonError(e) => Some(e),
            // _ => None,
        }
    }
}

// 实现from trait
impl From<bson::oid::Error> for ApiError {
    fn from(value: bson::oid::Error) -> Self {
        Self::BsonError(value)
    }
}

impl From<IOErr> for ApiError {
    fn from(value: IOErr) -> Self {
        Self::IOError(value)
    }
}

impl From<mongodb::error::Error> for ApiError {
    fn from(value: mongodb::error::Error) -> Self {
        Self::DBError(value)
    }
}

impl From<InternalError> for ApiError {
    fn from(value: InternalError) -> Self {
        Self::InternalServerError(value)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        println!(
            "error={:?}",
            json!(
                {
                    "code":CODE_INTERNAL_ERROR.as_u16(),
                    "msg":format!("{:?}", &self),
                }
            )
        );
        let msg = format!("{:?}", &self);
        let err_resp: ErrorResponse = ErrorResponse::new(&msg);
        
        let status_code = match self {
            // Self::IOError(e) => Some(e),
            // Self::DBError(e) => Some(e),
            // Self::InternalServerError(e) => Some(e),
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status_code, Json(err_resp)).into_response()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorResponse<'a> {
    code: Option<u16>,
    msg: &'a str,
}

pub const CODE_INTERNAL_ERROR: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;
pub const CODE_BAD_REQUEST: StatusCode = StatusCode::BAD_REQUEST;

impl<'a> ErrorResponse<'a> {
    fn new(e: &'a str) -> Self {
        ErrorResponse {
            code: Some(CODE_INTERNAL_ERROR.as_u16()),
            msg: e,
        }
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct Pagination {
    pub skip: u64,
    pub limit: i64,
}

impl Pagination {}

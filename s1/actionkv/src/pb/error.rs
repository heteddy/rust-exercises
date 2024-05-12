use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use bson;
use mongodb;
use serde::{de::DeserializeOwned, de::IntoDeserializer, Deserialize, Serialize};
use serde_json::{self, json};
use std::convert::From;
use std::error::Error;
use std::fmt::{self, Display};
use std::io::Error as IOErr;

// 定义

#[derive(Debug)]
pub struct InteralError(String);

// impl IntoResponse for InteralError {
//     fn into_response(self) -> Response {
//         self.0.into_response()
//     }
// }

impl fmt::Display for InteralError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl std::error::Error for InteralError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // 泛型错误，没有记录其内部原因。
        None
    }
}

#[derive(Debug)] // 没有实现clone和Serialize
pub enum ApiError {
    IOError(IOErr),
    DBError(mongodb::error::Error),
    InternalServerError(InteralError),
    BsonError(bson::oid::Error),
}

// impl ApiErrorTrait for ApiError {}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl std::error::Error for ApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
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

impl From<InteralError> for ApiError {
    fn from(value: InteralError) -> Self {
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

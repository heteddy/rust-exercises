use axum::{http::StatusCode, response::Response};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use self::user::User;

pub mod user;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseBody {
    status: &'static str,
    msg: String,
    Data: ReturnType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReturnType {
    // 返回一个User
    Single(Option<User>),
    // 返回多个User
    Mutiple(Option<Vec<User>>),
    // 返回
}

impl ResponseBody {
    pub fn new_sucess_single(user: Option<User>) -> Self {
        ResponseBody {
            status: "sucess",
            msg: "".to_owned(),
            Data: ReturnType::Single(user),
        }
    }
    pub fn new_sucess_multiple(users: Option<Vec<User>>) -> Self {
        ResponseBody {
            status: "sucess",
            msg: "".to_owned(),
            Data: ReturnType::Mutiple(users),
        }
    }
}

//常量
pub const CODE_SUCCESS: StatusCode = StatusCode::OK;
pub const CODE_FAIL: StatusCode = StatusCode::BAD_REQUEST;

/// http接口返回模型结构，提供基础的 code，msg，data 等json数据结构
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RespVO<T> {
    pub code: Option<u16>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> RespVO<T>
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

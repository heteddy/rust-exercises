use serde::{Deserialize, Serialize};
use std::ffi::CString;

use std::sync::Arc;

use crate::utils; //直接引用lib.rs就行了，不需要外部

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromClient {
    #[serde(rename = "join")]
    Join { group_name: Arc<String> },
    #[serde(rename = "post")]
    Post {
        group_name: Arc<String>,
        message: Arc<String>,
    },
}

impl FromClient {
    pub fn newPost(name: String, message: String) -> Self {
        FromClient::Post {
            group_name: Arc::new(name),
            message: Arc::new(message),
        }
    }
    pub fn newJoin(name: String) -> Self {
        FromClient::Join {
            group_name: Arc::new(name),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromServer {
    Message {
        group_name: Arc<String>,
        message: Arc<String>,
    },
    Error(String),
}

#[cfg(test)] // 只在运行cargo test时候才编译和测试，cargo build并不编译这部分代码
pub mod pbtests {
    use super::*;

    #[test]
    fn test_from_client_json() {
        let from_client = FromClient::Post {
            group_name: Arc::new("Dogs".to_owned()),
            message: Arc::new("Samoyeds rock!".to_string()),
        };
        let json = serde_json::to_string(&from_client).unwrap();
        assert_eq!(
            json,
            r#"{"post":{"group_name":"Dogs","message":"Samoyeds rock!"}}"#
        );
        assert_eq!(
            serde_json::from_str::<FromClient>(&json).unwrap(),
            from_client // 这里move了
        );
    }
}

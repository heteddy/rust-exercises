use serde::{Deserialize, Serialize};

use std::sync::Arc;

use crate::utils; //直接引用lib.rs就行了，不需要外部

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromClient {
    Join {
        group_name: Arc<String>,
    },
    Post {
        group_name: Arc<String>,
        message: Arc<String>,
    },
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromServer {
    Message {
        group_name: Arc<String>,
        msg: String,
    },
    Error(String),
}

pub mod tests {
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
            r#"{"Post":{"group_name":"Dogs","message":"Samoyeds rock!"}}"#
        );
        assert_eq!(
            serde_json::from_str::<FromClient>(&json).unwrap(),
            from_client // 这里move了
        );
    }
}

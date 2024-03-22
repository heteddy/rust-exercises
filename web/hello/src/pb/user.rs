use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: u64,
    username: String,
}

impl User {
    pub fn new(id: u64, username: &str) -> Self {
        User {
            id,
            username: username.to_owned(),
        }
    }
}

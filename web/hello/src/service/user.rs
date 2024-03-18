//!
//!
//!
//! 这里是具体的服务
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use std::sync::RwLock;

use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};

pub struct UserLastAccess {
    user: Arc<RwLock<HashMap<String, DateTime<Local>>>>,
}

// type UserCenter = Arc<Rwlock<UserLastAccess>>;

impl UserLastAccess {
    pub fn new() -> Self {
        UserLastAccess {
            user: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn add_user(&mut self, username: String) {
        let mut guard = self.user.write().unwrap();
        let n = Local::now();
        println!(" {:?} join at {:?}", username, n);
        guard.insert(username, n);
    }

    pub fn get_user(&self, username: &str) -> Option<DateTime<Local>> {
        let mut guard = self.user.read().unwrap();
        let o = guard.get(username);
        o.copied()
    }
}

pub struct User {
    id: u64,
    username: String,
}

impl User {}

pub struct UserRepo {
    users: RwLock<HashMap<u64, User>>,
}

impl UserRepo {
    pub fn new() -> Self {
        UserRepo {
            users: RwLock::new(HashMap::new()),
        }
    }
    pub fn get_user(&self, username: &str) -> Option<&User> {
        None
    }
    
    pub fn create_user(&self, id: u64, username: &str) {
        let mut guard = self.users.write().unwrap();
        guard.insert(
            id,
            User {
                id: id,
                username: username.to_owned(),
            },
        );
    }
}

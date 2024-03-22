//!
//!
//!
//! 这里是具体的服务
use crate::pb::user::{self, User};
use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};
use serde::Serialize;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use tracing::{event, Level,instrument};
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
        // guard.insert(username, n);
        guard.entry(username).and_modify(|v| *v = Local::now());
    }

    pub fn get_user(&self, username: &str) -> Option<DateTime<Local>> {
        let mut guard = self.user.read().unwrap();
        let o = guard.get(username);
        o.copied()
    }
}

#[derive(Debug, Clone)] // 这里必须是clone
pub struct UserRepo {
    users: Arc<RwLock<HashMap<u64, Arc<User>>>>,
}

impl UserRepo {
    pub fn new() -> Self {
        UserRepo {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    #[instrument(skip(self))]
    pub fn create_user(&self, id: u64, username: &str) -> User {
        event!(Level::INFO, "creating user");
        let mut guard = self.users.write().unwrap();
        let u = Arc::new(User::new(id, username));

        guard.insert(id, u.clone());
        event!(Level::INFO, "created user");
        (*u).clone() // *u是deref，然后调用&t的clone()
    }
    #[instrument(skip(self))]
    pub fn get_user(&self, id: u64) -> Option<Arc<User>> {
        event!(Level::INFO, "get user id {:?}", id);
        let mut guard = self.users.write().unwrap();
        let ret = guard.get(&id);
        event!(Level::INFO, "get user ret {:?}", ret);
        ret.cloned()
    }
}

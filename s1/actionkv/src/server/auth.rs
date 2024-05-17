use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
// use tokio::sync::Mutex;
// use tokio::sync::RwLock;

lazy_static! {
    // 要共享一个mut的state 就需要mutex; 在tokio中共享需要arc
    pub static ref TENANT_AUTH_SVC: Arc<Mutex<TenantAuthSvc>> = Arc::new(Mutex::new(TenantAuthSvc::new()));
}

pub trait AuthTrait {
    // trait 默认是pub
    fn auth(
        &self,
        app_id: impl AsRef<str>,
        app_secret: impl AsRef<str>,
        name: impl AsRef<str>,
    ) -> bool;
}

#[derive(Debug, Clone)]
struct AuthInfo {
    name: String,
    app_id: String,
    app_secret: String,
}

#[derive(Debug)]
pub struct TenantAuthSvc {
    index_auth_table: HashMap<String, AuthInfo>,
    counter: u64,
}

impl TenantAuthSvc {
    pub fn new() -> Self {
        TenantAuthSvc {
            index_auth_table: HashMap::new(),
            counter: 0,
        }
    }
    pub fn test(&self) -> bool {
        true
    }
    pub fn add_counter(&mut self) {
        self.counter += 1;
    }
}

impl AuthTrait for TenantAuthSvc {
    fn auth(
        &self,
        app_id: impl AsRef<str>,
        app_secret: impl AsRef<str>,
        name: impl AsRef<str>,
    ) -> bool {
        true
    }
}

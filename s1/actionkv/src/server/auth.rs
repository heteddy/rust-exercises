use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::RwLock;

lazy_static! {
    pub static ref TENANT_AUTH_SVC: TenantAuthSvc = TenantAuthSvc::new();
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

#[derive(Debug, Clone)]
pub struct TenantAuthSvc {
    index_auth_table: Arc<RwLock<HashMap<String, AuthInfo>>>,
}

impl TenantAuthSvc {
    pub fn new() -> Self {
        TenantAuthSvc {
            index_auth_table: Arc::new(RwLock::new(HashMap::new())),
        }
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

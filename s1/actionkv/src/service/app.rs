// use crate::pb;
use crate::config;
use crate::dao;
use std::sync::{Arc, RwLock};
use tracing::{event, Level, instrument, info};
use crate::dao::app::AppEntity;
// use mongodb::error::Error as MongoError;
// use mongodb::{options::ClientOptions, Client};

// use tokio::sync::OnceCell;
#[derive(Clone)]
pub struct AppService {
    repo: dao::app::AppRepo,
}

impl AppService {
    pub fn new() -> Self {
        AppService {
            repo: dao::app::AppRepo::init(
                &config::cc::GLOBAL_CONFIG.lock().unwrap().mongo.database,
                "tb_app_collection")
        }
    }
    #[instrument(skip_all)]
    pub async fn create_app_service(&self, app: dao::app::AppEntity) -> dao::app::AppEntity {
        info!("insert app {:?}",app.app_id);
        let _ = self.repo.insert_app(&app).await;
        app
    }
    #[instrument(skip_all)]
    pub async fn list_all(&self, skip: u64, limit: i64) -> Vec<Result<AppEntity, mongodb::error::Error>> {
        info!("list_all apps");
        let ret = self.repo.list(skip, limit).await;
        ret
    }
}
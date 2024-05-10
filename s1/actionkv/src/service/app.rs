// use crate::pb;
use crate::config;
use crate::dao;
use crate::pb;
use std::sync::{Arc, RwLock};

use crate::dao::app::AppEntity;
use std::convert::AsRef;
use tracing::{event, info, instrument, Level};
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
                "tb_app_collection",
            ),
        }
    }
    #[instrument(skip_all)]
    pub async fn create_app_service(
        &self,
        app: dao::app::AppEntity,
    ) -> Result<dao::app::AppEntity, pb::error::ApiError> {
        info!("insert app {:?}", app.app_id);
        let _app = self.repo.insert_app(&app).await?;
        Ok(_app)
    }
    #[instrument(skip_all)]
    pub async fn list_all(
        &self,
        skip: u64,
        limit: i64,
    ) -> Result<Vec<AppEntity>, pb::error::ApiError> {
        info!("list_all apps");
        let ret = self.repo.list(skip, limit).await?;
        Ok(ret)
    }

    #[instrument(skip_all)]
    pub async fn get_app_by_id(
        &self,
        _id: impl AsRef<str> + std::fmt::Debug,
    ) -> Result<AppEntity, pb::error::ApiError> {
        info!("get_app_by_id apps :{:?}", _id);
        let ret = self.repo.get_app(_id).await?;
        Ok(ret)
    }
}

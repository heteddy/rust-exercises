// use crate::pb;
use crate::config;
use crate::dao;
use crate::dao::app::AppEntity;
use crate::pb::svr::{ApiResponse, ApiError};
use std::convert::AsRef;
use std::fmt::{Debug, Display};
use std::sync::{Arc, RwLock};
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
            repo: dao::app::AppRepo::new(),
        }
    }
    #[instrument(skip_all)]
    pub async fn create_app(
        &self,
        app: dao::app::AppEntity,
    ) -> Result<dao::app::AppEntity, ApiError> {
        info!("insert app {:?}", app.app_id);
        let _app = self.repo.insert_app(&app).await?;
        Ok(_app)
    }

    #[instrument(skip_all)]
    pub async fn update_app(
        &self,
        _id: impl AsRef<str> + Debug,
        app: AppEntity,
    ) -> Result<AppEntity, ApiError> {
        info!("update app {:?}", app.app_id);
        let _app = self.repo.update_app_by_id(_id, &app).await?;
        Ok(_app)
    }

    #[instrument(skip_all)]
    pub async fn list_all(
        &self,
        skip: u64,
        limit: i64,
    ) -> Result<Vec<AppEntity>, ApiError> {
        info!("list_all apps");
        let ret = self.repo.list(skip, limit).await?;
        Ok(ret)
    }

    #[instrument(skip_all)]
    pub async fn get_app_by_id(
        &self,
        _id: impl AsRef<str> + Debug,
    ) -> Result<AppEntity, ApiError> {
        info!("get_app_by_id apps :{:?}", _id);
        let ret = self.repo.get_app_by_id(_id).await?;
        Ok(ret)
    }

    #[instrument(skip_all)]
    pub async fn delete_app_by_id(
        &self,
        _id: impl AsRef<str> + Debug,
    ) -> Result<AppEntity, ApiError> {
        info!("delete_app_by_id apps :{:?}", _id);
        let ret = self.repo.soft_delete_app_by_id(_id).await?;
        Ok(ret)
    }
}

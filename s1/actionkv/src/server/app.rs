// use crate::pb;
use crate::cache::{repo, sync};
use crate::config;
use crate::dao::app::{AppEntity, AppRepo};
use crate::pb::svr::{ApiError, ApiResponse};
use std::convert::AsRef;
use std::fmt::{Debug, Display};
use tokio::sync::mpsc;
use tracing::{event, info, instrument, Level};

// use tokio::sync::OnceCell;
#[derive(Clone)]
pub struct AppSvc {
    repo: AppRepo,
    sender: mpsc::Sender<sync::SyncData>,
}

impl AppSvc {
    pub fn new(tx: mpsc::Sender<sync::SyncData>) -> Self {
        AppSvc {
            repo: AppRepo::new(),
            // 通过sender发送到
            sender: tx,
            // sender: repo::GLOBAL_SYNCHRONIZER.lock().unwrap().get_tx(),
        }
    }
    #[instrument(skip_all)]
    pub async fn create_app(&self, app: AppEntity) -> Result<AppEntity, ApiError> {
        info!("insert app {:?}", app.app_id);
        let _app = self.repo.insert_app(&app).await?;

        self.sender
            .send(sync::SyncData::build::<AppEntity>("app", &_app))
            .await;
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
        self.sender
            .send(sync::SyncData::build::<AppEntity>("app", &_app))
            .await;
        // match self.sender.send(repo::SyncMsg::App(_app.clone())).await {
        //     _ => {

        //     }
        // }
        Ok(_app)
    }

    #[instrument(skip_all)]
    pub async fn list_all(&self, skip: u64, limit: i64) -> Result<Vec<AppEntity>, ApiError> {
        info!("list_all apps");
        let ret = self.repo.list(skip, limit).await?;
        Ok(ret)
    }

    #[instrument(skip_all)]
    pub async fn get_app_by_id(&self, _id: impl AsRef<str> + Debug) -> Result<AppEntity, ApiError> {
        info!("get_app_by_id apps :{:?}", _id);
        let ret = self.repo.get(_id).await?;
        Ok(ret)
    }
    ///todo add 删除逻辑
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

use crate::cache::sync;
use crate::dao::base::EntityDao;
use crate::dao::server::{ServerDao, ServerEntity};
use crate::pb::svr::ApiError;
use std::convert::AsRef;
use std::fmt::Debug;
use tokio::sync::mpsc;
use tracing::{info, instrument};

#[derive(Clone)]
pub struct ServerSvc {
    sender: mpsc::Sender<sync::SyncData>,
    repo: ServerDao,
}

impl ServerSvc {
    pub fn new(sender: mpsc::Sender<sync::SyncData>) -> Self {
        Self {
            sender,
            repo: ServerDao::new(),
        }
    }

    #[instrument(skip_all)]
    pub async fn create(&self, e: ServerEntity) -> Result<ServerEntity, ApiError> {
        let _e = self.repo.insert(e).await?;
        let _ = self
            .sender
            .send(sync::SyncData::build::<ServerEntity>("server", &_e))
            .await;
        Ok(_e)
    }
    #[instrument(skip_all)]
    pub async fn delete(&self , id:impl AsRef<str>) -> Result<ServerEntity, ApiError> {
        let _e = self.repo.delete(id).await?;
        // let _ = self
        //     .sender
        //     .send(sync::SyncData::build::<TemplateEntity>("tempalte", &_e))
        //     .await;
        // todo 删除的时候增加
        Ok(_e)
    }
    #[instrument(skip_all)]
    pub async fn update(&self , id:impl AsRef<str>,e: ServerEntity) -> Result<ServerEntity, ApiError> {
        let _e = self.repo.update(id, e).await?;
        let _ = self
            .sender
            .send(sync::SyncData::build::<ServerEntity>("server", &_e))
            .await;
        Ok(_e)
    }
    #[instrument(skip_all)]
    pub async fn list(&self , skip: u64, limit:i64) -> Result<Vec<ServerEntity>, ApiError> {
        let _e = self.repo.list(skip,limit).await?;
        
        Ok(_e)
    }
    #[instrument(skip_all)]
    pub async fn get(&self , id:impl AsRef<str>) -> Result<ServerEntity, ApiError> {
        let _e = self.repo.get(id).await?;
        Ok(_e)
    }

}

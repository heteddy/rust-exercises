use crate::cache::sync;
use crate::dao::base::EntityDao;
use crate::dao::bert::{BertDao, BertEntity};
use crate::pb::svr::ApiError;
use std::fmt;
use tokio::sync::mpsc;
use tracing::{info, instrument};

#[derive(Clone)]
pub struct BertSvc {
    repo: BertDao,
    sender: mpsc::Sender<sync::SyncData>,
}

impl BertSvc {
    pub fn new(tx: mpsc::Sender<sync::SyncData>) -> Self {
        BertSvc {
            repo: BertDao::new(),
            sender: tx,
        }
    }
    #[instrument(skip_all)]
    pub async fn create(&self, _bert: BertEntity) -> Result<BertEntity, ApiError> {
        info!("insert bert {:?}", _bert.name);

        let _e = self.repo.insert(_bert).await?;
        let _ = self
            .sender
            .send(sync::SyncData::build::<BertEntity>("bert", &_e))
            .await;
        Ok(_e)
    }
    
    #[instrument(skip_all)]
    pub async fn update(
        &self,
        _id: impl AsRef<str> + fmt::Debug,
        e: BertEntity,
    ) -> Result<BertEntity, ApiError> {
        info!("update app {:?}", e.name);

        let _e = self.repo.update(_id, e).await?;
        let _ = self
            .sender
            .send(sync::SyncData::build::<BertEntity>("bert", &_e))
            .await;
        Ok(_e)
    }
    #[instrument(skip_all)]
    pub async fn list(&self, skip: u64, limit: i64) -> Result<Vec<BertEntity>, ApiError> {
        info!("list_all apps");
        let ret = self.repo.list(skip, limit).await?;
        Ok(ret)
    }

    #[instrument(skip_all)]
    pub async fn get(&self, _id: impl AsRef<str> + fmt::Debug) -> Result<BertEntity, ApiError> {
        info!("get_id  :{:?}", _id);
        let ret = self.repo.get(_id).await?;
        Ok(ret)
    }
    ///todo add 删除逻辑
    #[instrument(skip_all)]
    pub async fn delete(&self, _id: impl AsRef<str> + fmt::Debug) -> Result<BertEntity, ApiError> {
        info!("delete_app_by_id apps :{:?}", _id);
        let ret = self.repo.delete(_id).await?;
        Ok(ret)
    }
}

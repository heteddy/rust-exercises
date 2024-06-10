use crate::cache::sync;
use crate::dao::base::EntityDao;
use crate::dao::preprocess::{PreprocessDao, PreprocessEntity};
use crate::pb::svr::ApiError;
use std::convert::AsRef;
use std::fmt::Debug;
use tokio::sync::mpsc;
use tracing::{info, instrument};

#[derive(Clone)]
pub struct PreprocessSvc {
    sender: mpsc::Sender<sync::SyncData>,
    repo: PreprocessDao,
}

impl PreprocessSvc {
    pub fn new(sender: mpsc::Sender<sync::SyncData>) -> Self {
        Self {
            sender,
            repo: PreprocessDao::new(),
        }
    }
    #[instrument(skip_all)]
    pub async fn create(&self, e: PreprocessEntity) -> Result<PreprocessEntity, ApiError> {
        info!("insert {:?}", e);
        let _ = self
            .sender
            .send(sync::SyncData::build::<PreprocessEntity>("preprocess", &e))
            .await;
        let _e = self.repo.insert(e).await?;

        Ok(_e)
    }

    #[instrument(skip_all)]
    pub async fn update(
        &self,
        _id: impl AsRef<str> + Debug,
        e: PreprocessEntity,
    ) -> Result<PreprocessEntity, ApiError> {
        let _e = self.repo.update(_id, e).await?;
        let _ = self
            .sender
            .send(sync::SyncData::build::<PreprocessEntity>("preprocess", &_e))
            .await;
        Ok(_e)
    }

    #[instrument(skip_all)]
    pub async fn list(&self, skip: u64, limit: i64) -> Result<Vec<PreprocessEntity>, ApiError> {
        info!("list_all");
        let ret = self.repo.list(skip, limit).await?;
        Ok(ret)
    }

    #[instrument(skip_all)]
    pub async fn get(&self, _id: impl AsRef<str> + Debug) -> Result<PreprocessEntity, ApiError> {
        info!("get id={:?}", _id);
        let ret = self.repo.get(_id).await?;
        Ok(ret)
    }

    ///todo add 删除逻辑
    #[instrument(skip_all)]
    pub async fn delete(&self, _id: impl AsRef<str> + Debug) -> Result<PreprocessEntity, ApiError> {
        info!("delete id :{:?}", _id);
        let ret = self.repo.delete(_id).await?;
        Ok(ret)
    }
}

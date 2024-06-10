use crate::cache::sync;
use crate::dao::base::EntityDao;
use crate::dao::template::{TemplateDao, TemplateEntity};
use crate::pb::svr::ApiError;
use std::convert::AsRef;
use std::fmt::Debug;
use tokio::sync::mpsc;
use tracing::{info, instrument};

pub struct TemplateSvc {
    sender: mpsc::Sender<sync::SyncData>,
    repo: TemplateDao,
}

impl TemplateSvc {
    pub fn new(sender: mpsc::Sender<sync::SyncData>) -> Self {
        Self {
            sender,
            repo: TemplateDao::new(),
        }
    }

    #[instrument(skip_all)]
    pub async fn create(&self, e: TemplateEntity) -> Result<TemplateEntity, ApiError> {
        let _e = self.repo.insert(e).await?;
        let _ = self
            .sender
            .send(sync::SyncData::build::<TemplateEntity>("tempalte", &_e))
            .await;
        Ok(_e)
    }
    #[instrument(skip_all)]
    pub async fn delete(&self , id:impl AsRef<str>) -> Result<TemplateEntity, ApiError> {
        let _e = self.repo.delete(id).await?;
        // let _ = self
        //     .sender
        //     .send(sync::SyncData::build::<TemplateEntity>("tempalte", &_e))
        //     .await;
        // todo 删除的时候增加
        Ok(_e)
    }
    #[instrument(skip_all)]
    pub async fn update(&self , id:impl AsRef<str>,e: TemplateEntity) -> Result<TemplateEntity, ApiError> {
        let _e = self.repo.update(id, e).await?;
        let _ = self
            .sender
            .send(sync::SyncData::build::<TemplateEntity>("tempalte", &_e))
            .await;
        Ok(_e)
    }
    #[instrument(skip_all)]
    pub async fn list(&self , skip: u64, limit:i64) -> Result<Vec<TemplateEntity>, ApiError> {
        let _e = self.repo.list(skip,limit).await?;
        
        Ok(_e)
    }
    #[instrument(skip_all)]
    pub async fn get(&self , id:impl AsRef<str>) -> Result<TemplateEntity, ApiError> {
        let _e = self.repo.get(id).await?;
        Ok(_e)
    }

}

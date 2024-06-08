use crate::cache::sync;
use crate::dao::bert::{BertEntity, BertRepo};
use crate::dao::base::EntityDao;
use crate::pb::svr::ApiError;
use tokio::sync::mpsc;
use tracing::{info, instrument};

#[derive(Clone)]
pub struct BertSvc {
    repo: BertRepo,
    sender: mpsc::Sender<sync::SyncData>,
}

impl BertSvc {
    pub fn new(tx: mpsc::Sender<sync::SyncData>) -> Self {
        BertSvc {
            repo: BertRepo::new(),
            sender: tx,
        }
    }
    #[instrument(skip_all)]
    pub async fn create(&self, _bert: BertEntity) -> Result<BertEntity, ApiError> {
        info!("insert bert {:?}", _bert.name);
        let _app = self.repo.insert(_bert).await?;
        Ok(_app)
    }
}

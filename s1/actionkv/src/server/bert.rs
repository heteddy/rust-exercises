use crate::cache::chan;
use crate::dao::bert::{BertEntity, BertRepo};
use crate::pb::svr::{ApiError, ApiResponse};
use tokio::sync::mpsc;
use tracing::{event, info, instrument, Level};

#[derive(Clone)]
pub struct BertSvc {
    repo: BertRepo,
    sender: mpsc::Sender<chan::SyncData>,
}

impl BertSvc {
    pub fn new(tx: mpsc::Sender<chan::SyncData>) -> Self {
        BertSvc {
            repo: BertRepo::new(),
            sender: tx,
        }
    }
    #[instrument(skip_all)]
    pub async fn create(&self, _bert: BertEntity) -> Result<BertEntity, ApiError> {
        info!("insert bert {:?}", _bert.name);
        let _app = self.repo.insert(&_bert).await?;
        Ok(_app)
    }
}

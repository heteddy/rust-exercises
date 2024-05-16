use crate::dao::bert::{BertEntity, BertRepo};
use crate::pb::svr::{ApiError, ApiResponse};
use tracing::{event, info, instrument, Level};
#[derive(Clone)]
pub struct BertSvc {
    repo: BertRepo,
}

impl BertSvc {
    pub fn new() -> Self {
        BertSvc {
            repo: BertRepo::new(),
        }
    }
    #[instrument(skip_all)]
    pub async fn create(&self, _bert: BertEntity) -> Result<BertEntity, ApiError> {
        info!("insert bert {:?}", _bert.name);
        let _app = self.repo.insert(&_bert).await?;
        Ok(_app)
    }
}

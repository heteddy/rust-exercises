use crate::cache::repo;
use crate::dao::base::EntityDao;
use crate::dao::index::{IndexDao, IndexEntity};
use crate::pb::engine::search;
use crate::pb::svr::ApiError;
use chrono::prelude::*;
use std::convert::AsRef;
use std::fmt::Debug;
use tokio::sync::mpsc;
use tracing::{info, instrument};

#[derive(Clone)]
pub struct CollectionSvc {}

impl CollectionSvc {
    pub fn new() -> Self {
        CollectionSvc {}
    }

    #[instrument(skip_all)]
    pub async fn create(&self, req: search::CollectionReq) -> anyhow::Result<()> {
        let r = repo::IndexConfigRepo::get_instance();
        let i = r.read().unwrap().get_index(&(req.name));
        if let Some(ref e) = i {
            // 构造创建index的配置参数
            info!("create index {:?}",e.name);
            

        }
        anyhow::Ok(())
    }
}

use crate::cache::repo;
use crate::dao::base::EntityDao;
use crate::dao::index::{IndexDao, IndexEntity};
use crate::engine::qdrant::collection::{create, delete, list, list_alias, update, update_alias};
use crate::pb::engine::qdrant::collection::{CollectionOperationResponse, CreateCollection};
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
    pub async fn create(
        &self,
        req: search::CollectionReq,
    ) -> anyhow::Result<CollectionOperationResponse> {
        let r = repo::IndexConfigRepo::get_instance();
        let i = r.read().unwrap().get_index(&(req.name));
        if let Some(ref e) = i {
            // 构造创建index的配置参数
            info!("create index {:?}", e.name);
            let req: CreateCollection = e.clone().into();
            let svr_name = e.configure.server.clone();
            let svr_entity = r.read().unwrap().server.get(svr_name);
            let svr_host = match svr_entity {
                Some(host) => host.http_addr.clone(),
                None => String::new(),
            };
            if svr_host.len() == 0 {
                anyhow::Result::Err(anyhow::anyhow!("not found server {:?}", svr_host))
            }else{
                create(svr_host, req).await
            }
        } else {
            anyhow::Result::Err(anyhow::anyhow!("not found index entity {:?}", req.name))
        }
    }
}

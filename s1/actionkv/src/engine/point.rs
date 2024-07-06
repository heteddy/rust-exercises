use crate::cache::repo;
use crate::cache::sync;
use crate::dao::base::EntityDao;
use crate::dao::index::{IndexDao, IndexEntity};
use crate::driver::qdrant::index;
use crate::driver::qdrant::points::{
    delete_points, discover_points, get_points, recommend_points, search_points, upsert_points,
};
use crate::pb::engine::qdrant::collection as pb_collection;
use crate::pb::engine::qdrant::points as pb_points;
use crate::pb::engine::qdrant::points::UpsertPoints;
use crate::pb::engine::search;
use crate::pb::svr::ApiError;
use crate::server::index::IndexSvc;
use chrono::prelude::*;
use std::convert::AsRef;
use std::fmt::Debug;
use tokio::sync::mpsc;
use tracing::{event, info, instrument, span, Level};

#[derive(Clone)]
pub struct PointSvc {}

impl PointSvc {
    pub fn new() -> Self {
        PointSvc {}
    }
    pub async fn upsert_points(
        &self,
        name: String, // 直接move进来，免得多次构造
        point: pb_points::PointStruct,
    ) -> anyhow::Result<pb_points::PointsOperationResponse> {
        let r = repo::IndexConfigRepo::get_instance();
        let host = r.read().unwrap().get_svr_http_address(&name);
        if host.is_none() {
            return anyhow::Result::Err(anyhow::anyhow!("not found host {:?}", &name));
        }
        let active = r.read().unwrap().get_active_collection(&name);
        if active.is_none() {
            return anyhow::Result::Err(anyhow::anyhow!("not found active collection {:?}", &name));
        }
        let _points = pb_points::UpsertPoints {
            // collection_name: active.unwrap().clone(), // 不需要collection name了
            points: vec![point],
            ..Default::default()
        };
        upsert_points(host.unwrap(), active.unwrap(), _points).await
    }

    pub async fn delete_points(
        &self,
        name: String,
        req: pb_points::DeletePoints,
    ) -> anyhow::Result<pb_points::PointsOperationResponse> {
        let r = repo::IndexConfigRepo::get_instance();
        let host = r.read().unwrap().get_svr_http_address(&name);
        if host.is_none() {
            return anyhow::Result::Err(anyhow::anyhow!("not found host {:?}", &name));
        }
        let active = r.read().unwrap().get_active_collection(&name);
        if active.is_none() {
            return anyhow::Result::Err(anyhow::anyhow!("not found active collection {:?}", &name));
        }
        delete_points(host.unwrap(), active.unwrap(), req).await
    }

    pub async fn get_points(
        &self,
        name: String,
        req: pb_points::GetPoints,
    ) -> anyhow::Result<pb_points::GetResponse> {
        let r = repo::IndexConfigRepo::get_instance();
        let host = r.read().unwrap().get_svr_http_address(&name);
        if host.is_none() {
            return anyhow::Result::Err(anyhow::anyhow!("not found host {:?}", &name));
        }
        let active = r.read().unwrap().get_active_collection(&name);
        if active.is_none() {
            return anyhow::Result::Err(anyhow::anyhow!("not found active collection {:?}", &name));
        }
        get_points(host.unwrap(), active.unwrap(), req).await
    }
}

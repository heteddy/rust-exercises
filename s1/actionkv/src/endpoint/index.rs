/// 实现配置索引信息
use crate::cache::repo;
use crate::cache::sync::SyncData;
use crate::dao::index::IndexEntity;
use crate::pb::svr::{index::IndexReq, index::IndexResp, ApiError, ApiResponse, Pagination};
use crate::server;
use crate::transport::middleware::auth::auth_middleware;
use axum::extract::{Json, Path, Query, State};
use axum::handler::Handler;
use axum::middleware::from_fn_with_state;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::convert::From;
use tokio::sync::mpsc;
use tracing::{event, instrument, span, Level};

#[instrument(skip_all)]
async fn create(
    State(svc): State<server::index::IndexSvc>,
    Json(mut payload): Json<IndexReq>,
) -> Result<ApiResponse<IndexResp>, ApiError> {

    let e = svc.create(IndexEntity::from(payload)).await?;
    Ok(ApiResponse::from_result(e.into()))
}
#[instrument(skip_all)]
async fn retrieve(
    State(svc): State<server::index::IndexSvc>,
    Path(id): Path<String>,
) -> Result<ApiResponse<IndexResp>, ApiError> {
    let e = svc.get(id).await?;
    Ok(ApiResponse::from_result(e.into()))
}
#[instrument(skip_all)]
async fn update(
    State(svc): State<server::index::IndexSvc>,
    Path(id): Path<String>,
    Json(payload): Json<IndexReq>,
) -> Result<ApiResponse<IndexResp>, ApiError> {
    let e = svc.update(id, IndexEntity::from(payload)).await?;
    Ok(ApiResponse::from_result(e.into()))
}
#[instrument(skip_all)]
async fn list(
    State(svc): State<server::index::IndexSvc>,
    Query(p): Query<Pagination>,
) -> Result<ApiResponse<Vec<IndexResp>>, ApiError> {
    let list = svc.list(p.skip, p.limit).await?;
    let ret = list.into_iter().map(|e| e.into()).collect();
    Ok(ApiResponse::from_result(ret))
}
#[instrument(skip_all)]
async fn del(
    State(svc): State<server::index::IndexSvc>,
    Path(id): Path<String>,
) -> Result<ApiResponse<IndexResp>, ApiError> {
    let e = svc.delete(id).await?;
    Ok(ApiResponse::from_result(e.into()))
}


pub fn register_route(tx: mpsc::Sender<SyncData>) -> Router {
    let mut _router = Router::new();
    let svc: server::index::IndexSvc = server::index::IndexSvc::new(tx);
    _router = _router.route("/indices/:id", get(retrieve).put(update).delete(del));
    _router = _router.route("/indices", get(list).post(create));
    Router::new().nest("/api", _router).with_state(svc)
}

// 实现配置索引信息

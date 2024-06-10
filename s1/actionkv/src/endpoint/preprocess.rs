use crate::cache::sync::{self, SyncData};
use crate::dao::preprocess::PreprocessEntity;
use crate::pb::svr::{
    preprocess::PreprocessReq, preprocess::PreprocessResp, ApiError, ApiResponse, Pagination,
};
use crate::server;
use axum::extract::{Json, Path, Query, State};
use axum::http::header::HeaderMap;
use axum::{
    routing::{get, post},
    Router,
};
use std::convert::From;
use tokio::sync::mpsc;
use tracing::{event, instrument, span, Level};

async fn create(
    State(svc): State<server::preprocess::PreprocessSvc>,
    Json(payload): Json<PreprocessReq>,
) -> Result<ApiResponse<PreprocessResp>, ApiError> {
    let e = svc.create(PreprocessEntity::from(payload)).await?;
    Ok(ApiResponse::from_result(e.into()))
}
async fn retrieve(
    State(svc): State<server::preprocess::PreprocessSvc>,
    Path(id): Path<String>,
) -> Result<ApiResponse<PreprocessResp>, ApiError> {
    let e = svc.get(id).await?;
    Ok(ApiResponse::from_result(e.into()))
}
async fn update(
    State(svc): State<server::preprocess::PreprocessSvc>,
    Path(id): Path<String>,
    Json(payload): Json<PreprocessReq>,
) -> Result<ApiResponse<PreprocessResp>, ApiError> {
    let e = svc.update(id, PreprocessEntity::from(payload)).await?;
    Ok(ApiResponse::from_result(e.into()))
}
async fn list(
    State(svc): State<server::preprocess::PreprocessSvc>,
    Query(p): Query<Pagination>,
) -> Result<ApiResponse<Vec<PreprocessResp>>, ApiError> {
    let list = svc.list(p.skip, p.limit).await?;
    let ret = list.into_iter().map(|e| e.into()).collect();
    Ok(ApiResponse::from_result(ret))
}
async fn del(
    State(svc): State<server::preprocess::PreprocessSvc>,
    Path(id): Path<String>,
) -> Result<ApiResponse<PreprocessResp>, ApiError> {
    let e = svc.delete(id).await?;
    Ok(ApiResponse::from_result(e.into()))
}

pub fn register_route(tx: mpsc::Sender<SyncData>) -> Router {
    let mut _router = Router::new();
    let svc: server::preprocess::PreprocessSvc = server::preprocess::PreprocessSvc::new(tx);
    _router = _router.route("/preprocess/:id", get(retrieve).put(update).delete(del));
    _router = _router.route("/preprocess", get(list).post(create));

    Router::new().nest("/api", _router).with_state(svc)
}

use crate::cache::sync::{self, SyncData};
use crate::dao::server::{ServerDao, ServerEntity};
use crate::pb::svr::{
    server::ServerReq, server::ServerResp, ApiError, ApiResponse, Pagination,
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

#[instrument(skip_all)]
async fn create(
    State(svc): State<server::server::ServerSvc>,
    Json(payload): Json<ServerReq>,
) -> Result<ApiResponse<ServerResp>, ApiError> {
    let e = svc.create(ServerEntity::from(payload)).await?;
    Ok(ApiResponse::from_result(e.into()))
}
#[instrument(skip_all)]
async fn retrieve(
    State(svc): State<server::server::ServerSvc>,
    Path(id): Path<String>,
) -> Result<ApiResponse<ServerResp>, ApiError> {
    let e = svc.get(id).await?;
    Ok(ApiResponse::from_result(e.into()))
}
#[instrument(skip_all)]
async fn update(
    State(svc): State<server::server::ServerSvc>,
    Path(id): Path<String>,
    Json(payload): Json<ServerReq>,
) -> Result<ApiResponse<ServerResp>, ApiError> {
    let e = svc.update(id, ServerEntity::from(payload)).await?;
    Ok(ApiResponse::from_result(e.into()))
}
#[instrument(skip_all)]
async fn list(
    State(svc): State<server::server::ServerSvc>,
    Query(p): Query<Pagination>,
) -> Result<ApiResponse<Vec<ServerResp>>, ApiError> {
    let list = svc.list(p.skip, p.limit).await?;
    let ret = list.into_iter().map(|e| e.into()).collect();
    Ok(ApiResponse::from_result(ret))
}
#[instrument(skip_all)]
async fn del(
    State(svc): State<server::server::ServerSvc>,
    Path(id): Path<String>,
) -> Result<ApiResponse<ServerResp>, ApiError> {
    let e = svc.delete(id).await?;
    Ok(ApiResponse::from_result(e.into()))
}

pub fn register_route(tx: mpsc::Sender<SyncData>) -> Router {
    let mut _router = Router::new();
    let svc: server::server::ServerSvc = server::server::ServerSvc::new(tx);
    _router = _router.route("/servers/:id", get(retrieve).put(update).delete(del));
    _router = _router.route("/servers", get(list).post(create));

    Router::new().nest("/api/config", _router).with_state(svc)
}

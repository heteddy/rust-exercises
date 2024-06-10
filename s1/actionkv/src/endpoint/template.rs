use crate::cache::sync::{self, SyncData};
use crate::dao::template::{TemplateDao, TemplateEntity};
use crate::pb::svr::{
    template::TemplateReq, template::TemplateResp, ApiError, ApiResponse, Pagination,
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
    State(svc): State<server::template::TemplateSvc>,
    Json(payload): Json<TemplateReq>,
) -> Result<ApiResponse<TemplateResp>, ApiError> {
    let e = svc.create(TemplateEntity::from(payload)).await?;
    Ok(ApiResponse::from_result(e.into()))
}
async fn retrieve(
    State(svc): State<server::template::TemplateSvc>,
    Path(id): Path<String>,
) -> Result<ApiResponse<TemplateResp>, ApiError> {
    let e = svc.get(id).await?;
    Ok(ApiResponse::from_result(e.into()))
}
async fn update(
    State(svc): State<server::template::TemplateSvc>,
    Path(id): Path<String>,
    Json(payload): Json<TemplateReq>,
) -> Result<ApiResponse<TemplateResp>, ApiError> {
    let e = svc.update(id, TemplateEntity::from(payload)).await?;
    Ok(ApiResponse::from_result(e.into()))
}
async fn list(
    State(svc): State<server::template::TemplateSvc>,
    Query(p): Query<Pagination>,
) -> Result<ApiResponse<Vec<TemplateResp>>, ApiError> {
    let list = svc.list(p.skip, p.limit).await?;
    let ret = list.into_iter().map(|e| e.into()).collect();
    Ok(ApiResponse::from_result(ret))
}
async fn del(
    State(svc): State<server::template::TemplateSvc>,
    Path(id): Path<String>,
) -> Result<ApiResponse<TemplateResp>, ApiError> {
    let e = svc.delete(id).await?;
    Ok(ApiResponse::from_result(e.into()))
}

pub fn register_route(tx: mpsc::Sender<SyncData>) -> Router {
    let mut _router = Router::new();
    let svc: server::template::TemplateSvc = server::template::TemplateSvc::new(tx);
    _router = _router.route("/templates/:id", get(retrieve).put(update).delete(del));
    _router = _router.route("/templates", get(list).post(create));

    Router::new().nest("/api", _router).with_state(svc)
}

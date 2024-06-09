use crate::cache::repo;
use crate::cache::sync;
use crate::dao::bert::BertEntity;
use crate::middleware::auth::auth_middleware;
use crate::pb::svr::{bert::BertReq, bert::BertResp, ApiError, ApiResponse, Pagination};
use crate::server;
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
    State(svc): State<server::bert::BertSvc>,
    Json(payload): Json<BertReq>,
) -> Result<ApiResponse<BertResp>, ApiError> {
    let s = span!(Level::INFO, "create_bert");
    let _enter = s.enter();
    event!(Level::INFO, "endpoint create bert {:?}", payload);
    let b = BertEntity::from(payload);
    let u = svc.create(b).await?;
    // Ok(Json(u))
    Ok(ApiResponse::from_result(u.into()))
}
#[instrument(skip_all)]
async fn retrieve(
    State(svc): State<server::bert::BertSvc>,
    Path(id): Path<String>,
) -> Result<ApiResponse<BertResp>, ApiError> {
    let s = span!(Level::INFO, "get");
    let _enter = s.enter();
    event!(Level::INFO, "endpoint get bert {:?}", id);
    let e = svc.get(id).await?;
    Ok(ApiResponse::from_result(e.into()))
}

#[instrument(skip_all)]
async fn update(
    State(svc): State<server::bert::BertSvc>,
    Path(id): Path<String>,
    Json(payload): Json<BertReq>,
) -> Result<ApiResponse<BertResp>, ApiError> {
    let s = span!(Level::INFO, "get");
    let _enter = s.enter();
    event!(Level::INFO, "endpoint update bert {:?}", id);
    let b = BertEntity::from(payload);
    let e = svc.update(id, b).await?;
    Ok(ApiResponse::from_result(e.into()))
}
#[instrument(skip_all)]
async fn list(
    State(svc): State<server::bert::BertSvc>,
    Query(pag): Query<Pagination>,
) -> Result<ApiResponse<Vec<BertResp>>, ApiError> {
    let s = span!(Level::INFO, "list");
    let _enter = s.enter();

    // let b = BertEntity::from(payload);
    let entities = svc.list(pag.skip, pag.limit).await?;
    let resp = entities.into_iter().map(|e| e.into()).collect();
    Ok(ApiResponse::from_result(resp))
}

#[instrument(skip_all)]
async fn del(
    State(svc): State<server::bert::BertSvc>,
    Path(id): Path<String>,
) -> Result<ApiResponse<BertResp>, ApiError> {
    let s = span!(Level::INFO, "get");
    let _enter = s.enter();
    event!(Level::INFO, "endpoint update bert {:?}", id);
    let e = svc.delete(id).await?;
    Ok(ApiResponse::from_result(e.into()))
}

pub fn register_route(tx: mpsc::Sender<sync::SyncData>) -> Router {
    let svc = server::bert::BertSvc::new(tx);
    let mut _route = Router::new();

    // let middle_svc = repo::IndexConfigRepo::get_instance();

    // todo 这里不要删除！！！新构建一个route然后使用route_layer 添加middleware
    // _route = _route.route(
    //     "/berts/:name",
    //     // handler middleware的方法
    //     post(create.layer(from_fn_with_state(middle_svc, auth_middleware))),
    // );
    // https://docs.rs/axum/latest/axum/middleware/index.html#passing-state-from-middleware-to-handlers
    //.route_layer(from_fn_with_state(middle_svc, auth_middleware));
    _route = _route.route("/berts", get(list).post(create));
    _route = _route.route("/berts/:id", get(retrieve).put(update).delete(del));
    Router::new().nest("/api", _route).with_state(svc)
}

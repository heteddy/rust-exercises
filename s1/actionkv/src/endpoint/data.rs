/// 实现数据增删改查
use crate::cache::repo;
use crate::cache::sync;
use crate::dao::bert::BertEntity;
use crate::pb::search::data::{InboundDataReq, SearchReq, SearchRet};
use crate::pb::svr::{ApiError, ApiResponse, Pagination};
use crate::server;
use crate::transport::middleware::auth::auth_middleware;
use axum::extract::{Json, Path, Query, State};
use axum::handler::Handler;
use axum::http::header::HeaderMap;
use axum::middleware::from_fn_with_state;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::convert::From;
use tokio::sync::mpsc;
use tracing::{event, info, instrument, span, Level};

#[instrument(skip_all)]
async fn upsert(
    headers: HeaderMap,
    Path(name): Path<String>,
    Json(body): Json<InboundDataReq>,
) -> Result<ApiResponse<String>, ApiError> {
    let id = body.id.clone();
    headers
        .iter()
        .for_each(|(k, v)| info!("received header k= {:?} v={:?}", k, v));

    info!("received name:{:?}", name);

    let body_str = serde_json::to_string(&body).unwrap_or_default();

    info!("received body_str = {:?}", body_str);
    info!("received body:{:?}", body);
    Ok(ApiResponse::from_result(id))
}

#[instrument(skip_all)]
async fn list(
    headers: HeaderMap,
    Query(id): Query<String>,
    Path(name): Path<String>,
    Json(body): Json<InboundDataReq>,
) -> Result<(), ApiError> {
    headers
        .iter()
        .for_each(|(k, v)| info!("received header k= {:?} v={:?}", k, v));

    info!("received name:{:?}", name);

    let body_str = serde_json::to_string(&body).unwrap_or_default();
    
    info!("received body_str = {:?}", body_str);
    info!("received body:{:?}", body);
    Ok(())
}

pub fn register_route() -> Router {
    let mut _router = Router::new();
    // let svc: server::index::IndexSvc = server::index::IndexSvc::new(tx);
    _router = _router.route("/index-data/:name", post(upsert).put(upsert));
    // _router = _router.route("/data", get(list).post(create));
    Router::new().nest("/api", _router)
}

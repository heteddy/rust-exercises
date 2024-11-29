/// 实现数据增删改查
use crate::cache::repo;
use crate::cache::sync;
use crate::dao::bert::BertEntity;
use crate::engine::search::SearchSvc;
use crate::pb::engine::qdrant::points as pb_points;
use crate::pb::engine::search::{InboundDataReq, SearchReq, SearchRet};
use crate::pb::svr::{ApiError, ApiResponse, InternalError, Pagination};
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
use serde_json::{self, json, Map, Number, Value};
use std::convert::From;
use tokio::sync::mpsc;
use tracing::{event, info, instrument, span, Level};

pub async fn search_points(
    State(svc): State<SearchSvc<'_>>,
    headers: HeaderMap,
    Path(name): Path<String>,
    Json(mut req): Json<SearchReq>,
) -> Result<pb_points::SearchResponse, ApiError> {
    info!(
        "request {}, search from index name:{}",
        req.request_id, name
    );

    match req.limit {
        0 => req.limit = 1,
        200.. => req.limit = 200,
        _ => {}
    }
    req.params
        .insert("limit".to_owned(), Value::Number(Number::from(req.limit)));

    match svc.search(name, req.template, req.params).await {
        anyhow::Result::Ok(resp) => Ok(resp),
        anyhow::Result::Err(e) => Err(InternalError::from(e.to_string()).into()),
    }
}

pub fn register_route() -> Router {
    let mut _router = Router::new();
    let svc = SearchSvc::new();

    let middle_svc = repo::IndexConfigRepo::get_instance();
    _router = _router.route(
        "/:name",
        post(search_points).layer(from_fn_with_state(middle_svc.clone(), auth_middleware)),
    );
    // _router = _router.route("/data", get(list).post(create));
    Router::new()
        .nest("/api/engine/search/index", _router)
        .with_state(svc)
}

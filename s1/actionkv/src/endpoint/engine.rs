/// 创建qdrant collection;
/// 配置修改collection
use crate::cache::repo;
use crate::cache::sync;
use crate::engine::search::collection::CollectionSvc;
use crate::pb::engine::qdrant::collection::CollectionOperationResponse;
use crate::pb::engine::search::CollectionReq;
use crate::pb::svr::{ApiError,InternalError, ApiResponse, Pagination};
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

// 创建qdrant 索引
pub async fn create_collection(
    State(svc): State<CollectionSvc>,
    Json(payload): Json<CollectionReq>,
) -> Result<ApiResponse<CollectionOperationResponse>, ApiError> {
    // 创建collection; 设置
    match svc.create(payload).await {
        Ok(resp) => Ok(ApiResponse::from_result(resp)),
        Err(e) => Err(InternalError::from(e.to_string()).into()),
    }
}
// 获取单个collection
async fn retrieve() {}

// 修改alias
async fn alias() {}
// 获取collections
async fn list_collections() {}

pub fn register_route() -> Router {
    let mut _router = Router::new();
    let svc = CollectionSvc::new();
    // _router = _router.route("/collections/:id", get(retrieve).put(update).delete(del));
    _router = _router.route("/collections", post(create_collection));
    Router::new().nest("/api/engine", _router).with_state(svc)
}

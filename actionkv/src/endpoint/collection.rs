/// 创建qdrant collection;
/// 配置修改collection
use crate::cache::repo;
use crate::cache::sync;
use crate::engine::collection::CollectionSvc;
use crate::pb::engine::qdrant::collection::{
    ChangeAliases, CollectionOperationResponse, GetCollectionInfoResponse, ListAliasesResponse,
    ListCollectionsResponse,
};
use crate::pb::engine::search::CollectionReq;
use crate::pb::svr::{ApiError, ApiResponse, InternalError, Pagination};
use crate::server;
use crate::transport::middleware::auth::auth_middleware;
// use anyhow::Ok;
use axum::extract::{Json, Path, Query, State};
use axum::handler::Handler;
use axum::middleware::from_fn_with_state;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::convert::From;
use tokio::sync::mpsc;
use tracing::{event, info, instrument, span, Level};

// 创建qdrant 索引
#[instrument(skip_all)]
pub async fn create_collection(
    State(svc): State<CollectionSvc>,
    Json(payload): Json<CollectionReq>,
) -> Result<CollectionOperationResponse, ApiError> {
    // 创建collection; 设置
    match svc.create(payload).await {
        anyhow::Result::Ok(resp) => Ok(resp),
        anyhow::Result::Err(e) => Err(InternalError::from(e.to_string()).into()),
    }
}
// 获取单个collection
#[instrument(skip_all)]
async fn retrieve_collection(
    State(svc): State<CollectionSvc>,
    Path(name): Path<String>,
) -> Result<GetCollectionInfoResponse, ApiError> {
    match svc.get(&name).await {
        anyhow::Result::Ok(resp) => Ok(resp),
        anyhow::Result::Err(e) => Err(InternalError::from(e.to_string()).into()),
    }
}
#[instrument(skip_all)]
async fn delete_collection(
    State(svc): State<CollectionSvc>,
    Path(name): Path<String>,
) -> Result<CollectionOperationResponse, ApiError> {
    match svc.delete(&name).await {
        anyhow::Result::Ok(resp) => Ok(resp),
        anyhow::Result::Err(e) => Err(InternalError::from(e.to_string()).into()),
    }
}

// 修改alias
#[instrument(skip_all)]
async fn change_alias(
    State(svc): State<CollectionSvc>,
    Path(name): Path<String>,
    Json(payload): Json<ChangeAliases>,
) -> Result<CollectionOperationResponse, ApiError> {
    match svc.change_alias(name, payload).await {
        anyhow::Result::Ok(resp) => Ok(resp),
        anyhow::Result::Err(e) => Err(InternalError::from(e.to_string()).into()),
    }
}

#[instrument(skip_all)]
async fn get_alias(
    State(svc): State<CollectionSvc>,
    Path(name): Path<String>,
) -> Result<ListAliasesResponse, ApiError> {
    match svc.get_alias(name).await {
        anyhow::Result::Ok(resp) => Ok(resp),
        anyhow::Result::Err(e) => Err(InternalError::from(e.to_string()).into()),
    }
}
#[instrument(skip_all)]
async fn list_all_aliases(
    State(svc): State<CollectionSvc>,
    Path(sever): Path<String>,
) -> Result<ListAliasesResponse, ApiError> {
    info!("receiving list aliases, severname={}", &sever);

    // Err(InternalError::from("hi".to_string()))
    // Ok(ListAliasesResponse::default())

    match svc.list_aliases(sever).await {
        anyhow::Result::Ok(resp) => Ok(resp),
        anyhow::Result::Err(e) => Err(InternalError::from(e.to_string()).into()),
    }
}

// 获取collections
async fn list_collections(
    State(svc): State<CollectionSvc>,
    Path(svr): Path<String>,
) -> Result<ListCollectionsResponse, ApiError> {
    match svc.list(svr).await {
        anyhow::Result::Ok(resp) => Ok(resp),
        anyhow::Result::Err(e) => Err(InternalError::from(e.to_string()).into()),
    }
}

pub fn register_route(tx: mpsc::Sender<sync::SyncData>) -> Router {
    let mut _router = Router::new();
    let svc = CollectionSvc::new(tx);
    // _router = _router.route("/collections/:id", get(retrieve).put(update).delete(del));
    _router = _router.route("/alias/:name", post(change_alias).get(get_alias)); // collection name
    _router = _router.route("/list_aliases/:severname", get(list_all_aliases));
    _router = _router.route("/server/:svr/collections", get(list_collections));
    _router = _router.route("/collections", post(create_collection));
    _router = _router.route(
        "/collections/:name",
        get(retrieve_collection).delete(delete_collection),
    );
    Router::new().nest("/api/engine", _router).with_state(svc)
}

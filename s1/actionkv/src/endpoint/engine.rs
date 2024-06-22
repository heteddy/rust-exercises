/// 创建qdrant collection;
/// 配置修改collection
use crate::cache::repo;
use crate::cache::sync;
use crate::dao::bert::BertEntity;
use crate::pb::svr::{bert::BertReq, bert::BertResp, ApiError, ApiResponse, Pagination};
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
fn create_engine() {}
// 修改alias
fn alias() {}
// 获取collections
fn list_collections() {}
// 获取单个collection
fn get_collection() {}


pub fn register_route() -> Router {
    let mut _router = Router::new();
    // let svc: server::index::IndexSvc = server::index::IndexSvc::new(tx);
    // _router = _router.route("/indices/:id", get(retrieve).put(update).delete(del));
    // _router = _router.route("/indices", get(list).post(create));
    Router::new().nest("/api", _router).with_state(svc)
}
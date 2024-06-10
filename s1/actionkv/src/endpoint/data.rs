/// 实现数据增删改查
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

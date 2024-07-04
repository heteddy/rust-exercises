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
use tracing::{event, instrument, span, Level};
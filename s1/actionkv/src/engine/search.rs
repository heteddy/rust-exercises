use crate::cache::repo;
use crate::cache::sync;
use crate::dao::base::EntityDao;
use crate::dao::index::{IndexDao, IndexEntity};
use crate::driver::qdrant::index;
use crate::driver::qdrant::points as driver_points;
use crate::pb::engine::qdrant::collection;
use crate::pb::engine::qdrant::collection::{
    ChangeAliases, CollectionOperationResponse, CreateCollection, GetCollectionInfoResponse,
    ListAliasesResponse, ListCollectionsResponse,
};
use crate::pb::engine::search;
use crate::pb::svr::ApiError;
use crate::server::index::IndexSvc;
use chrono::prelude::*;
use std::convert::AsRef;
use std::fmt::Debug;
use tokio::sync::mpsc;
use tracing::{event, info, instrument, span, Level};
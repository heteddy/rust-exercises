/// 实现数据增删改查
use crate::cache::repo;
use crate::cache::sync;
use crate::dao::bert::BertEntity;
use crate::engine::point::PointSvc;
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
use std::convert::From;
use tokio::sync::mpsc;
use tracing::{event, info, instrument, span, Level};

#[instrument(skip_all)]
async fn upsert_point(
    // headers: HeaderMap,
    State(svc): State<PointSvc>,
    Path(name): Path<String>,
    Json(body): Json<InboundDataReq>,
) -> Result<pb_points::PointsOperationResponse, ApiError> {
    // let id = body.id.clone();
    // headers
    //     .iter()
    //     .for_each(|(k, v)| info!("received header k= {:?} v={:?}", k, v));
    info!("upsert points into index name:{:?}", name);
    match svc.upsert_points(name, body.into()).await {
        anyhow::Result::Ok(resp) => Ok(resp),
        anyhow::Result::Err(e) => Err(InternalError::from(e.to_string()).into()),
    }
}

#[instrument(skip_all)]
async fn retrieve(
    State(svc): State<PointSvc>,
    headers: HeaderMap,
    Path(name): Path<String>,
    Json(body): Json<pb_points::GetPoints>,
) -> Result<pb_points::GetResponse, ApiError> {
    info!("retrieve points from index name:{:?}", name);
    match svc.get_points(name, body).await {
        anyhow::Result::Ok(resp) => Ok(resp),
        anyhow::Result::Err(e) => Err(InternalError::from(e.to_string()).into()),
    }
}

#[instrument(skip_all)]
async fn delete_point(
    State(svc): State<PointSvc>, // 必须放在第一个
    headers: HeaderMap,
    Path(name): Path<String>,
    Json(body): Json<pb_points::DeletePoints>,
) -> Result<pb_points::PointsOperationResponse, ApiError> {
    info!("retrieve points from index name:{:?}", name);
    match svc.delete_points(name, body).await {
        anyhow::Result::Ok(resp) => Ok(resp),
        anyhow::Result::Err(e) => Err(InternalError::from(e.to_string()).into()),
    }
}

pub fn register_route() -> Router {
    let mut _router = Router::new();
    let svc = PointSvc::new();
    /*
    // 获取全局的state
    let middle_svc = repo::IndexConfigRepo::get_instance();
    _route = _route.route(
        "/berts/:name",
        // handler middleware的方法
        post(create.layer(from_fn_with_state(middle_svc, auth_middleware))),
    );
    .route_layer(from_fn_with_state(middle_svc, auth_middleware));
    */
    let middle_svc = repo::IndexConfigRepo::get_instance();
    _router = _router.route(
        "/:name/points/upsert",
        post(upsert_point).layer(from_fn_with_state(middle_svc.clone(), auth_middleware)),
    );
    _router = _router.route(
        "/:name/points/get",
        post(retrieve).layer(from_fn_with_state(middle_svc.clone(), auth_middleware)),
    );
    _router = _router.route(
        "/:name/points/delete",
        delete(delete_point).layer(from_fn_with_state(middle_svc, auth_middleware)),
    );
    // _router = _router.route("/data", get(list).post(create));
    Router::new().nest("/api/data", _router).with_state(svc)
}

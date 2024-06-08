use crate::cache::sync;

use crate::dao::bert::BertEntity;
use crate::middleware::auth::auth_middleware;
use crate::pb::svr::{bert::BertReq, bert::BertResp, ApiError, ApiResponse};
use crate::server;
use axum::extract::{Json, State};
use axum::handler::Handler;

use crate::cache::repo;
use axum::{middleware::from_fn_with_state, routing::post, Router};

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

pub fn register_route(tx: mpsc::Sender<sync::SyncData>) -> Router {
    let svc = server::bert::BertSvc::new(tx);
    let mut _route = Router::new();
    // let middle_svc = server::auth::TENANT_AUTH_SVC.clone();
    let middle_svc = repo::IndexConfigRepo::get_instance();

    // todo 新构建一个route然后使用route_layer 添加middleware
    _route = _route.route(
        "/berts/:name",
        post(create.layer(from_fn_with_state(middle_svc, auth_middleware))),
    );
    // https://docs.rs/axum/latest/axum/middleware/index.html#passing-state-from-middleware-to-handlers
    //.route_layer(from_fn_with_state(middle_svc, auth_middleware));
    // _route = _route.route("/bert", get(get_app).put(update_app));
    Router::new().nest("/api", _route).with_state(svc)
}

use crate::pb;
use crate::service;
use std::convert::From;
use axum::extract::{Json, Path, Query, State};
use axum::http::StatusCode;
use axum::Router;
use axum::routing::{delete, get, post, put};
use tracing::{event, info, instrument, span, Level};
use crate::dao;
use crate::dao::app::AppEntity;

#[instrument(skip_all)]
async fn create_app(
    State(repo): State<service::app::AppService>,
    Json(payload): Json<pb::app::AppReq>,
) -> Json<AppEntity> {
    let s = span!(Level::INFO, "create_app");
    let _enter = s.enter();
    event!(Level::INFO, "endpoint create app {:?}", payload);
    let app = AppEntity::from(payload);
    let u = repo.create_app_service(app).await;
    Json(u)
}


pub fn register_app_route() -> Router {
    let s = service::app::AppService::new();

    let mut app_route = Router::new();
    app_route = app_route.route("/apps",post(create_app));
    Router::new().nest("/api",app_route).with_state(s)
}
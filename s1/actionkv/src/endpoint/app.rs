use crate::dao;
use crate::dao::app::AppEntity;
use crate::pb;
use crate::service;
use axum::extract::{Json, Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{
    routing::MethodFilter,
    routing::{delete, get, on, post, put},
    Router,
};
use serde_derive::Deserialize;
use std::convert::From;
use tracing::{event, info, instrument, span, Level};

#[instrument(skip_all)]
async fn create_app(
    State(svc): State<service::app::AppService>,
    Json(payload): Json<pb::app::AppReq>,
) -> Result<Json<AppEntity>, pb::error::ApiError> {
    let s = span!(Level::INFO, "create_app");
    let _enter = s.enter();
    event!(Level::INFO, "endpoint create app {:?}", payload);
    let app = AppEntity::from(payload);
    let u = svc.create_app_service(app).await?;
    Ok(Json(u))
}

#[derive(Debug, Deserialize)]
struct Pagination {
    skip: u64,
    limit: i64,
}

#[instrument(skip_all)]
pub async fn list_apps(
    State(svc): State<service::app::AppService>,
    page: Query<Pagination>,
) -> Json<Vec<AppEntity>> {
    event!(Level::INFO, "endpoint list all apps {:?}", page);
    let results = svc.list_all(page.skip, page.limit).await;
    let ret = results.unwrap();
    Json(ret)
    // Json<Vec<Result< crate::dao::app::AppEntity, mongodb::error::Error>>>
}

#[instrument(skip_all)]
pub async fn get_app(
    State(svc): State<service::app::AppService>,
    Path(id): Path<String>,
) -> Result<Json<AppEntity>,pb::error::ApiError> {
    event!(Level::INFO, "endpoint get path apps {:?}", id);
    let entity = svc.get_app_by_id(&id).await?;
    Ok(Json(entity))
}

pub fn register_app_route() -> Router {
    let svc = service::app::AppService::new();
    let mut app_route = Router::new();
    app_route = app_route.route("/apps", post(create_app).get(list_apps));
    app_route = app_route.route("/apps/:id", get(get_app));
    Router::new().nest("/api", app_route).with_state(svc)
}

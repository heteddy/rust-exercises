use crate::dao;
use crate::dao::app::AppEntity;
use crate::pb::{app::AppReq, app::AppResp, error::ApiError, ApiResponse};
use crate::service;
use axum::extract::{Json, Path, Query, State};
use axum::http::{header::HeaderMap, StatusCode};
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
    Json(payload): Json<AppReq>,
) -> Result<ApiResponse<AppResp>, ApiError> {
    let s = span!(Level::INFO, "create_app");
    let _enter = s.enter();
    event!(Level::INFO, "endpoint create app {:?}", payload);
    let app = AppEntity::from(payload);
    let u = svc.create_app(app).await?;
    // Ok(Json(u))
    Ok(ApiResponse::from_result(u.into()))
}

#[derive(Debug, Deserialize)]
struct Pagination {
    skip: u64,
    limit: i64,
}

#[instrument(skip_all)]
pub async fn list_apps(
    State(svc): State<service::app::AppService>,
    headers: HeaderMap,
    page: Query<Pagination>,
) -> Result<ApiResponse<Vec<AppResp>>, ApiError> {
    event!(Level::INFO, "endpoint list all apps {:?}", page);
    headers.iter().for_each(|(name, value)| {
        event!(
            Level::INFO,
            "received headers key={:?} value={:?}",
            name,
            value
        );
    });
    let results = svc.list_all(page.skip, page.limit).await?;
    let mut resp = Vec::with_capacity(results.len());
    results.into_iter().for_each(|e| resp.push(e.into()));
    Ok(ApiResponse::from_result(resp))

    // match results {
    //     Ok(entity) => Ok(pb::ApiResponse::from_result(&entity)),
    //     Err(e) => Err((
    //         StatusCode::INTERNAL_SERVER_ERROR,
    //         pb::ApiResponse::from_error(e),
    //     )),
    // }
    // Json<Vec<Result< crate::dao::app::AppEntity, mongodb::error::Error>>>
}

#[instrument(skip_all)]
pub async fn get_app(
    State(svc): State<service::app::AppService>,
    Path(id): Path<String>,
) -> Result<ApiResponse<AppResp>, ApiError> {
    event!(Level::INFO, "endpoint get path apps {:?}", id);
    let result = svc.get_app_by_id(&id).await?;
    Ok(ApiResponse::from_result(result.into()))
}

#[instrument(skip_all)]
pub async fn update_app(
    State(svc): State<service::app::AppService>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(payload): Json<AppReq>,
) -> Result<ApiResponse<AppResp>, ApiError> {
    headers.iter().for_each(|(name, value)| {
        event!(
            Level::INFO,
            "received headers key={:?} value={:?}",
            name,
            value
        );
    });
    event!(Level::INFO, "endpoint update path apps {:?}", id);
    // 这里实现了from，因此类型不匹配可以直接传
    let result = svc.update_app(&id, payload.into()).await?;
    Ok(ApiResponse::from_result(result.into()))
}

pub fn register_app_route() -> Router {
    let svc = service::app::AppService::new();
    let mut app_route = Router::new();
    app_route = app_route.route("/apps", post(create_app).get(list_apps));
    app_route = app_route.route("/apps/:id", get(get_app).put(update_app));
    Router::new().nest("/api", app_route).with_state(svc)
}

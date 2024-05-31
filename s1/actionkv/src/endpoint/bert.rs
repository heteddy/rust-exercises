use crate::cache::sync;
use crate::dao;
use crate::dao::bert::BertEntity;
use crate::middleware::auth::auth_middleware;
use crate::pb::svr::{bert::BertReq, bert::BertResp, ApiError, ApiResponse};
use crate::server;
use axum::extract::{Json, Path, Query, State};
use axum::handler::Handler;
use axum::http::{header::HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::{
    middleware::from_fn_with_state,
    routing::MethodFilter,
    routing::{delete, get, on, post, put},
    Router,
};
use serde_derive::Deserialize;
use std::convert::From;
use tokio::sync::mpsc;
use tracing::{event, info, instrument, span, Level};

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

// #[derive(Debug, Deserialize)]
// struct Pagination {
//     skip: u64,
//     limit: i64,
// }

// #[instrument(skip_all)]
// pub async fn list_apps(
//     State(svc): State<server::app::AppService>,
//     headers: HeaderMap,
//     page: Query<Pagination>,
// ) -> Result<ApiResponse<Vec<AppResp>>, ApiError> {
//     event!(Level::INFO, "endpoint list all apps {:?}", page);
//     headers.iter().for_each(|(name, value)| {
//         event!(
//             Level::INFO,
//             "received headers key={:?} value={:?}",
//             name,
//             value
//         );
//     });
//     let results = svc.list_all(page.skip, page.limit).await?;
//     let mut resp = Vec::with_capacity(results.len());
//     results.into_iter().for_each(|e| resp.push(e.into()));
//     Ok(ApiResponse::from_result(resp))

//     // match results {
//     //     Ok(entity) => Ok(pb::ApiResponse::from_result(&entity)),
//     //     Err(e) => Err((
//     //         StatusCode::INTERNAL_SERVER_ERROR,
//     //         pb::ApiResponse::from_error(e),
//     //     )),
//     // }
//     // Json<Vec<Result< crate::dao::app::AppEntity, mongodb::error::Error>>>
// }

// #[instrument(skip_all)]
// pub async fn get_app(
//     State(svc): State<server::app::AppService>,
//     Path(id): Path<String>,
// ) -> Result<ApiResponse<AppResp>, ApiError> {
//     event!(Level::INFO, "endpoint get path apps {:?}", id);
//     let result = svc.get_app_by_id(&id).await?;
//     Ok(ApiResponse::from_result(result.into()))
// }

// #[instrument(skip_all)]
// pub async fn update_app(
//     State(svc): State<server::app::AppService>,
//     headers: HeaderMap,
//     Path(id): Path<String>,
//     Json(payload): Json<AppReq>,
// ) -> Result<ApiResponse<AppResp>, ApiError> {
//     headers.iter().for_each(|(name, value)| {
//         event!(
//             Level::INFO,
//             "received headers key={:?} value={:?}",
//             name,
//             value
//         );
//     });
//     event!(Level::INFO, "endpoint update path apps {:?}", id);
//     // 这里实现了from，因此类型不匹配可以直接传
//     let result = svc.update_app(&id, payload.into()).await?;
//     Ok(ApiResponse::from_result(result.into()))
// }

pub fn register_route(tx: mpsc::Sender<sync::SyncData>) -> Router {
    let svc = server::bert::BertSvc::new(tx);
    let mut _route = Router::new();
    let middle_svc = server::auth::TENANT_AUTH_SVC.clone();

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

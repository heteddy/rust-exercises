use crate::endpoints::{hello, user};
use axum::http::HeaderName;
use axum::{Router, BoxError, http::StatusCode};
use std::time::Duration;
use tokio::time::sleep;
use tower::{ServiceBuilder, self};
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use axum::error_handling::{HandleError, HandleErrorLayer};

#[derive(Clone)]
struct State {}

pub fn init_app() -> axum::Router {
    // 会move
    let mut app = axum::Router::new();

    app = app
        .merge(hello::register_hello())
        .merge(user::register_user())
        .fallback(fallback);

    app = app.layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(CompressionLayer::new().gzip(true))
            .layer(TimeoutLayer::new(Duration::new(0, 200000)))
            .layer(SetRequestIdLayer::new(
                HeaderName::from_static("x-request-id"),
                MakeRequestUuid,
            ))
            .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any))
            // .layer(HandleErrorLayer::new(handle_timeout_error))
        ,
    );
    app
}

async fn fallback() -> String {
    format!("错误路由")
}

async fn handle_timeout_error(err: BoxError) -> (StatusCode, String) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (StatusCode::REQUEST_TIMEOUT,format!( "Request time too long， Timeout！！！"))
    }else{
        (StatusCode::INTERNAL_SERVER_ERROR,format!("Unhandled internal error: {}", err))
    }
}

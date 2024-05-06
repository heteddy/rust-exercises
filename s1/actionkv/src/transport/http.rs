use axum::error_handling::HandleErrorLayer;
use axum::{
    http::{HeaderName, Method, StatusCode, Uri},
    response::Html,
    BoxError, Router,
    routing::get,
};
use std::time::Duration;
// use tokio::time::sleep;
use tower::{self, ServiceBuilder};
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use crate::endpoint::app;

#[derive(Clone)]
struct State {}

pub fn init_app() -> Router {
    // 会move
    let mut app = Router::new();

    app = app
        .route("/", get(hello_world))
        .merge(app::register_app_route())
        // .merge(user::register_user())
        .fallback(fallback);

    app = app.layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http()
                       .make_span_with(trace::DefaultMakeSpan::new()
                           .level(Level::INFO))
                       .on_response(trace::DefaultOnResponse::new()
                           .level(Level::INFO)), )
            .layer(CompressionLayer::new().gzip(true))
            .layer(TimeoutLayer::new(Duration::new(0, 900_000_000))) //900ms
            .layer(SetRequestIdLayer::new(
                HeaderName::from_static("x-request-id"),
                MakeRequestUuid,
            ))
            .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any))
            .layer(HandleErrorLayer::new(handle_timeout_error))
            .timeout(Duration::from_secs(30)),
    );
    app
}

async fn fallback() -> String {
    format!("错误路由")
}

async fn handle_timeout_error(method: Method, uri: Uri, err: BoxError) -> (StatusCode, String) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            "Request time too long，Timeout！！！".to_owned(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", err),
        )
    }
}

async fn hello_world() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

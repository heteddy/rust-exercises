use crate::cache::sync;
use crate::endpoint::{app, bert, collection, index, points, preprocess, search, server, template};
// use axum::body::{box_body, BoxBody};
use axum::error_handling::HandleErrorLayer;
use axum::{
    body::Body,
    http::{HeaderMap, HeaderName, Method, Request, Response, StatusCode, Uri},
    response::IntoResponse,
    response::Json,
    routing::get,
    BoxError, Router,
};
use http::HeaderValue;
use std::time::Duration;
use tokio::sync::mpsc;
use tower::{self, ServiceBuilder};
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{self, TraceLayer};
use tracing::{debug, info, warn, Level, Span};
// use utoipa::{
//     openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
//     Modify, OpenApi,
// };
// use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
struct State {}

pub fn init_app(tx: mpsc::Sender<sync::SyncData>) -> Router {
    // 会move

    // #[derive(OpenApi)]
    // #[openapi(
    //     modifiers(&SecurityAddon),
    //     nest(
    //         (path = "/api/v1/todos", api = todo::TodoApi)
    //     ),
    //     tags(
    //         (name = "todo", description = "Todo items management API")
    //     )
    // )]
    // struct ApiDoc;

    // struct SecurityAddon;

    // impl Modify for SecurityAddon {
    //     fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
    //         if let Some(components) = openapi.components.as_mut() {
    //             components.add_security_scheme(
    //                 "api_key",
    //                 SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("actionkv_apikey"))),
    //             )
    //         }
    //     }
    // }

    let mut app = Router::new();
    app = app
        .route("/", get(hello_world))
        .merge(app::register_route(tx.clone()))
        .merge(bert::register_route(tx.clone()))
        .merge(server::register_route(tx.clone()))
        .merge(preprocess::register_route(tx.clone()))
        .merge(index::register_route(tx.clone()))
        .merge(template::register_route(tx.clone()))
        .merge(points::register_route())
        .merge(collection::register_route(tx))
        .merge(search::register_route())
        // .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // .route_layer(layer)   // 仅命中路由才打印
        .fallback(fallback);
    info!("merged routers");
    // 先添加的路由会被后面的middleware处理，后添加的不处理
    app = app.layer(
        // 这里是按照从上到下的顺序执行
        // todo 注意顺序
        ServiceBuilder::new()
            .layer(
                TraceLayer::new_for_http()
                    .on_request(|request: &Request<Body>, _span: &Span| {})
                    .on_failure(
                        |error: ServerErrorsFailureClass, latency: Duration, _span: &Span| {
                            tracing::error!("error request, {}, latency:{:?}", error, latency)
                        },
                    ), // .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                       // .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
            )
            // .layer(SetResponseHeaderLayer::<HeaderValue>::appending(
            //     HeaderName::from_static("content-type"),
            //     HeaderValue::from_static("charset=UTF-8"),
            // ))
            // .layer(CompressionLayer::new().gzip(true))
            .layer(TimeoutLayer::new(Duration::new(0, 900_000_000))) //900ms
            .layer(SetRequestIdLayer::new(
                HeaderName::from_static("x-request-id"),
                MakeRequestUuid,
            ))
            .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any))
            .layer(HandleErrorLayer::new(handle_timeout_error))
            .timeout(Duration::from_secs(30)),
    );

    info!("app created");
    app
}

async fn fallback() -> String {
    format!("错误路由")
}

async fn handle_timeout_error(method: Method, uri: Uri, err: BoxError) -> impl IntoResponse {
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

async fn hello_world() -> Json<&'static str> {
    // Html("<h1>Hello, World!</h1>")
    Json("ok")
}

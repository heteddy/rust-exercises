use crate::cache::repo;
use axum::{
    extract::{Path, Request, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::{Arc, RwLock};
use tracing::{info, warn};
/// 通过router
/**
MethodRouter::layer：除了只能绑定在一个特定的method_router，其他与 Router::layer 效果一致

MethodRouter::route_layer：除了只能绑定在一个特定的method_router，其他与 Router::router_layer 效果一致，特别的，该方式在method_router的fallback中不触发
.route("/aaa", get(aaa).layer(trace_layer.clone()))
         .route("/bbb", get(bbb).fallback(handler_fallback).layer(trace_layer.clone())
         )
         .route("/ccc", get(ccc).fallback(handler_fallback).route_layer(trace_layer));
Handler::layer：handler粒度的Layer，也是最小粒度的Layer
```
use axum::{Extension, Router};
use axum::handler::{HandlerService, Layered};
use axum::handler::Handler;
use axum::routing::{get, post};
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;
​
async fn aaa() {}
​
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
​
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));
​
    let layer_handler = aaa.layer(trace_layer);
​
    let app = Router::new()
        .route("/aaa", get(layer_handler));
​
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

# axum::middleware::from_extractor
适用场景：
https://docs.rs/axum/latest/axum/middleware/fn.from_extractor.html
兼容 Extractor，即：该类型有时候作为 Extractor方式使用，有时候作为中间件使用
示例代码，假设接口需要验证请求头的签名：

use axum::{Extension, Router};
 use axum::extract::FromRequestParts;
 use axum::handler::Handler;
 use axum::routing::{get, post};
 use http::request::Parts;
 use http::{HeaderName, HeaderValue, StatusCode};
 use async_trait::async_trait;

 async fn aaa() -> &'static str {
     "hello"
 }

 struct RequireSign;

 #[async_trait]
 impl <S> FromRequestParts<S> for RequireSign
     where
         S: Send + Sync,
 {
     type Rejection = StatusCode;

     async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
         let app_id_header = parts.headers
             .get(HeaderName::from_static("x-app-id"))
             .and_then(|value| value.to_str().ok());
         let sign_header = parts.headers
             .get(HeaderName::from_static("x-sign"))
             .and_then(|value| value.to_str().ok());

         match (app_id_header, sign_header) {
             (Some(app_id), Some(sign)) if verify(app_id, sign) => {
                 Ok(Self)
             }
             _ => Err(StatusCode::UNAUTHORIZED)
         }
     }
 }

 fn verify(app_id: &str, sign: &str) -> bool {
     //replace with some real logic code
     true
 }

 #[tokio::main]
 async fn main() {
     let duration = std::time::Duration::from_secs(3);

     let app = Router::new()
         .route("/aaa",get(aaa))
         .layer(axum::middleware::from_extractor::<RequireSign>());

     // run it with hyper on localhost:3000
     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
         .serve(app.into_make_service())
         .await
         .unwrap();
 }
```
# axum::middleware::from_fn
https://docs.rs/axum/latest/axum/middleware/fn.from_fn.html
pub fn from_fn<F, T>(f: F) -> FromFnLayer<F, (), T>
Create a middleware from an async function.

from_fn requires the function given to

Be an async fn.
Take one or more extractors as the first arguments.
Take Next as the final argument.
Return something that implements IntoResponse.
Note that this function doesn’t support extracting State. For that, use from_fn_with_state.
```
use axum::{
    Router,
    http,
    routing::get,
    response::Response,
    middleware::{self, Next},
    extract::Request,
};

async fn my_middleware(
    request: Request,
    next: Next,
) -> Response {
    // do something with `request`...

    let response = next.run(request).await;

    // do something with `response`...

    response
}

let app = Router::new()
    .route("/", get(|| async { /* ... */ }))
    .layer(middleware::from_fn(my_middleware));
Running extractors
use axum::{
    Router,
    extract::Request,
    http::{StatusCode, HeaderMap},
    middleware::{self, Next},
    response::Response,
    routing::get,
};

async fn auth(
    // run the `HeaderMap` extractor
    headers: HeaderMap,
    // you can also add more extractors here but the last
    // extractor must implement `FromRequest` which
    // `Request` does
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    match get_token(&headers) {
        Some(token) if token_is_valid(token) => {
            let response = next.run(request).await;
            Ok(response)
        }
        _ => {
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

fn get_token(headers: &HeaderMap) -> Option<&str> {
    // ...
}

fn token_is_valid(token: &str) -> bool {
    // ...
}

let app = Router::new()
    .route("/", get(|| async { /* ... */ }))
    .route_layer(middleware::from_fn(auth));

 ```
*/

/// 基于 from_fn_with_state 实现app_id和app_secret的校验
///
///

pub async fn auth_middleware(
    State(svc): State<Arc<RwLock<repo::IndexConfigRepo>>>,
    Path(name): Path<String>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    let get_header_value = |v: &HeaderValue| v.to_str().unwrap_or("").to_owned();

    let app_id = headers
        .get("x-app-id")
        .map_or("".to_owned(), get_header_value);

    let app_secret = headers
        .get("x-app-secret")
        .map_or("".to_owned(), get_header_value);
    // 绑定name是参数
    info!(
        "auth_middleware app_id={:?},app_secret={:?} uri={:?}",
        app_id,
        app_secret,
        request.uri()
    );
    if name.len() > 0 && app_id.len() > 0 && app_secret.len() > 0 {
        // 读index config
        let s: std::sync::RwLockReadGuard<repo::IndexConfigRepo> = svc.read().unwrap();

        if !s.auth(&app_id, &app_secret, &name) {
            return Err((
                StatusCode::UNAUTHORIZED,
                "app_id or app_secret 错误".to_owned(),
            ));
        }
    } else {
        warn!("auth_middleware {:?},{:?}", app_id, app_secret);
        return Err((
            StatusCode::UNAUTHORIZED,
            "app_id or app_secret 错误".to_owned(),
        ));
    }
    let response = next.run(request).await;
    Ok(response)
}

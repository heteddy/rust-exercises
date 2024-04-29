#![cfg_attr(
debug_assertions,
allow(
// unused,
dead_code,
// unused_imports,
unused_variables,
unused_assignments,
non_snake_case
)
)]

// use ansi_term::Colour;
// use axum::{
//     // body::Bytes,
//     // extract::MatchedPath,
//     // http::{HeaderMap, Request},
//     // response::{Html, Response},
//     // routing::get,
//     // Router,
// };
// use clap::{Parser, Subcommand};

use libakv::{
    config,
    transport::http,
};

use tokio::net::TcpListener;
// use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
// use tracing::{event, info, info_span, instrument, span, span::Span, warn, Level};
use tracing::{info, warn, debug};

#[tokio::main]
async fn main() {
    // thread::sleep(Duration::from_secs(2));
    // 这里会阻塞
    config::global_configure().await;
    //
    warn!("start tracing subscriber");
    info!("start app");
    // build our application with a route
    let app = http::init_app();
    let listener = TcpListener::bind("127.0.0.1:8090").await.unwrap();
    debug!("listening on {}", listener.local_addr().unwrap());
    warn!("add tracing info");
    axum::serve(listener, app).await.unwrap();
}

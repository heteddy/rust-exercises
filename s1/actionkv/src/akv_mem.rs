#![cfg_attr(
    debug_assertions,
    allow(
        // unused,
        dead_code,
        // unused_imports,
        unreachable_patterns,
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

// use chrono::Utc;
use libakv::{cache, config, dao, transport::http};
use tokio::net::TcpListener;
use tracing::{info, warn};
// use utoipa::{
//     openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
//     Modify, OpenApi,
// };

#[tokio::main]
async fn main() {
    // thread::sleep(Duration::from_secs(2));
    // 这里会阻塞
    config::global_configure().await;
    let result = dao::init_indexes().await;
    //

    warn!("start tracing subscriber");
    info!("start app");
    // build our application with a route

    let tx = cache::start_cacher().await;
    info!("cache started......");

    let app = http::init_app(tx.clone());
    
    let listener = TcpListener::bind(format!("0.0.0.0:{}",config::cc::CLI_ARGS.port)).await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

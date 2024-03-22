#![cfg_attr(
    debug_assertions,
    allow(
        unused,
        dead_code,
        unused_imports,
        unused_variables,
        unused_assignments,
        non_snake_case
    )
)]
use axum::{routing::get, Router};
use tokio::net::TcpListener;
use std::net::SocketAddr;
use tracing_subscriber;
use tracing::{event, Level, info};


mod endpoints;
mod transport;
mod service;
mod pb;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().json().init();

    let app = transport::route::init_app();
    let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    println!("{:?}", addr);
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    // axum::Serve::bind(&listener).serve(app.into_make_service()).await.unwrap();
    axum::serve(listener,app).await.unwrap();
}





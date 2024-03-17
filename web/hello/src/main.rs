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

mod endpoints;
mod transport;
mod service;

#[tokio::main]
async fn main() {
    let app = transport::route::init_app();
    let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    println!("{:?}", addr);
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener,app).await.unwrap();
}





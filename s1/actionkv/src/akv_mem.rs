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
    let listener = TcpListener::bind("127.0.0.1:8090").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());

    // let app_repo = dao::app::AppRepo::init("test","vector_app");
    // // let entity = dao::app::AppEntity{
    // //     id: None,
    // //     app_id: "new_app1".into(),
    // //     app_secret: "1234".to_string(),
    // //     tenant: "pib_core".into(),
    // //     liaison: "hedetao909".to_owned(),
    // //     system: "pib_core".to_owned(), // 子系统编号
    // //     created_at: Utc::now(),
    // //     updated_at: Utc::now(),
    // //     deleted_at: 0,
    // // };
    // // let ret = app_repo.insert_app(&entity).await;
    // // println!("app_repo insert = {:?}",ret.unwrap().inserted_id);
    // let ret = app_repo.get_app(&"663106a359ff0ccd90542633".to_string()).await;
    // println!("{:?}",ret);
    axum::serve(listener, app).await.unwrap();
}

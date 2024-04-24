use crate::config::cc;
use lazy_static::lazy_static;
use log::{info, warn};
use mongodb::error::Error as MongoError;
use mongodb::{options::ClientOptions, Client};
use std::fmt::format;
use tokio::sync::OnceCell;
pub static MONGO_CLIENT: OnceCell<Client> = OnceCell::const_new();

// init_mongodb 是一个异步任务
pub async fn init_mongodb(_mongo: &cc::MongoConfig) -> &'static Client {
    MONGO_CLIENT
        .get_or_init(|| async {
            let mut client_options = ClientOptions::parse(format!("mongodb://{}", _mongo.hosts))
                .await
                .unwrap();
            // Manually set an option.
            client_options.app_name = Some("My App".to_string());
            // Get a handle to the deployment.
            let client = Client::with_options(client_options).unwrap();
            // List the names of the databases in that deployment.
            for db_name in client.list_database_names(None, None).await.unwrap() {
                info!("db name ={:?}", db_name);
            }
            client
        })
        .await
}

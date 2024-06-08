use crate::config::cc;
use mongodb::{options::ClientOptions, Client};
use serde_derive::{Deserialize, Serialize};
use tokio::sync::OnceCell;
use tracing::info;
// 异步初始化方式
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

mod tests {

    use super::*;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Book {
        title: String,
        author: String,
    }

    #[ctor::ctor]
    fn init() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {

                // configure::global_configure().await;
            })
    }
    #[tokio::test]
    async fn test_insert() {
        // 使用if let 避免使用unwrap
        if let Some(d) = MONGO_CLIENT.get() {
            let db: mongodb::Database = d.database("test");
            let result = db.list_collection_names(None).await.unwrap();
            for c in &result {
                info!("collection = {:?}", c);
            }
            info!("will insert book");
            // let docs = vec![
            //     doc! { "title": "1984", "author": "George Orwell" },
            //     doc! { "title": "Animal Farm", "author": "George Orwell" },
            //     doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
            // ];
            // db.collection("vector_search").insert_many(docs, None).await; // .unwrap().database("test")
            let typed_collection = db.collection::<Book>("books"); // name是collection名

            let books = vec![
                Book {
                    title: "The Grapes of Wrath".to_string(),
                    author: "John Steinbeck".to_string(),
                },
                Book {
                    title: "To Kill a Mockingbird".to_string(),
                    author: "Harper Lee".to_string(),
                },
            ];
            // Insert the books into "mydb.books" collection, no manual conversion to BSON necessary.
            let _ = typed_collection.insert_many(books, None).await;
            info!("book inserted");
        }
    }
}

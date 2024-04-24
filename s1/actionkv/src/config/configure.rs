// use crate::config::{cc, kafka, mongo};
use crate::config::{cc, kafka, mongo};
use log::{info, warn};
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}
pub async fn init_configure() {
    // 这里必须await，并且需要在tokio中初始化，否则可以使用lazy static(无法await),需要调用tokio的 runtime
    mongo::init_mongodb(&(cc::GLOBAL_CONFIG.lock().unwrap().mongo)).await;

    // 使用if let 避免使用unwrap
    if let Some(d) = mongo::MONGO_CLIENT.get() {
        let db = d.database("test");
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
        let typed_collection = db.collection::<Book>("books");

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
    info!("connected kafka");
    let mut v = Vec::new();
    v.push("rust-events");

    // kafka::consume_and_print("127.0.0.1:9092", "test-consumer-group1", v.as_slice()).await;

    let consumer = kafka::KakfaSource::new("127.0.0.1:9092", "test-consumer-group1");
    let (tx, rx) = mpsc::channel(1);

    let shared_consumer = Arc::new(consumer);
    let shared_consumer1 = shared_consumer.clone();
    tokio::spawn(async move {
        shared_consumer1.receive(rx).await;
    });
    // 这里阻塞接收
    shared_consumer.start(&["rust-events"], tx).await; // 这里通知channel
}

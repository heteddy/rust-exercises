pub mod cc;
pub mod kafka;
pub mod logger;
pub mod mongo;
pub mod qdrant;
pub mod redis;

use tracing::{info, Level};
use tracing_subscriber;

pub async fn global_configure() {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .pretty()
        .init();
    // 这里必须await，并且需要在tokio中初始化，否则可以使用lazy static(无法await),需要调用tokio的 runtime
    mongo::init_mongodb(&(cc::GLOBAL_CONFIG.lock().unwrap().mongo)).await;
    // {
    //     let mut v = cc::GLOBAL_CONFIG.lock().unwrap();
    //     v.mongo.replica = "rs1".to_owned();
    //     info!("configure ={:?}", v);
    // }
    info!("connected kafka");
    // let mut v = Vec::new();
    // v.push("rust-events");
    // kafka::consume_and_print("127.0.0.1:9092", "test-consumer-group1", v.as_slice()).await;

    // todo: kafka消费实例
    // let consumer = kafka::KakfaSource::new("127.0.0.1:9092", "test-consumer-group1");
    // let (tx, rx) = mpsc::channel(1);
    //
    // let shared_consumer = Arc::new(consumer);
    // let shared_consumer1 = shared_consumer.clone();
    // tokio::spawn(async move {
    //     shared_consumer1.receive(rx).await;
    // });
    // // 这里阻塞接收
    // tokio::spawn(async move { shared_consumer.start(&["rust-events"], tx).await });
    // 这里通知channel);
}

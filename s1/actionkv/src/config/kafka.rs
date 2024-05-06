use tracing::{event, info, info_span, instrument, span, warn, Level};
use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::{
    stream_consumer::StreamConsumer, BaseConsumer, CommitMode, Consumer, ConsumerContext, Rebalance,
};
use rdkafka::error::{KafkaError, KafkaResult};
use rdkafka::message::{Header, Headers, Message, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::topic_partition_list::TopicPartitionList;
use rdkafka::util::get_rdkafka_version;
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, Mutex};

// todo 定义一个struct，包含kafka broker，topic等，定义一个函数可以接收kakfa 收到的消息

async fn produce(brokers: &str, topic_name: &str) {
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    // This loop is non blocking: all messages will be sent one after the other, without waiting
    // for the results.
    let futures = (0..5)
        .map(|i| async move {
            // The send operation on the topic returns a future, which will be
            // completed once the result or failure from Kafka is received.
            let delivery_status = producer
                .send(
                    FutureRecord::to(topic_name)
                        .payload(&format!("Message {}", i))
                        .key(&format!("Key {}", i))
                        .headers(OwnedHeaders::new().insert(Header {
                            key: "header_key",
                            value: Some("header_value"),
                        })),
                    Duration::from_secs(0),
                )
                .await;

            // This will be executed when the result is received.
            println!("Delivery status for message {} received", i);
            delivery_status
        })
        .collect::<Vec<_>>();

    // This loop will wait until all delivery statuses have been received.
    for future in futures {
        println!("Future completed. Result: {:?}", future.await);
    }
}

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}

// A type alias with your custom consumer can be created for convenience.
type LoggingConsumer = StreamConsumer<CustomContext>;

#[derive(Debug, Clone)]
pub struct KafkaMsg {
    key: Option<Vec<u8>>,
    value: Vec<u8>,
}

impl KafkaMsg {
    pub fn new(key: Option<Vec<u8>>, value: Vec<u8>) -> Self {
        KafkaMsg {
            key,
            value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct KakfaSource {
    brokers: String,
    group: String,
}
// todo 考虑如何销毁kafka source
impl KakfaSource {
    pub fn new(brokers: &str, group: &str) -> Self {
        KakfaSource {
            brokers: brokers.to_owned(),
            group: group.to_owned(),
        }
    }

    pub async fn start(&self, topics: &[&str], tx: mpsc::Sender<KafkaMsg>) {
        let context: CustomContext = CustomContext {};

        let consumer: LoggingConsumer = ClientConfig::new()
            .set("group.id", self.group.clone())
            .set("bootstrap.servers", self.brokers.clone())
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            //.set("statistics.interval.ms", "30000")
            //.set("auto.offset.reset", "smallest")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create_with_context(context)
            .expect("Consumer creation failed");
        // 开始订阅
        consumer
            .subscribe(topics)
            .expect("Can't subscribe to specified topics");

        loop {
            match consumer.recv().await {
                Err(e) => warn!("Kafka error: {}", e),
                Ok(m) => {
                    let payload = match m.payload_view::<str>() {
                        None => "",
                        Some(Ok(s)) => s,
                        Some(Err(e)) => {
                            warn!("Error while deserializing message payload: {:?}", e);
                            ""
                        }
                    };
                    // todo 这里增加异步处理的逻辑；拿到topic，payload，key等信息然后序列化
                    /*
                    &str -> String--| String::from(s) or s.to_string() or s.to_owned()
                    &str -> &[u8]---| s.as_bytes()
                    &str -> Vec<u8>-| s.as_bytes().to_vec() or s.as_bytes().to_owned()
                    String -> &str----| &s if possible* else s.as_str()
                    String -> &[u8]---| s.as_bytes()
                    String -> Vec<u8>-| s.into_bytes()
                    &[u8] -> &str----| std::str::from_utf8(s).unwrap()
                    &[u8] -> String--| String::from_utf8(s).unwrap()
                    &[u8] -> Vec<u8>-| s.to_vec()
                    Vec<u8> -> &str----| std::str::from_utf8(&s).unwrap()
                    Vec<u8> -> String--| String::from_utf8(s).unwrap()
                    Vec<u8> -> &[u8]---| &s if possible* else s.as_slice()
                    */
                    let _ = tx.send(KafkaMsg::new(
                        m.key().map(|s| s.to_vec()),
                        payload.as_bytes().to_vec(),
                    ))
                    .await; // 发送消息是异步的
                            // // 拿到payload，然后反序列化到一个结构中
                            // info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                            //       m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                            // if let Some(headers) = m.headers() {
                            //     for header in headers.iter() {
                            //         // 获取的是应用
                            //         info!("Header {:#?}: {:?}", header.key, header.value);
                            //     }
                            // }
                    consumer.commit_message(&m, CommitMode::Async).unwrap();
                }
            };
        }
    }
    // 启动一个新的task spawn;
    pub async fn receive(&self, mut rx: mpsc::Receiver<KafkaMsg>) -> Result<(), KafkaError> {
        info!("starting receive....");
        loop {
            match rx.recv().await {
                Some(msg) => {
                    info!("received = {:?}", msg);
                }
                None => {
                    warn!("received None");
                }
            }
        }
        // Ok(())
    }
}

// pub async fn consume_and_print(brokers: &str, group_id: &str, topics: &[&str]) {
//     info!("consume_and_print started");
//     // let (tx, mut rx) = mpsc::channel<KafkaMsg>(100);
//     let context: CustomContext = CustomContext {};
//     let consumer: LoggingConsumer = ClientConfig::new()
//         .set("group.id", group_id)
//         .set("bootstrap.servers", brokers)
//         .set("enable.partition.eof", "false")
//         .set("session.timeout.ms", "6000")
//         .set("enable.auto.commit", "true")
//         //.set("statistics.interval.ms", "30000")
//         //.set("auto.offset.reset", "smallest")
//         .set_log_level(RDKafkaLogLevel::Debug)
//         .create_with_context(context)
//         .expect("Consumer creation failed");

//     consumer
//         .subscribe(&topics.to_vec())
//         .expect("Can't subscribe to specified topics");

//     loop {
//         match consumer.recv().await {
//             Err(e) => warn!("Kafka error: {}", e),
//             Ok(m) => {
//                 let payload = match m.payload_view::<str>() {
//                     None => "",
//                     Some(Ok(s)) => s,
//                     Some(Err(e)) => {
//                         warn!("Error while deserializing message payload: {:?}", e);
//                         ""
//                     }
//                 };
//                 // todo 这里增加异步处理的逻辑；拿到topic，payload，key等信息然后序列化
//                 // 拿到payload，然后反序列化到一个结构中
//                 info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
//                       m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
//                 if let Some(headers) = m.headers() {
//                     for header in headers.iter() {
//                         // 获取的是应用
//                         info!("Header {:#?}: {:?}", header.key, header.value);
//                     }
//                 }
//                 consumer.commit_message(&m, CommitMode::Async).unwrap();
//             }
//         };
//     }
// }

mod tests {
    use super::*;
    // #[tokio::test]     // 等同于下面的形式
    // async fn kafka_test() {

    // }
    #[test]
    fn my_test() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                produce("127.0.0.1:9092", "rust-events").await;
            })
    }
}

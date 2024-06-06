use anyhow::Result;
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
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, Mutex};
use tracing::{event, info, info_span, instrument, span, warn, Level};

pub struct KafkaProducer {
    topic: String,
    brokers: String,
    client: FutureProducer,
}

impl KafkaProducer {
    pub fn new(brokers: String, topic: String) -> KafkaProducer {
        let t = topic.to_owned();
        let b = brokers.to_owned();
        let client = ClientConfig::new()
            .set("bootstrap.servers", &brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .unwrap();

        KafkaProducer {
            topic: t,
            brokers: b,
            client,
        }
    }

    pub async fn send<T: Serialize>(&self, key: &str, msg: T) -> Result<(i32, i64)>
    where
        T: Serialize,
    {
        let payload = serde_json::to_string(&msg).unwrap_or_default();

        let record = FutureRecord::to(&self.topic).payload(&payload).key(key);
        // headers 和 payload key只保留一个就行了
        // .headers(OwnedHeaders::new().insert(Header {
        //     key,
        //     value: Some(&payload),
        // }));
        let status = self.client.send(record, Duration::from_secs(0)).await;

        match status {
            Ok(s) => Ok(s),
            Err((e, _)) => Err(anyhow::anyhow!("{}", e)),
        }
    }
}

// // todo 定义一个struct，包含kafka broker，topic等，定义一个函数可以接收kakfa 收到的消息
// async fn produce(brokers: &str, topic_name: &str) {
//     let producer: &FutureProducer = &ClientConfig::new()
//         .set("bootstrap.servers", brokers)
//         .set("message.timeout.ms", "5000")
//         .create()
//         .expect("Producer creation error");

//     // This loop is non blocking: all messages will be sent one after the other, without waiting
//     // for the results.
//     let futures = (0..5)
//         .map(|i| async move {
//             // The send operation on the topic returns a future, which will be
//             // completed once the result or failure from Kafka is received.
//             let delivery_status = producer
//                 .send(
//                     FutureRecord::to(topic_name)
//                         .payload(&format!("Message {}", i))
//                         .key(&format!("Key {}", i))
//                         .headers(OwnedHeaders::new().insert(Header {
//                             key: "header_key",
//                             value: Some("header_value"),
//                         })),
//                     Duration::from_secs(0),
//                 )
//                 .await;

//             // This will be executed when the result is received.
//             println!("Delivery status for message {} received", i);
//             delivery_status
//         })
//         .collect::<Vec<_>>();

//     // This loop will wait until all delivery statuses have been received.
//     for future in futures {
//         println!("Future completed. Result: {:?}", future.await);
//     }
// }

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
        KafkaMsg { key, value }
    }
}
// 不支持clone和debug等
pub struct KakfaSource {
    client: LoggingConsumer,
}
// todo 考虑如何销毁kafka source
impl KakfaSource {
    pub fn new(brokers: &str, group: &str) -> Self {
        let context: CustomContext = CustomContext {};

        let client: LoggingConsumer = ClientConfig::new()
            .set("group.id", group)
            .set("bootstrap.servers", brokers)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            //.set("statistics.interval.ms", "30000")
            //.set("auto.offset.reset", "smallest")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create_with_context(context)
            .unwrap();

        KakfaSource { client }
    }

    pub async fn start(&self, topics: &[&str], tx: mpsc::Sender<KafkaMsg>) {
        // 开始订阅
        self.client
            .subscribe(topics)
            .expect("Can't subscribe to specified topics");

        loop {
            match self.client.recv().await {
                Err(e) => warn!("Kafka received msg error: {}", e),
                Ok(m) => {
                    let payload = match m.payload_view::<str>() {
                        None => "",
                        Some(Ok(s)) => s, // 取出payload
                        Some(Err(e)) => {
                            warn!("Error while deserializing message payload: {:?}", e);
                            ""
                        }
                    };
                    // todo 这里增加异步处理的逻辑；拿到topic，payload，key等信息然后序列化
                    /**
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

                    // 拿到payload，然后反序列化到一个结构中
                    info!(
                        r#"kafka receiver got msg 
                         key: '{:?}', payload: '{}', topic: {}, 
                        partition: {}, offset: {}, timestamp: {:?}"#,
                        m.key(),
                        payload,
                        m.topic(),
                        m.partition(),
                        m.offset(),
                        m.timestamp()
                    );
                    if let Some(headers) = m.headers() {
                        for header in headers.iter() {
                            // 获取的是应用
                            info!("Header {:#?}: {:?}", header.key, header.value);
                        }
                    }
                    let _ = tx
                        .send(KafkaMsg::new(
                            m.key().map(|s: &[u8]| s.to_vec()),
                            payload.as_bytes().to_vec(),
                        ))
                        .await; // 发送消息是异步的

                    let commit_result = self.client.commit_message(&m, CommitMode::Async);
                    commit_result.map_err(|e| {
                        warn!("commit message error :{}", e);
                        // e  还需要返回error么
                    });
                }
            };
        }
    }
    // 启动一个新的task spawn;  这个函数应该是包含receiver的实现
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

mod tests {
    use super::*;
    #[test]
    fn my_test() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                // produce("127.0.0.1:9092", "rust-events").await;
            })
    }
}

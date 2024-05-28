use crate::dao::{
    app::AppEntity, bert::BertEntity, index::IndexEntity, preprocess::PreprocessEntity,
    server::ServerEntity,
};
use crate::pb;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex, RwLock};
use tokio::sync::mpsc;
use tokio::sync::OnceCell;
use tracing::{info, instrument, trace};
use anyhow;
//  定义一个oncecell，然后初始化它
// pub static GLOBAL_SYNCHRONIZER: OnceCell<Synchronizer> = OnceCell::const_new();
lazy_static! {
    pub static ref GLOBAL_SYNCHRONIZER: Mutex<Synchronizer> =
        Mutex::new(Synchronizer::build());
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum SyncMsg {
    App(AppEntity),
    Index(IndexEntity),
    Bert(BertEntity),
    Preprocess(PreprocessEntity),
    Server(ServerEntity),
}
impl pb::entity::Namer for SyncMsg {
    fn name(&self) -> &'static str {
        match self {
            SyncMsg::App(a) => a.name(),
            SyncMsg::Index(a) => a.name(),
            SyncMsg::Bert(a) => a.name(),
            SyncMsg::Preprocess(a) => a.name(),
            SyncMsg::Server(a) => a.name(),
            _ => "",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MsgType {
    APP,
    INDEX,
    BERT,
    PREPROCESS,
    SERVER,
    TEMPLATE,
}

pub trait Messager {
    fn get_type(&self) -> &str;
    fn get_body(&mut self) -> Option<String>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncData {
    msg_type: String,
    body: Option<String>,
}

impl SyncData {
    pub fn new(msg_type: String, body: Option<String>) -> SyncData {
        SyncData { msg_type, body }
    }
}

impl Messager for SyncData {
    fn get_body(&mut self) -> Option<String> {
        self.body.take()
    }
    fn get_type(&self) -> &str {
        &self.msg_type
    }
}

///
#[derive(Debug)] //只能保存listener的引用
pub struct Synchronizer
// where
//     T: Messager + 'static + Send + Sync + Clone,
{
    tx: mpsc::Sender<SyncData>,
    rx: Option<mpsc::Receiver<SyncData>>,
    // 要给另外一个协程使用
    listeners: Arc<RwLock<HashMap<String, Vec<mpsc::Sender<SyncData>>>>>,
}

impl Synchronizer
// where
//     T: Messager + 'static + Send + Sync + Clone + Debug,
{
    #[instrument]
    pub fn build() -> Synchronizer {
        // 已经移动到Synchronizer
        let (tx, rx) = mpsc::channel::<SyncData>(10);
        // 定义一个watch, 实现通知各个repo变化
        Synchronizer {
            tx,
            rx: Some(rx),
            listeners: Arc::new(RwLock::new(HashMap::with_capacity(10))),
        }
    }
    #[instrument]
    pub fn register(&mut self, name: &str, l: mpsc::Sender<SyncData>) {
        let mut t = self.listeners.write().unwrap();
        let e = t.entry(name.to_string()).or_insert(Vec::with_capacity(1));
        e.push(l);
    }

    // #[instrument]
    // pub fn get_tx(&self) -> mpsc::Sender<T> {
    //     self.tx.clone()
    // }
    #[instrument]
    pub async fn send<E>(&self, msg_type: &str, e: &E) ->anyhow::Result<()>
    where
        E: Serialize + Clone + Debug,
    {
        let body = serde_json::to_string(e).unwrap_or_default();
        let m = SyncData::new(msg_type.to_owned(), Some(body));
        self.tx.send(m).await?; // todo:判断是否发送成功
        anyhow::Ok(())
    }

    #[instrument]
    pub async fn receive(&mut self) {
        // 启动一个新的routine
        let rx = self.rx.take();
        // let listeners = self.table.clone();
        if let Some(mut _rx) = rx {
            let table = self.listeners.clone();
            tokio::spawn(async move {
                // todo 优化方式，发送到redis或消息队列，然后都从amqp中订阅
                // 启动一个新的协程处理
                // let received = ;
                while let Some(res) = _rx.recv().await {
                    // todo 收到了更新的通知，分发到指定的
                    info!("received message:{:?}", res);

                    let map = table.read().unwrap();
                    let listener_senders = map.get(res.get_type());
                    info!("received msg={:?}", &res);
                    // 这里虽然没有解析
                    // match listener_senders {
                    //     Some(t) => {
                    //         t.iter().for_each(|l| async {
                    //             l.send(res).await;
            
                    //         });
                    //     }
                    //     None => {}
                    // }
                    // 定义一个watch
                }
            });
        }
    }
}

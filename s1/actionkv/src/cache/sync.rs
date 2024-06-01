//! 采用actor的方式实现，分离actor和actor handler，否则需要保证self是'static
//! actor只处理 接收到的信息，run方法独立到另外一个函数中，可以move actor
//! handler是想actor发送消息方面；
//!

use crate::dao::{
    app::AppEntity, bert::BertEntity, index::IndexEntity, preprocess::PreprocessEntity,
    server::ServerEntity,
};
use crate::pb;
use anyhow;
use core::sync;
use futures::future;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex, RwLock};
use tokio::net::unix::pipe::Receiver;
use tokio::{spawn, sync::mpsc, sync::OnceCell};
use tracing::{info, instrument, trace, warn};

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

    #[instrument]
    pub fn build<E>(msg_type: &str, e: &E) -> SyncData
    where
        E: Serialize + Clone + Debug,
    {
        let body = serde_json::to_string(e).unwrap_or_default();
        let m = SyncData::new(msg_type.to_owned(), Some(body));
        m
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
pub struct Synchronizer {
    // 要给另外一个协程使用
    listeners: Arc<RwLock<HashMap<String, Vec<mpsc::Sender<SyncData>>>>>,
}

impl Synchronizer {
    #[instrument]
    pub fn build() -> Synchronizer {
        // 定义一个watch, 实现通知各个repo变化
        Synchronizer {
            // tx,  // note: 包含tx不能send，因此这里不要再包含send了，也就不能move到新的协程
            listeners: Arc::new(RwLock::new(HashMap::with_capacity(10))),
        }
    }
    #[instrument(skip(self))]
    pub fn register(&mut self, name: impl AsRef<str> + Debug, sender: mpsc::Sender<SyncData>) {
        let mut t = self.listeners.write().unwrap();
        let e = t
            .entry(name.as_ref().to_string())
            .or_insert(Vec::with_capacity(1));
        e.push(sender);
        info!("after registered to hashmap, it's length={:?}", e.len());
    }

    // #[instrument]
    // pub fn get_tx(&self) -> mpsc::Sender<T> {
    //     self.tx.clone()
    // }
    // #[instrument]
    // pub async fn send<E>(&self, msg_type: &str, e: &E) -> anyhow::Result<()>
    // where
    //     E: Serialize + Clone + Debug,
    // {
    //     let body = serde_json::to_string(e).unwrap_or_default();
    //     let m = SyncData::new(msg_type.to_owned(), Some(body));
    //     self.tx.send(m).await?; // todo:判断是否发送成功
    //     anyhow::Ok(())
    // }

    #[instrument(skip(self))]
    pub async fn handle_sync_data(&mut self, data: SyncData) {
        info!("Synchronizer: handle received sync data");
        let map = self.listeners.read().unwrap();
        let listener_senders = map.get(data.get_type());

        // 这里虽然没有解析
        match listener_senders {
            Some(list) => {
                info!("register sender list length={:?}", list.len());

                // future::try_join_all(ids.iter().map(|id| Foo::get(*id)))
                // .await
                // .unwrap();
                // future::try_join_all().await.unwrap();
                // list.iter().for_each(|l| {
                //     // todo 为什么不能用async move
                //     // let l2: mpsc::Sender<SyncData> = l.clone();
                //     let _d2 = data.clone(); // todo 这里能不能用arc
                //     info!("Synchronizer: coroutine in map");
                //     tokio::spawn(async move |l| l.send(_d2).await);

                //     info!(
                //         "iter sender vector and data_type=[{:?}] sent",
                //         data.get_type()
                //     );
                // });
                for i in 0..list.len() {
                    let _d2 = data.clone(); // todo 这里能不能用arc
                    info!("Synchronizer: coroutine in map");
                    let s = list[i].clone(); // 如果要在spawn中使用，需要clone； 因此使用for_each比较麻烦
                    tokio::spawn(async move {
                        //todo：为什么需要启动一个新的协程发送？
                        // This will return an error and send
                        // no message if the buffer is full
                        let _ = s.send(_d2).await;
                    });
                    info!(
                        "iter sender vector and data_type=[{:?}] sent",
                        data.get_type()
                    );
                }
            }
            None => {
                warn!(
                    "not found! sender vector is None; data_type={:?}",
                    data.get_type()
                );
            }
        } // todo map 和 for_each的区别, 下面的代码不能编译
          // match listener_senders {
          //     Some(t) => {
          //         t.iter().for_each(| l| async{
          //             let l2 = l.clone();
          //             let res2 = res.clone();
          //             spawn(async move {
          //                 l2.send(res2).await;
          //             });
          //         });
          //     }
          //     None => {}
          // }
          // 定义一个watch
    }
}

#[instrument(skip(synchronizer))]
pub async fn run_synchronizer(mut synchronizer: Synchronizer, mut _rx: mpsc::Receiver<SyncData>) {
    // 或者放到一个{}里面，
    // let mut _s_lock: std::sync::MutexGuard<Synchronizer> = synchronizer.lock().unwrap();
    // let mut _rx = _s_lock.rx.take().unwrap();
    // drop(_s_lock);
    while let Some(res) = _rx.recv().await {
        // todo 收到了更新的通知，分发到指定的
        info!("received message:{:?}", res);
        synchronizer.handle_sync_data(res).await;
    }
}

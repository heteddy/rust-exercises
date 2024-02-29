use crate::pb::msg::FromServer;
use crate::{core::connection, utils};
use async_std::future::IntoFuture;
use async_std::task;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast::{self, error::RecvError};

// use async_std::

/// 组定义了一个广播channel，组内每个client都对应一个线程，接收广播消息并发送给对应客户端
/// 通常设计是一个group保存了所有的连接，每个调用发送，但是这个设计是每个连接一个线程，而且不是放在连接中定义这个线程
pub struct Group {
    name: Arc<String>,
    sender: broadcast::Sender<Arc<String>>,
}

impl Group {
    pub fn new(name: Arc<String>) -> Self {
        // channel 设置背压
        let (sender, _receiver) = broadcast::channel(1000);
        Group {
            name: name.clone(),
            sender,  //简化的表达式
        }
    }
    /// 加入一个组，向sender订阅，为当前的client创建一个线程，接收消息并发送给client；
    pub fn join(&self, outbound: Arc<connection::Outbound>) {
        // 获取tokio
        let receiver = self.sender.subscribe();
        // 启动一个新线程处理，收到广播消息发送给客户端
        task::spawn(handle_subscribe(self.name.clone(), receiver, outbound));
    }
    pub fn broadcast(&self, msg: Arc<String>) {
        // 当没有人订阅时候会报错，
        let _ret = self.sender.send(msg); // channel会收到消息
    }
}

/// server端启动新线程处理消息转发，每个group一个线程
async fn handle_subscribe(
    group: Arc<String>,
    mut receiver: broadcast::Receiver<Arc<String>>,
    outbound: Arc<connection::Outbound>,
) -> utils::err::ChatResult<()> {
    loop {
        // 接收广播消息，构造FromServer Message
        let packet = match receiver.recv().await {
            Ok(msg) => FromServer::Message {
                group_name: group.clone(),
                message: msg.clone(),
            },
            Err(RecvError::Lagged(n)) => {
                FromServer::Error(format!("Dropped {} messages from {}.", n, group))
            }
            Err(RecvError::Closed) => break,
        };
        // 把message 发送到client
        if outbound.send(packet).await.is_err() {
            break;
        }
    }
    Ok(())
}

pub struct GroupTable(Mutex<HashMap<Arc<String>, Arc<Group>>>);

impl GroupTable {
    pub fn new() -> Self {
        GroupTable(Mutex::new(HashMap::new()))
    }
    pub fn get(&self, name: &String) -> Option<Arc<Group>> {
        let result = self.0.lock();
        let result = result.unwrap();
        // note hashmap的get返回的是option<&T>
        let result = result.get(name);
        // note Option><&T> 调用.cloned() 返回一个新数据
        let result = result.cloned();
        result
    }
    pub fn get_or_create(&self, name: Arc<String>) -> Arc<Group> {
        self.0
            .lock()
            .unwrap()
            .entry(name.clone())
            .or_insert_with(|| Arc::new(Group::new(name.clone())))
            .clone()
    }
}

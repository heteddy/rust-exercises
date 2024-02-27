use crate::pb::msg::FromServer;
use crate::{core::connection, utils};
use async_std::task;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::broadcast::{self, error::RecvError};

// use async_std::

pub struct Group {
    name: Arc<String>,
    sender: broadcast::Sender<Arc<String>>,
}

impl Group {
    pub fn new(name: Arc<String>) -> Self {
        let (sender, _receiver) = broadcast::channel(1000);
        Group {
            name: name.clone(),
            sender: sender,
        }
    }
    /// 加入一个组，向sender订阅
    pub fn join(&self, outbound: Arc<connection::Outbound>) {
        let receiver = self.sender.subscribe();
        // 启动一个新线程处理，收到广播消息发送给客户端
        task::spawn(handle_subscribe(self.name.clone(), receiver, outbound));
    }
    pub fn send(&self, msg: Arc<String>) {
        // 当没有人订阅时候会报错，
        let _ret = self.sender.send(msg);
    }
}

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

use async_std::io::{WriteExt, BufReader};
use async_std::net::TcpStream;
use async_std::stream::StreamExt;
use async_std::sync::{Arc, Mutex};

use crate::core::group::GroupTable;
use crate::pb::msg::{FromServer, FromClient};
use crate::utils::err::ChatResult;

use crate::utils::stream::{send_as_json, receive_as_json};

pub struct Outbound(Mutex<TcpStream>);

impl Outbound {
    pub fn new(stream: TcpStream) -> Self {
        Outbound(Mutex::new(stream))
    }
    pub async fn send(&self, msg: FromServer) -> ChatResult<()> {
        // 先上锁？
        let mut guard = self.0.lock().await;

        // &mut *guard 是什么意思
        send_as_json(&mut *guard, msg).await?;
        guard.flush().await?;
        Ok(())
    }
}

///接收一个新连接，处理这个连接的数据请求，每个连接放到一个新线程中
pub async fn serve(socket: TcpStream, groups: Arc<GroupTable>) -> ChatResult<()> {
    // 由于是异步，并且异步mutex锁住tcp stream 防止多个协程同时向一个tcp中写入； 克隆1份新的用户发送数据
    let outbound = Arc::new(Outbound::new(socket.clone()));
    // 接收消息使用原来的
    let data = BufReader::new(socket);

    let mut from_client = receive_as_json(data);
    while let Some(request_result) = from_client.next().await {
        let request = request_result?;
        let result = match request {
            FromClient::Join { group_name } => {
                // 如果是收到加入group的消息，获取group，加入这个group
                let group = groups.get_or_create(group_name);
                group.join(outbound.clone());
                Ok(())
            }
            // enum里面的必须是跟成员名称一样
            FromClient::Post { group_name, message } => {
                match groups.get(&group_name) { // 拿到group信息，并发送
                    Some(group) => {
                        group.send(message);
                        Ok(())
                    }
                    None => {
                        Err(format!("group {:?} 不存在", group_name))
                    }
                }
            }
        };
    }
    Ok(())
}

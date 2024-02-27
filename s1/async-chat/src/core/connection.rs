use async_std::io::WriteExt;
use async_std::net::TcpStream;
use async_std::sync::{Arc, Mutex};

use crate::pb::msg::FromServer;
use crate::utils;

pub struct Outbound(Mutex<TcpStream>);

impl Outbound {
    pub fn new(stream: TcpStream) -> Self {
        Outbound(Mutex::new(stream))
    }
    pub async fn send(&self, msg: FromServer) -> utils::err::ChatResult<()> {
        // 先上锁？
        let mut guard = self.0.lock().await;

        // &mut *guard 是什么意思
        utils::stream::send_as_json(&mut *guard, msg).await?;
        guard.flush().await?;
        Ok(())
    }
}

#![cfg_attr(
debug_assertions,
allow(
dead_code,
unused_imports,
unused_variables,
unused_assignments,
non_snake_case
)
)]

use async_std::prelude::*;
use async_std::net;
use async_std::task;
use chat::core::command;
// main中引用的方式,使用lib中的名称
use chat::utils::err;
use std::env;

fn main() -> err::ChatResult<()> {
    // 首先获取地址
    let address = std::env::args().nth(1).expect("Usage: client address:port");
    println!("address = {} ", &address);

    task::block_on(async {
        let socket = net::TcpStream::connect(address).await?;
        // 客户端拿到socket，连接到服务器
        socket.set_nodelay(true)?;
        let to_server = command::send_commands(socket.clone());
        let from_server = command::handle_replies(socket);
        from_server.race(to_server).await?;   //这里是不是可以换成其他的方式
        Ok(())
    })
}

#![warn(rust_2018_idioms)]

use async_std::prelude::*;
use async_std::{net, task};
use chat::core::{connection, group};
use chat::utils::err;
use std::sync::Arc;

fn main() -> err::ChatResult<()> {
    let address = std::env::args().nth(1).unwrap();
    println!("server address = {:?}", address);
    let group_table = Arc::new(group::GroupTable::new());
    async_std::task::block_on(async {
        let listener = net::TcpListener::bind(address).await.unwrap();
        let mut connection_iters = listener.incoming();
        while let Some(conn) = connection_iters.next().await {
            let conn = conn.unwrap(); //连接套接字
            //拿到tcp stream
            let groups = group_table.clone();
            task::spawn(async {
                log_error(connection::serve(conn, groups).await);
            });
        }
    });
    Ok(())
}


fn log_error(result: err::ChatResult<()>) {
    if let Err(error) = result {
        eprintln!("Error: {}", error);
    }
}
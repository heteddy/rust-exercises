use crate::utils::err::ChatResult;

use async_std::io::{self, BufRead};
use async_std::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::Unpin;

/// send_as_json 序列化并调用write接口发送
pub async fn send_as_json<S, P>(outbound: &mut S, data: P) -> ChatResult<()>
where
    S: io::Write + Unpin, // 为什么是unpin
    P: Serialize,
{
    let mut buffer = serde_json::to_string(&data).unwrap();
    buffer.push('\n');
    outbound.write_all(buffer.as_bytes()).await?;
    // outbound.flush().await?;
    Ok(())
}

/// 解码函数
pub fn receive_as_json<S, P>(inbound: S) -> impl Stream<Item = ChatResult<P>>
//impl 只能定义一个值
where
    S: BufRead + Unpin,
    P: DeserializeOwned,
{
    inbound.lines().map(|line_result| -> ChatResult<P> {
        let line = line_result?;
        let parsed = serde_json::from_str::<P>(&line)?;
        Ok(parsed)
    })
}

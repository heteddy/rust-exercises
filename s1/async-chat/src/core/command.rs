use crate::pb::msg::{FromClient, FromServer};
use crate::utils::{self, err::ChatResult, stream};
// 这里已经导入ChatResult直接使用
use async_std::prelude::*;
use async_std::{io, net};

pub async fn send_commands(mut to: net::TcpStream) -> ChatResult<()> {
    println!(
        "commands: \n \
        join GROUP \n \
        post GROUP MESSAGE ... \n \
        type Control+D to close connection \n
    "
    );
    let mut command_lines = io::BufReader::new(io::stdin()).lines();
    while let Some(command_result) = command_lines.next().await {
        let command = command_result?;
        // 解析出来command request
        let request = match parse_commands(&command) {
            Some(req) => req,
            None => continue,
        };

        stream::send_as_json(&mut to, request).await?;
        to.flush().await?;
    }
    Ok(())
}

/// 解析用户输入的字符串，构造命令
pub fn parse_commands(input: &str) -> Option<FromClient> {
    // input 被move 所以不需要input是mut
    let (cmd, s2) = get_next_token(input)?; // token返回一直，如果是None，则直接返回None
    match cmd {
        "post" => {
            // 构造post 命令
            let (group, message) = get_next_token(s2)?;
            let message = message.to_string();
            Some(FromClient::newPost(group.to_string(), message))
        }
        "join" => {
            let (group, _) = get_next_token(s2)?;
            Some(FromClient::newJoin(group.to_string()))
        }
        _ => {
            eprintln!("未知命令: {:?}", input);
            None
        }
    }
}

/// get_next_token 当前行空格拆分，提取参数
fn get_next_token(mut input: &str) -> Option<(&str, &str)> {
    println!("input = {:?}", input);

    input = input.trim_start();
    if input.is_empty() {
        return None;
    }

    match input.find(char::is_whitespace) {
        // char::is_whitespace 是一个函数
        Some(index) => Some((&input[0..index], &input[index..])),
        None => Some((input, "")),
    }
    //
}

pub async fn handle_replies(from_server: net::TcpStream) -> ChatResult<()> {
    let buffered = io::BufReader::new(from_server);
    let mut reply_stream = stream::receive_as_json(buffered);
    while let Some(reply) = reply_stream.next().await {
        match reply? {
            msg @ FromServer::Message { .. } => {
                println!("message posted to {:?}", msg)
            }
            _err @ FromServer::Error(..) => {
                println!("error received from server {:?}", _err)
            }
        }
    }
    Ok(())
}

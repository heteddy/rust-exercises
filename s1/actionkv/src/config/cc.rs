use ansi_term::Colour;
use clap::builder::OsStr;
use clap::{Parser, Subcommand};
use lazy_static::lazy_static;
use mongodb::bson::{doc, Document};
use serde::de::Deserialize;
// 调用struct实例的deserialize 方法
use serde_derive::{Deserialize, Serialize};
use serde_json;
use serde_yaml;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{env, fs, thread};
use tokio::sync::mpsc;
use tracing::{event, info, info_span, instrument, span, warn, Level};
lazy_static! {
    pub static ref CLI_ARGS: Cli = init_cli_args();
    // 直接用，不需要传递到另外一个coroutine中，全局变量不需要arc
    pub static ref GLOBAL_CONFIG: Mutex<Configure> = Mutex::new(Configure::build());  // 全局共享不需要arc
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    port: usize,
    timeout: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MongoConfig {
    pub hosts: String,
    pub user: String,
    pub pass: String,
    pub database: String,
    pub replica: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RedisConfig {
    addr: String,
    database: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KafkaConfig {
    broker: String,
    topics: String,
    client_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Configure {
    pub name: String,
    pub kafka_config: KafkaConfig,
    pub redis: RedisConfig,
    pub mongo: MongoConfig,
    pub table: Table,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Table {
    pub app: String,
    pub bert: String,
    pub preprocess: String,
    pub server: String,
    pub setting: String,
    pub mapping: String,
    pub template: String,
}

impl Configure {
    pub fn build() -> Self {
        let mut configure = Self::yaml();
        configure.env_var();
        configure
    }
    /// 通过配置文件初始化
    fn yaml() -> Self {
        let p = CLI_ARGS.file.as_ref().unwrap().as_path();
        let yaml_str = fs::read_to_string(p).unwrap();
        let de = serde_yaml::Deserializer::from_str(&yaml_str);
        let value = Configure::deserialize(de).unwrap();
        info!("{}", serde_json::to_string_pretty(&value).unwrap());
        value
    }
    /// 通过环境变量更新
    fn env_var(&mut self) {
        match env::var::<OsStr>("MONGO_DATABASE".into()) {
            Ok(v) => self.mongo.database = v,
            Err(e) => warn!("VAR NOT FOUND: MONGO_DATABASE"),
        };
    }
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,
    #[arg(short, long)]
    name: Option<String>,
    #[arg(short, long)]
    port: String,
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[instrument]
pub fn init_configure_by_yaml() -> Configure {
    let p = CLI_ARGS.file.as_ref().unwrap().as_path();
    let yaml_str = fs::read_to_string(p).unwrap();
    let value = serde_yaml::from_str::<Configure>(&yaml_str).unwrap();
    // 等效下面的反序列化方式
    // let de = serde_yaml::Deserializer::from_str(&yaml_str);
    // let value = Configure::deserialize(de).unwrap();
    info!("configure={:?}", value);
    value
}

/// 根据环境变量覆盖全局配置
#[instrument]
pub fn init_cli_args() -> Cli {
    let args = Cli::parse();
    for _ in 0..args.count {
        let v = match &args.name {
            Some(n) => n,
            None => "",
        };
        info!("hello {}!", v);
    }
    info!("args = {:?}", args);
    info!(
        "This is {} in color, {} in color and {} in color",
        Colour::Red.paint("red"),
        Colour::Blue.paint("blue"),
        Colour::Green.paint("green")
    );
    args
}

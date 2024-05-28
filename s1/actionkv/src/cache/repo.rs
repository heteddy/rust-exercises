use crate::dao::{
    app::AppEntity, bert::BertEntity, index::IndexEntity, preprocess::PreprocessEntity,
    server::ServerEntity,
};
// use chan::Synchronizer;
use super::chan::Messager;
use crate::cache::chan;
use crate::pb;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json;
use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use tokio::sync::mpsc;
use tracing::{info, instrument, trace, warn};

lazy_static! {
    pub static ref G_INDEX_REPO_INSTANCE: Arc<Mutex<IndexRepo>> =
        Arc::new(Mutex::new(IndexRepo::new()));
    pub static ref G_APP_REPO_INSTANCE: Arc<Mutex<AppRepo>> = Arc::new(Mutex::new(AppRepo::new()));
}

// pub trait TableContainer {
//     fn msg_received<T: std::fmt::Debug>(&mut self, msg: T);
// }

#[derive(Debug, Clone)]
pub struct App {
    app_id: String,
    app_secret: String,
}

#[derive(Debug)]
pub struct AppRepo {
    auth_table: Arc<RwLock<HashMap<String, String>>>,
    // rx: Option<mpsc::Receiver<chan::SyncData>>,
}

impl AppRepo {
    pub fn new() -> Self {
        AppRepo {
            auth_table: Arc::new(RwLock::new(HashMap::with_capacity(10))),
        }
    }

    pub fn auth(&self, app_id: &str, app_secret: &str) -> bool {
        false
    }
    pub async fn update(&mut self, mut rx: mpsc::Receiver<chan::SyncData>) {
        while let Some(ref mut e) = rx.recv().await {
            let t = e.get_type();
            match t {
                "app" => {
                    if let Some(body) = e.get_body() {
                        let ret = serde_json::from_str::<AppEntity>(&body);
                        match ret {
                            Ok(e) => {
                                let app_id = &e.app_id;
                                let app_secret = &e.app_secret;
                                info!("update app_id={:?}, app_secret={:?}", app_id, app_secret);
                                let mut auth_table = self.auth_table.write().unwrap();
                                auth_table
                                    .entry(app_id.to_string())
                                    .and_modify(|v| *v = app_secret.to_string())
                                    .or_insert(app_secret.to_string());
                            }
                            Err(e) => warn!("json decode error:{:?}", e),
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct IndexConfigure {
    name: String,
    app_id: String,
    index: Option<IndexEntity>,
}

#[derive(Debug)]
pub struct IndexRepo {
    index_table: Arc<RwLock<HashMap<String, IndexConfigure>>>, // app_id
                                                               // auth_table: Arc<RwLock<HashMap<String, String>>>,
}

impl IndexRepo {
    pub fn new() -> IndexRepo {
        //全局变量不能
        IndexRepo {
            index_table: Arc::new(RwLock::new(HashMap::with_capacity(10))),
            // auth_table: Arc::new(RwLock::new(HashMap::with_capacity(10))),
        }
    }

    // 这里会一直阻塞因此需要一个新的协程
    pub async fn update(&mut self, mut rx: mpsc::Receiver<chan::SyncData>) {
        while let Some(mut data) = rx.recv().await {
            let t = data.get_type();
            match t {
                "index" => {
                    if let Some(body) = data.get_body() {
                        let ret = serde_json::from_str::<IndexEntity>(&body);
                        match ret {
                            Ok(e) => {
                                // let name = e.name;
                                info!("update name={:?}, app_id={:?}", e.name, e.app_id);
                                let mut index_table = self.index_table.write().unwrap();
                                index_table
                                    .entry(e.name.clone())
                                    .and_modify(|configure| (*configure).index = Some(e.clone()))
                                    .or_insert(IndexConfigure {
                                        name: e.name.clone(),
                                        app_id: e.app_id.clone(),
                                        index: Some(e),
                                    });
                            }
                            Err(e) => warn!("json decode error:{:?}", e),
                        }
                    }
                }
                "bert" => {}
                "preprocess" => {}
                "server" => {}
                "template" => {}
                _ => {}
            }
        }
    }
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum MsgType {
//     APP,
//     INDEX,
//     BERT,
//     PREPROCESS,
//     SERVER,
//     TEMPLATE,
// }

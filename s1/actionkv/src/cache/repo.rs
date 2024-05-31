use crate::dao::{
    app::AppEntity, bert::BertEntity, index::IndexEntity, preprocess::PreprocessEntity,
    server::ServerEntity,
};
// use chan::Synchronizer;
use super::chan::{Messager, SyncData};
use crate::cache::chan;
use crate::pb;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use tokio::sync::mpsc;
use tracing::{info, instrument, trace, warn};
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
    #[instrument]
    pub fn handle_entity(&mut self, e: AppEntity) {
        let app_id = &e.app_id;
        let app_secret = &e.app_secret;
        info!(
            "update app entity: app_id={:?}, app_secret={:?}",
            app_id, app_secret
        );

        let mut auth_table = self.auth_table.write().unwrap();
        auth_table
            .entry(app_id.to_string())
            .and_modify(|v| *v = app_secret.to_string())
            .or_insert(app_secret.to_string());
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
}

impl IndexRepo {
    pub fn new() -> IndexRepo {
        //全局变量不能
        IndexRepo {
            index_table: Arc::new(RwLock::new(HashMap::with_capacity(10))),
        }
    }
    #[instrument]
    pub fn handle_entity(&mut self, e: IndexEntity) {
        info!(
            "update index entity name={:?}, app_id={:?}",
            e.name, e.app_id
        );
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
}

pub struct IndexConfigureRepository {
    app: AppRepo, // 会不会有运行时的问题, refcell 不能send
    index: IndexRepo,
    // rx: mpsc::Receiver<chan::SyncData>,
}

impl IndexConfigureRepository {
    // 需要一个
    pub fn new() -> Self {
        Self {
            app: AppRepo::new(),
            index: IndexRepo::new(),
            // rx,
        }
    }
    pub fn auth(&self, app_id: &str, app_secret: &str) -> bool {
        false
    }
    /// 分发到对应的repo中
    pub fn handle_data(&mut self, mut data: chan::SyncData) {
        let t: &str = data.get_type();
        match t {
            "app" => {
                if let Some(ref body) = data.get_body() {
                    let ret = serde_json::from_str::<AppEntity>(&body);
                    match ret {
                        Ok(e) => {
                            // let name = e.name;
                            self.app.handle_entity(e);
                        }
                        Err(e) => warn!("json decode error:{:?}", e),
                    }
                }
            }
            "index" => {
                if let Some(ref body) = data.get_body() {
                    let ret = serde_json::from_str::<IndexEntity>(&body);
                    match ret {
                        Ok(e) => {
                            // let name = e.name;
                            self.index.handle_entity(e);
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

pub async fn watch_configure_change(
    configure_repo: Arc<Mutex<IndexConfigureRepository>>,
    mut rx: mpsc::Receiver<chan::SyncData>,
) {
    while let Some(mut data) = rx.recv().await {
        configure_repo.lock().unwrap().handle_data(data);
    }
}

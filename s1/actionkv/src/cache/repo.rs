use crate::dao::{
    app::AppEntity, bert::BertEntity, index::IndexEntity, preprocess::PreprocessEntity,
    server::ServerEntity,
};
// use chan::Synchronizer;
use super::sync::{Messager, SyncData};
use crate::cache::sync;
use crate::pb;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json;
use std::any::Any;
use std::borrow::Cow;
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
    table: Arc<RwLock<HashMap<String, String>>>,
}

impl AppRepo {
    pub fn new() -> Self {
        AppRepo {
            table: Arc::new(RwLock::new(HashMap::with_capacity(10))),
        }
    }
    pub fn auth(&self, app_id: impl AsRef<str>, app_secret: impl AsRef<str>) -> bool {
        let map = self.table.read().unwrap();
        let v = map.get(app_id.as_ref());
        match v {
            Some(v) => v.eq(app_secret.as_ref()),
            None => false,
        }
    }
    #[instrument(skip(self))]
    pub fn handle_entity(&mut self, e: AppEntity) {
        let app_id = &e.app_id;
        let app_secret = &e.app_secret;
        info!(
            "update app entity: app_id={:?}, app_secret={:?}",
            app_id, app_secret
        );

        let mut auth_table = self.table.write().unwrap();
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
    table: Arc<RwLock<HashMap<String, IndexConfigure>>>, // app_id
}

impl IndexRepo {
    pub fn new() -> IndexRepo {
        //全局变量不能
        IndexRepo {
            table: Arc::new(RwLock::new(HashMap::with_capacity(10))),
        }
    }
    // #[instrument(skip(self))] // 增加instrument，参数需要满足debug
    pub fn get_app_id(&self, name: impl AsRef<str>) -> String {
        let map = self.table.read().unwrap();
        let v = map.get(name.as_ref());
        
        match v {
            Some(v) => (&v.app_id).into(),
            None => "search-app2".into(), // todo 这里是验证
        }
    }

    #[instrument(skip(self))]
    pub fn handle_entity(&mut self, e: IndexEntity) {
        info!(
            "update index entity name={:?}, app_id={:?}",
            e.name, e.app_id
        );
        let mut index_table = self.table.write().unwrap();
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

pub struct IndexConfigRepo {
    app: AppRepo, // 会不会有运行时的问题, refcell 不能send
    index: IndexRepo,
    // rx: mpsc::Receiver<chan::SyncData>,
}

impl IndexConfigRepo {
    // 需要一个
    pub fn new() -> Self {
        Self {
            app: AppRepo::new(),
            index: IndexRepo::new(),
            // rx,
        }
    }

    pub fn get_instance() -> Arc<Mutex<IndexConfigRepo>> {
        // 使用懒加载创建单例实例
        // 这里使用了 Arc 和 Mutex 来实现线程安全的单例
        // 只有第一次调用 get_instance 时会创建实例，之后都会返回已创建的实例
        static mut INSTANCE: Option<Arc<Mutex<IndexConfigRepo>>> = None;
        unsafe { // 这里是需要在初始化时完成
            INSTANCE
                .get_or_insert_with(|| {
                    Arc::new(Mutex::new(IndexConfigRepo {
                        app: AppRepo::new(),
                        index: IndexRepo::new(),
                    }))
                })
                .clone()
        }
    }
    pub fn auth(
        &self,
        app_id: impl AsRef<str>,
        app_secret: impl AsRef<str>,
        name: impl AsRef<str>,
    ) -> bool {
        let mut _app_id = self.index.get_app_id(name);

        if _app_id.eq(app_id.as_ref()) {
            self.app.auth(app_id, app_secret)
        } else {
            false
        }
    }
    /// 分发到对应的repo中
    #[instrument(skip(self))]
    pub fn handle_data(&mut self, mut data: sync::SyncData) {
        let t: &str = data.get_type();
        info!(
            "index configuration received data_type={:?},msg={:?}",
            t, data
        );
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
    configure_repo: Arc<Mutex<IndexConfigRepo>>,
    mut rx: mpsc::Receiver<sync::SyncData>,
) {
    while let Some(mut data) = rx.recv().await {
        configure_repo.lock().unwrap().handle_data(data);
    }
}

// struct Singleton {
//     // 单例数据
//     data: String,
// }

// impl Singleton {
//     // 获取单例实例的方法
//     fn get_instance() -> Arc<Mutex<Singleton>> {
//         // 使用懒加载创建单例实例
//         // 这里使用了 Arc 和 Mutex 来实现线程安全的单例
//         // 只有第一次调用 get_instance 时会创建实例，之后都会返回已创建的实例
//         static mut INSTANCE: Option<Arc<Mutex<Singleton>>> = None;
//         unsafe {
//             INSTANCE
//                 .get_or_insert_with(|| {
//                     Arc::new(Mutex::new(Singleton {
//                         data: String::from("Singleton instance"),
//                     }))
//                 })
//                 .clone()
//         }
//     }
// }

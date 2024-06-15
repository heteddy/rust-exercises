use super::sync::Messager;
use crate::cache::sync;
use crate::dao::{
    app::{AppDao, AppEntity},
    base::{Entity, EntityDao},
    bert::{BertDao, BertEntity},
    index::{IndexDao, IndexEntity},
    preprocess::{PreprocessDao, PreprocessEntity},
    server::{ServerDao, ServerEntity},
    template::{TemplateDao, TemplateEntity},
};
use futures::{FutureExt, TryFutureExt};
use serde_json;
use std::collections::HashMap;
use std::sync::Once;
use std::sync::{Arc, RwLock};
use std::vec;
use tokio::sync::mpsc;
use tracing::{info, instrument, warn};

// static mut GLOBAL_CONFIGURE_REPO = Option

#[derive(Debug, Clone)]
pub struct App {
    app_id: String,
    app_secret: String,
}

#[derive(Debug)]
struct AppRepo {
    // table: Arc<RwLock<HashMap<String, String>>>,
    table: HashMap<String, String>,
}

impl AppRepo {
    pub fn new() -> Self {
        AppRepo {
            table: HashMap::with_capacity(10),
        }
        // table: Arc::new(RwLock::new(HashMap::with_capacity(10))),
    }

    pub async fn init(&mut self) {
        let d = AppDao::new();
        let result = d.list(0, 100).await;
        let entities = result.unwrap_or_else(|_| Vec::new());
        entities.into_iter().for_each(|e| {
            self.table.insert(e.app_id, e.app_secret);
        })
    }

    pub fn auth(&self, app_id: impl AsRef<str>, app_secret: impl AsRef<str>) -> bool {
        // let map = self.table.read().unwrap();
        // let v = map.get(app_id.as_ref());
        let v = self.table.get(app_id.as_ref());
        match v {
            Some(v) => v.eq(app_secret.as_ref()),
            None => false,
        }
    }
    pub fn get(&self, app_id: impl AsRef<str>) -> Option<String> {
        let o = self.table.get(app_id.as_ref());
        match o {
            Some(e) => Some(e.clone()),
            None => None,
        }
    }
    #[instrument(skip(self))]
    pub fn update_entity(&mut self, e: AppEntity) {
        let app_id = &e.app_id;
        let app_secret = &e.app_secret;
        info!(
            "update app entity: app_id={:?}, app_secret={:?}",
            app_id, app_secret
        );

        // let mut auth_table = self.table.write().unwrap();
        // auth_table
        self.table
            .entry(app_id.to_string())
            .and_modify(|v| *v = app_secret.to_string())
            .or_insert(app_secret.to_string());
    }
    #[instrument(skip(self))]
    pub fn handle_body(&mut self, mut data: sync::SyncData) {
        if let Some(ref body) = data.get_body() {
            let ret = serde_json::from_str::<AppEntity>(body);
            match ret {
                Ok(e) => {
                    // let name = e.name;
                    self.update_entity(e);
                }
                Err(e) => warn!("json decode error:{:?}", e),
            }
        }
    }
}

#[derive(Debug)]
pub struct BertRepo {
    table: HashMap<String, BertEntity>,
}

impl BertRepo {
    pub fn new() -> Self {
        Self {
            // table: Arc::new(RwLock::new(HashMap::with_capacity(10))),
            table: HashMap::with_capacity(10),
        }
    }
    pub async fn init(&mut self) {
        let d = BertDao::new();
        let result = d.list(0, 100).await;
        let entities = result.unwrap_or_else(|_| Vec::new());
        entities.into_iter().for_each(|e| {
            self.table.insert(e.name.clone(), e);
        })
    }
    pub fn get(&self, name: impl AsRef<str>) -> Option<BertEntity> {
        let o = self.table.get(name.as_ref());
        match o {
            Some(e) => Some(e.clone()),
            None => None,
        }
    }
    #[instrument(skip(self))]
    pub fn update_entity(&mut self, e: BertEntity) {
        // let mut auth_table = self.table.write().unwrap();
        // auth_table
        self.table
            .entry(e.name.clone())
            .and_modify(|v| *v = e.clone())
            .or_insert(e);
    }

    #[instrument(skip(self))]
    pub fn handle_body(&mut self, mut data: sync::SyncData) {
        if let Some(ref body) = data.get_body() {
            let ret = serde_json::from_str::<BertEntity>(body);
            match ret {
                Ok(e) => {
                    // let name = e.name;
                    self.update_entity(e);
                }
                Err(e) => warn!("json decode error:{:?}", e),
            }
        }
    }
}

#[derive(Debug)]
pub struct ServerRepo {
    table: HashMap<String, ServerEntity>,
}

impl ServerRepo {
    pub fn new() -> Self {
        Self {
            table: HashMap::with_capacity(10),
        }
    }
    pub fn get(&self, name: impl AsRef<str>) -> Option<ServerEntity> {
        let o = self.table.get(name.as_ref());
        match o {
            Some(e) => Some(e.clone()),
            None => None,
        }
    }
    pub async fn init(&mut self) {
        let d = ServerDao::new();
        let result = d.list(0, 100).await;
        let entities = result.unwrap_or_else(|_| Vec::new());
        entities.into_iter().for_each(|e| {
            self.table.insert(e.name.clone(), e);
        })
    }
    #[instrument(skip(self))]
    pub fn update_entity(&mut self, e: ServerEntity) {
        // let mut auth_table = self.table.write().unwrap();
        // auth_table
        self.table
            .entry(e.name.clone())
            .and_modify(|v| *v = e.clone())
            .or_insert(e);
    }

    #[instrument(skip(self))]
    pub fn handle_body(&mut self, mut data: sync::SyncData) {
        if let Some(ref body) = data.get_body() {
            let ret = serde_json::from_str::<ServerEntity>(body);
            match ret {
                Ok(e) => {
                    // let name = e.name;
                    self.update_entity(e);
                }
                Err(e) => warn!("json decode error:{:?}", e),
            }
        }
    }
}

#[derive(Debug)]
pub struct TemplateRepo {
    table: HashMap<String, TemplateEntity>,
}

impl TemplateRepo {
    pub fn new() -> Self {
        Self {
            table: HashMap::with_capacity(10),
        }
    }
    pub async fn init(&mut self) {
        let d = TemplateDao::new();
        let result = d.list(0, 100).await;
        let entities = result.unwrap_or_else(|_| Vec::new());
        entities.into_iter().for_each(|e| {
            self.table.insert(e.name.clone(), e);
        })
    }
    pub fn get(&self, name: impl AsRef<str>) -> Option<TemplateEntity> {
        let o = self.table.get(name.as_ref());
        match o {
            Some(e) => Some(e.clone()),
            None => None,
        }
    }
    #[instrument(skip(self))]
    pub fn update_entity(&mut self, e: TemplateEntity) {
        // let mut auth_table = self.table.write().unwrap();
        // auth_table
        self.table
            .entry(e.name.clone())
            .and_modify(|v| *v = e.clone())
            .or_insert(e);
    }
    #[instrument(skip(self))]
    pub fn handle_body(&mut self, mut data: sync::SyncData) {
        if let Some(ref body) = data.get_body() {
            let ret = serde_json::from_str::<TemplateEntity>(body);
            match ret {
                Ok(e) => {
                    // let name = e.name;
                    self.update_entity(e);
                }
                Err(e) => warn!("json decode error:{:?}", e),
            }
        }
    }
}

#[derive(Debug)]
pub struct PreprocessRepo {
    table: HashMap<String, PreprocessEntity>,
}

impl PreprocessRepo {
    pub fn new() -> Self {
        PreprocessRepo {
            table: HashMap::with_capacity(10),
        }
    }
    pub async fn init(&mut self) {
        let d = PreprocessDao::new();
        let result = d.list(0, 100).await;
        let entities = result.unwrap_or_else(|_| Vec::new());
        entities.into_iter().for_each(|e| {
            self.table.insert(e.name.clone(), e);
        })
    }
    pub fn get(&self, name: impl AsRef<str>) -> Option<PreprocessEntity> {
        let o = self.table.get(name.as_ref());
        match o {
            Some(e) => Some(e.clone()),
            None => None,
        }
    }
    #[instrument(skip(self))]
    pub fn update_entity(&mut self, e: PreprocessEntity) {
        // let mut auth_table = self.table.write().unwrap();
        // auth_table
        self.table
            .entry(e.name.clone())
            .and_modify(|v| *v = e.clone())
            .or_insert(e);
    }
    #[instrument(skip(self))]
    pub fn handle_body(&mut self, mut data: sync::SyncData) {
        if let Some(ref body) = data.get_body() {
            let ret = serde_json::from_str::<PreprocessEntity>(body);
            match ret {
                Ok(e) => {
                    // let name = e.name;
                    self.update_entity(e);
                }
                Err(e) => warn!("json decode error:{:?}", e),
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
struct IndexRepo {
    table: HashMap<String, IndexConfigure>, // app_id
}

impl IndexRepo {
    pub fn new() -> IndexRepo {
        //全局变量不能
        IndexRepo {
            table: HashMap::with_capacity(10),
        }
    }
    pub async fn init(&mut self) {
        let d = IndexDao::new();
        let result = d.list(0, 100).await;
        let entities = result.unwrap_or_else(|_| Vec::new());
        entities.into_iter().for_each(|e| {
            self.table.insert(
                e.name.clone(),
                IndexConfigure {
                    name: e.name.clone(),
                    app_id: e.app_id.clone(),
                    index: Some(e),
                },
            );
        })
    }
    // #[instrument(skip(self))] // 增加instrument，参数需要满足debug
    pub fn get_app_id(&self, name: impl AsRef<str>) -> String {
        let v = self.table.get(name.as_ref());
        match v {
            Some(v) => (&v.app_id).into(),
            None => "search-app2".into(), // todo 这里是验证
        }
    }
    #[instrument(skip(self))]
    pub fn handle_body(&mut self, mut data: sync::SyncData) {
        if let Some(ref body) = data.get_body() {
            let ret = serde_json::from_str::<IndexEntity>(&body);
            match ret {
                Ok(e) => {
                    // let name = e.name;
                    self.update_entity(e);
                }
                Err(e) => warn!("json decode error:{:?}", e),
            }
        }
    }

    pub fn get(&self, name: impl AsRef<str>) -> Option<IndexEntity> {
        let o = self.table.get(name.as_ref());
        match o {
            Some(e) => e.index.clone(),
            None => None,
        }
    }

    #[instrument(skip(self))]
    pub fn update_entity(&mut self, e: IndexEntity) {
        info!(
            "update index entity name={:?}, app_id={:?}",
            e.name, e.app_id
        );

        self.table
            .entry(e.name.clone())
            .and_modify(|configure| (*configure).index = Some(e.clone()))
            .or_insert(IndexConfigure {
                name: e.name.clone(),
                app_id: e.app_id.clone(),
                index: Some(e),
            });
    }
}

static INIT_CONFIG_REPO: Once = Once::new();
// 因为需要2个函数使用，因此不能放到函数内部
static mut REPO_INSTANCE: Option<Arc<RwLock<IndexConfigRepo>>> = None;

pub struct IndexConfigRepo {
    // 只在config repo中更新，还需要arc么
    app: AppRepo,     // 会不会有运行时的问题, refcell 不能send
    index: IndexRepo, // index configure 没有设置为pub
    pub bert: BertRepo,
    pub server: ServerRepo,
    pub preprocess: PreprocessRepo,
    pub template: TemplateRepo,
}

impl IndexConfigRepo {
    // 需要一个
    pub fn new() -> Self {
        Self {
            app: AppRepo::new(),
            index: IndexRepo::new(),
            bert: BertRepo::new(),
            server: ServerRepo::new(),
            preprocess: PreprocessRepo::new(),
            template: TemplateRepo::new(),
        }
    }

    pub fn get_index(&self, name: impl AsRef<str>) -> Option<IndexEntity> {
        self.index.get(name)
    }

    pub async fn init(&mut self) {
        self.app.init().await;
        self.index.init().await;
        self.bert.init().await;
        self.server.init().await;
        self.preprocess.init().await;
        self.template.init().await;
    }

    pub fn get_instance1() -> Arc<RwLock<IndexConfigRepo>> {
        // 使用懒加载创建单例实例
        // 这里使用了 Arc 和 Mutex 来实现线程安全的单例
        // 只有第一次调用 get_instance 时会创建实例，之后都会返回已创建的实例
        static mut INSTANCE: Option<Arc<RwLock<IndexConfigRepo>>> = None;
        unsafe {
            // 这里是需要在初始化时完成；可以使用rwlock
            INSTANCE
                .get_or_insert_with(|| {
                    Arc::new(RwLock::new(IndexConfigRepo {
                        app: AppRepo::new(),
                        index: IndexRepo::new(),
                        bert: BertRepo::new(),
                        server: ServerRepo::new(),
                        preprocess: PreprocessRepo::new(),
                        template: TemplateRepo::new(),
                    }))
                })
                .clone()
        }
    }
    fn new_instance() {
        unsafe {
            REPO_INSTANCE = Some(Arc::new(RwLock::new(IndexConfigRepo {
                app: AppRepo::new(),
                index: IndexRepo::new(),
                bert: BertRepo::new(),
                server: ServerRepo::new(),
                preprocess: PreprocessRepo::new(),
                template: TemplateRepo::new(),
            })));
        }
    }

    /// 另外一种实现使用onceCell.get_or_init
    pub fn get_instance() -> Arc<RwLock<IndexConfigRepo>> {
        // once 是线程安全的，因此只能被调用一次
        INIT_CONFIG_REPO.call_once(|| IndexConfigRepo::new_instance());
        unsafe {
            // 多线程调用clone,arc是线程安全的
            REPO_INSTANCE.as_ref().unwrap().clone()
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
                self.app.handle_body(data);
            }
            "index" => {
                self.index.handle_body(data);
            }
            "bert" => {
                self.bert.handle_body(data);
            }
            "preprocess" => {
                self.preprocess.handle_body(data);
            }
            "server" => {
                self.server.handle_body(data);
            }
            "template" => {
                self.template.handle_body(data);
            }
            _ => {}
        }
    }
}

pub async fn watch_configure_change(
    configure_repo: Arc<RwLock<IndexConfigRepo>>,
    mut rx: mpsc::Receiver<sync::SyncData>,
) {
    while let Some(data) = rx.recv().await {
        // some data被move所以不加mut也可以
        configure_repo.write().unwrap().handle_data(data);
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

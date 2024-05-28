use crate::dao::{
    app::AppEntity, bert::BertEntity, index::IndexEntity, preprocess::PreprocessEntity,
    server::ServerEntity,
};
// use chan::Synchronizer;
use crate::pb;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use tokio::sync::mpsc;
use tracing::{info, instrument, trace};

lazy_static! {
    // pub static ref GLOBAL_SYNCHRONIZER: Mutex<Synchronizer<SyncMsg>> =
    //     Mutex::new(Synchronizer::<SyncMsg>::build());
    // pub static ref G_INDEX_REPO: Arc<Mutex<Box<dyn Listener<SyncMsg>>>> =
    //     Arc::new(Mutex::new(Box::new(ConfigureRepo::new())));
    pub static ref G_INDEX_REPO_INSTANCE: Arc<Mutex<Box<ConfigureRepo>>> =
        Arc::new(Mutex::new(Box::new(ConfigureRepo::new())));

}

// pub trait TableContainer {
//     fn msg_received<T: std::fmt::Debug>(&mut self, msg: T);
// }

#[derive(Debug, Clone)]
pub struct IndexConfigure {
    name: String,
    app_id: String,
    app_secret: String,
    index: IndexEntity,
}

// struct ConfigureTable(HashMap<String, IndexConfigure>);

// impl TableContainer for ConfigureTable {
//     fn msg_received<T: std::fmt::Debug>(&mut self, msg: T) {
//         info!("received={:?}", msg); //这里不能特化？
//     }
// }
#[derive(Debug, Clone)]
pub struct ConfigureRepo {
    table: Arc<Mutex<HashMap<String, IndexConfigure>>>,
    // sync: Synchronizer<SyncMsg>,
}

impl ConfigureRepo {
    pub fn new() -> ConfigureRepo {
        ConfigureRepo {
            table: Arc::new(Mutex::new(HashMap::with_capacity(10))),
            // sync: Synchronizer::build(),
        }
    }
}

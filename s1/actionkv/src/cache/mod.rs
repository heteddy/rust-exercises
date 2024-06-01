pub mod repo;
pub mod sync;
use crate::dao;
use std::sync::{Arc, Mutex};
use tokio::sync::OnceCell;
use tokio::{self, sync::mpsc};
use tracing::{info, instrument, trace, warn};

/// 这里有问题，需要定义一个inner struct来存储，并且使用refcell，否则无法编译通过
pub fn start_cacher() -> mpsc::Sender<sync::SyncData> {
    let (entity_tx, entity_rx) = mpsc::channel::<sync::SyncData>(1);
    let conf_repo = repo::IndexConfigRepo::get_instance();

    // 这里不能用lazy static，类型不一样，也不想解引用
    tokio::spawn(async move {
        repo::watch_configure_change(conf_repo, entity_rx).await;
    });
    
    let (sync_tx, mut sync_rx) = mpsc::channel::<sync::SyncData>(1);

    let mut synchronizer = sync::Synchronizer::build();
    synchronizer.register("app", entity_tx.clone());
    synchronizer.register("index", entity_tx.clone());

    tokio::spawn(async move {
        sync::run_synchronizer(synchronizer, sync_rx).await;
    });
    sync_tx
}

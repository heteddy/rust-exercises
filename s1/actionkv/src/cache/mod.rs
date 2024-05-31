pub mod repo;
pub mod sync;
use crate::dao;
use std::sync::{Arc, Mutex};
use tokio::sync::OnceCell;
use tokio::{self, sync::mpsc};
use tracing::{info, instrument, trace, warn};

/// 这里有问题，需要定义一个inner struct来存储，并且使用refcell，否则无法编译通过
pub fn start_cacher() -> (
    Arc<Mutex<repo::IndexConfigureRepository>>,
    mpsc::Sender<sync::SyncData>,
) {
    let (entity_tx, entity_rx) = mpsc::channel::<sync::SyncData>(1);
    let configure_rep: repo::IndexConfigureRepository = repo::IndexConfigureRepository::new();
    let conf_rep = Arc::new(Mutex::new(configure_rep));
    let configure1 = conf_rep.clone();

    tokio::spawn(async move {
        repo::watch_configure_change(configure1, entity_rx).await;
    });

    let (sync_tx, mut sync_rx) = mpsc::channel::<sync::SyncData>(1);

    let mut synchronizer = sync::Synchronizer::build();
    synchronizer.register("app", entity_tx.clone());
    synchronizer.register("index", entity_tx.clone());

    tokio::spawn(async move {
        sync::run_synchronizer(synchronizer, sync_rx).await;
    });

    (conf_rep.clone(), sync_tx)
}

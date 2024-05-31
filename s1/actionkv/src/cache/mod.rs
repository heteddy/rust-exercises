pub mod chan;
pub mod repo;
use crate::dao;
use std::sync::{Arc, Mutex};
use tokio::sync::OnceCell;
use tokio::{self, sync::mpsc};
use tracing::{info, instrument, trace, warn};
// use crate::cache::repo;

// pub static G_APP_REPO: OnceCell<repo::AppRepo> = OnceCell::const_new();
// pub static G_INDEX_REPO: OnceCell<repo::IndexRepo> = OnceCell::const_new();

// pub async fn init_app_repo() -> &'static repo::AppRepo {
//     G_APP_REPO.get_or_init(|| async {
//         let mut app_repo = repo::AppRepo::new();
//         let _app_repo = Arc::new(app_repo);
//         let mut _app_repo2 = _app_repo.clone();
//         let (itx, mut irx) = mpsc::channel::<chan::SyncData>(1);

//         tokio::spawn(async move {
//             // let mut _repo = index_repo.lock().unwrap();
//             _app_repo2.update(irx).await;
//         });

//         app_repo

//     }).await
// }

/// 这里有问题，需要定义一个inner struct来存储，并且使用refcell，否则无法编译通过
pub fn start_cacher() -> (
    Arc<Mutex<repo::IndexConfigureRepository>>,
    mpsc::Sender<chan::SyncData>,
) {
    let (entity_tx, entity_rx) = mpsc::channel::<chan::SyncData>(10);
    let configure_rep: repo::IndexConfigureRepository = repo::IndexConfigureRepository::new();
    let conf_rep = Arc::new(Mutex::new(configure_rep));
    let configure1 = conf_rep.clone();

    tokio::spawn(async move {
        repo::watch_configure_change(configure1, entity_rx).await;
    });

    let (sync_tx, mut sync_rx) = mpsc::channel::<chan::SyncData>(10);

    // let _synchronizer = chan::GLOBAL_SYNCHRONIZER.clone();
    // let mut synchronizer = _synchronizer.lock().unwrap();
    let mut synchronizer = chan::Synchronizer::build();
    synchronizer.register("app", entity_tx.clone());
    synchronizer.register("index", entity_tx.clone());

    // let _synchronizer: Arc<Mutex<chan::Synchronizer>> = Arc::new(Mutex::new(synchronizer));
    // let _synchronizer2 = _synchronizer.clone();
    tokio::spawn(async move {
        chan::run_synchronizer(synchronizer, sync_rx).await;
    });
    
    (conf_rep.clone(), sync_tx)
}

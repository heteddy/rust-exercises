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
pub async fn start_cacher() {
    // todo 需要定义drop
    let mut synchronizer = chan::GLOBAL_SYNCHRONIZER.lock().unwrap();
    let (itx, mut irx) = mpsc::channel::<chan::SyncData>(1);

    info!("index_repo starting receiving");
    // let index_repo = repo::G_INDEX_REPO_INSTANCE.clone();
    // tokio::spawn(async move {
    //     // let mut _repo = index_repo.lock().unwrap();
    //     index_repo.lock().unwrap().update(irx).await;
    // });

    info!("index_repo started receiving");
    synchronizer.register("index", itx);

    let (atx, mut arx) = mpsc::channel::<chan::SyncData>(1);

    // tokio::spawn(async move {
    //     let mut app_repo = repo::G_APP_REPO_INSTANCE.lock().unwrap();
    //     app_repo.update(arx).await;
    // });

    let mut app_repo: std::sync::MutexGuard<repo::AppRepo> =
        repo::G_APP_REPO_INSTANCE.lock().unwrap();
    app_repo.update(arx).await;

    info!("app_repo started receiving");
    synchronizer.register("app", atx);

    synchronizer.receive().await;
    info!("synchronizer started watching");
}

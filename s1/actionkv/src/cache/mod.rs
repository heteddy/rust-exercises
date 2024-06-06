pub mod repo;
pub mod sync;
use crate::dao;
use std::sync::{Arc, Mutex};
use tokio::sync::OnceCell;
use tokio::{self, sync::mpsc};
use tracing::{info, instrument, trace, warn};

/// 设计了一个synchronizer，主要是用于将来通过kafka或者redis同步变更消息给所有的实例
/// 将来可以基于redis来做配置缓存，使用2级缓存，第1级使用redis，第2级使用内存；设计一种lru的模式
pub fn start_cacher() -> mpsc::Sender<sync::SyncData> {
    let (entity_tx, entity_rx) = mpsc::channel::<sync::SyncData>(1);
    // 这里使用了锁，因此内部就不需要再上锁了
    let conf_repo: Arc<std::sync::RwLock<repo::IndexConfigRepo>> = repo::IndexConfigRepo::get_instance(); // 并非全局变量需要用arc
    // 这里不能用lazy static，类型不一样，也不想解引用;
    // 是否可以用once cell，然后get获取arc并clone应该也可以实现
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

pub mod chan;
pub mod repo;

use crate::dao;
use std::sync::{Arc, Mutex};
// use crate::cache::repo;

pub async fn start_cacher() {
    chan::GLOBAL_SYNCHRONIZER.lock().unwrap().receive().await;

    // repo::GLOBAL_SYNCHRONIZER
    //     .lock()
    //     .unwrap()
    //     .register(dao::ENTITY_APP, repo::G_INDEX_REPO.clone());

    // let your_struct = dyn_trait.as_any_mut().downcast_mut::<YourStruct>().unwrap();
    // your_struct.info = "HHHH".to_string();

    // let r = repo::G_INDEX_REPO.lock().unwrap().as_any_mut().downcast_mut::<repo::ConfigureRepo>().unwrap();
}

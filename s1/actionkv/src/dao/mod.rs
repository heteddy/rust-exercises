pub mod app;
pub mod bert;
pub mod mapping;
pub mod preprocess;
pub mod server;
pub mod setting;

// use mongodb::bson::oid::ObjectId;
// use serde::{Serialize, Serializer};
use tracing::{error, event, info, info_span, instrument, span, warn, Level};

use crate::{
    config::{self, mongo::MONGO_CLIENT},
    pb,
};
use mongodb::{
    options::{self, IndexOptions}, //modify here
    Client,
    Collection,
    IndexModel,
};

// note 这里是返回值泛型，这种使用关联类型，可以实现很多种类型
// pub trait MongoRepo {
//     type Entity;
//     fn create_indexes(&self, indexes: ) -> Result<(),pb::error::ApiError> {
//         Ok(())
//     }
// }

// 支持返回值的泛化，每个子模块引用这个模块
pub fn get_collection<T>(db: &str, collection: &str) -> Collection<T> {
    let col = MONGO_CLIENT
        .get()
        .unwrap()
        .database(db)
        .collection(collection);
    col
}

// 调用子模块，不会产生循环引用
pub async fn init_indexes() -> Result<(), pb::error::ApiError> {
    let _ = app::AppRepo::create_index().await.map_err(|e| {
        error!("init app repo index error {:?}", e);
        ()
    });
    let _ = bert::BertRepo::create_index().await.map_err(|e| {
        error!("init bert repo index error {:?}", e);
        ()
    });
    Ok(())
}

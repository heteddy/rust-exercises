pub mod app;
pub mod bert;
pub mod index;
pub mod preprocess;
pub mod server;
pub mod template;

use crate::pb::svr::{ApiError, ApiResponse};
use tracing::{error, event, info, info_span, instrument, span, warn, Level};

pub const ENTITY_APP: &'static str = "app_entity";
pub const ENTITY_BERT: &'static str = "bert_entity";
pub const ENTITY_PREPROCESS: &'static str = "preprocess_entity";
pub const ENTITY_SERVER: &'static str = "server_entity";
pub const ENTITY_INDEX: &'static str = "index_entity";
// use mongodb::bson::oid::ObjectId;
// use serde::{Serialize, Serializer};

// note 这里是返回值泛型，这种使用关联类型，可以实现很多种类型
// pub trait MongoRepo {
//     type Entity;
//     fn create_indexes(&self, indexes: ) -> Result<(),ApiError> {
//         Ok(())
//     }
// }

// 调用子模块，不会产生循环引用
pub async fn init_indexes() -> Result<(), ApiError> {
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



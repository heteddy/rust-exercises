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

pub async fn init_indexes() -> Result<(), pb::error::ApiError> {
    let _ = app::AppRepo::create_index().await.map_err(|e| {
        error!("init app repo index error {:?}", e);
        ()
    });

    Ok(())
}

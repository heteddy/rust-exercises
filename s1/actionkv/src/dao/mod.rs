pub mod app;
pub mod mappings;
pub mod settings;

use tracing::{error, event, info, info_span, instrument, span, warn, Level};

use crate::{
    config::{self, mongo::MONGO_CLIENT},
    pb,
};

pub async fn init_indexes() -> Result<(), pb::error::ApiError> {
    let _ = app::AppRepo::init_index().await.map_err(|e| {
        error!("init app repo index error {:?}", e);
        ()
    });

    Ok(())
}

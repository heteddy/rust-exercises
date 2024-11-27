pub mod app;
pub mod base;
pub mod bert;
pub mod index;
pub mod preprocess;
pub mod server;
pub mod template;

use crate::pb::svr::ApiError;
use app::AppDao;
use base::EntityDao;
use bert::BertDao;
use index::IndexDao;
use preprocess::PreprocessDao;
use server::ServerDao;
use template::TemplateDao;
use tracing::error;

// 调用子模块，不会产生循环引用
pub async fn init_indexes() -> Result<(), ApiError> {
    let _ = AppDao::new().create_index().await.map_err(|e| {
        error!("init app repo index error {:?}", e);
        ()
    });
    BertDao::new().create_index().await.map_err(|e| {
        error!("init bert repo index error {:?}", e);
    });
    IndexDao::new().create_index().await.map_err(|e| {
        error!("init index repo index error {:?}", e);
    });
    PreprocessDao::new().create_index().await.map_err(|e| {
        error!("init preprocess repo index error {:?}", e);
    });
    ServerDao::new().create_index().await.map_err(|e| {
        error!("init server repo index error {:?}", e);
    });
    TemplateDao::new().create_index().await.map_err(|e| {
        error!("init template repo index error {:?}", e);
    });
    Ok(())
}

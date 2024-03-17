use crate::endpoints::hello;
use axum::Router;

pub fn init_app() -> axum::Router {
    // 会move
    let mut app = axum::Router::new();
    app = hello::register_hello(app);
    app
}

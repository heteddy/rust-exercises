use crate::endpoints::{hello, user};
use axum::Router;

pub fn init_app() -> axum::Router {
    // 会move
    let mut app = axum::Router::new();
    app = app.merge(hello::register_hello())
        .merge(user::register_user());
    app
}

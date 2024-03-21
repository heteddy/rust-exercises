use crate::pb::user::User;
use crate::service::user;
use axum::extract::{Json, Path, Query};
use axum::routing::get;
/// handler是endpoint/controller层
use axum::Router;
use serde::{self, Deserialize};
use serde_json::to_string;

async fn hello() -> &'static str {
    "hello world"
}

// 怎么使用声明周期
async fn access<'a>(Path(name): Path<String>) -> Json<User> {
    // Json(User::new(10 as u64, &name))
    // let json_user = to_string(&User::new(10 as u64, &name));
    // println!("json_user={:?}", json_user.unwrap());
    Json(User::new(10 as u64, &name))
}

pub fn register_hello() -> Router {
    // route是一个move函数
    let hello_router = Router::new().route("/", get(hello));
    let hello_router = hello_router.route("/access/:id", get(access));
    hello_router
}

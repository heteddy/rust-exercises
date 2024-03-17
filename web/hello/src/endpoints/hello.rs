/// handler是endpoint/controller层
use axum::Router;
use axum::routing::get;
use crate::service::user;

async fn hello() -> &'static str {
    "hello world"
}



pub fn register_hello(r: Router) -> Router{
    
    // route是一个move函数
    r.route("/", get(hello))
}
use crate::service::user;
use axum::extract::{Json, Path, Query};
use axum::routing::{get, post, put, delete};
/// handler是endpoint/controller层
use axum::Router;
use std::vec::Vec;
use serde_json::{Value, json};

#[derive(Serialize, Deserialize)]
pub struct UserLoginReq<'a> {
    id: u64,
    username: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUser<'a> {
    username: &'a str,
}


async fn login() {}

/// users/<id>  
async fn get_user(Path(id): Path<u64>) -> String {
    println!("received get id={:?}", id);
    "".to_owned()
}

async fn create_user() -> String {
    "".to_owned()
}

async fn all() -> vec<String> {
    Vec::new()
}

pub fn register_user(r: Router) -> Router {
    // route是一个move函数
    r.route("/users/<id>", post(get_user))
}

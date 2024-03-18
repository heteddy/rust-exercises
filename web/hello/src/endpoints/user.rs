use crate::service::user;
use axum::extract::{Json, Path, Query};
use axum::routing::{delete, get, post, put};
/// handler是endpoint/controller层
// use http::StatusCode;
use axum::Router;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::vec::Vec;

#[derive(Serialize, Deserialize)]
pub struct UserLoginReq<'a> {
    id: u64,
    username: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUser {
    username: String,
}

async fn login() {}

/// users/<id>  
async fn get_user(Path(id): Path<u64>) -> String {
    println!("received get id={:?}", id);
    "".to_owned()
}

async fn create_user(Json(payload): Json<CreateUser>)  {
    //"create_user".to_owned()
    println!("received get id={:?}", payload);
    // (StatusCode::CREATED, Json(payload))
}

// Query参数，eg. /users?id=123&name=jim
async fn query_users(Query(params): Query<HashMap<String, String>>) -> String {
    "query_users".to_owned() // 不能返回Vec<String>?
}

pub fn register_user(r: Router) -> Router {
    // route是一个move函数
    let r = r.route("/users/<id>", post(get_user));
    let r = r.route("/users", get(query_users).post(create_user));
    r
}

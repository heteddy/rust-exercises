use crate::pb::user::User;
use crate::service::user::{self, UserRepo};
use axum::extract::{Json, Path, Query, State};
use axum::routing::{delete, get, post, put};
/// handler是endpoint/controller层
// use http::StatusCode;
use axum::Router;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string, Value};
use std::collections::HashMap;
use std::sync::Arc;
use std::vec::Vec;

#[derive(Serialize, Deserialize)]
pub struct UserLoginReq<'a> {
    id: u64,
    username: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUser {
    id: u64,
    username: String,
}

async fn login() {}

/// users/:id
async fn get_user(State(repo): State<UserRepo>, Path(id): Path<u64>) -> String {
    println!("received get id={:?}", id);

    "".to_owned()
}

//State(repo): State<UserRepo>,
async fn create_user(State(repo): State<UserRepo>, payload: Json<CreateUser>) -> String {
    //"create_user".to_owned()
    // println!("received get id={:?}, repo={:?}", payload, repo);
    // (StatusCode::CREATED, Json(payload))
    let u = repo.create_user(payload.id, &payload.username);
    "ok".to_owned()
}

// Query参数，eg. /users?id=123&name=jim
async fn query_users(
    // State(repo): State<UserRepo>,
    Query(params): Query<HashMap<String, String>>,
    State(repo): State<UserRepo>,
) -> Json<bool> {
    // "query_users".to_owned() // 不能返回Vec<String>?
    println!("query={:?}", params);
    // let u: Option<Arc<User>> = repo.get_user(1 as u64, params.get("username").unwrap());
    let u: Option<Arc<User>> = repo.get_user(1 as u64, params.get("username").unwrap());
    println!("query  u={:?}", u);
    Json(u.is_some())
}

pub fn register_user() -> Router {
    let state_user = UserRepo::new();

    // route是一个move函数
    let mut user_router = Router::new();
    user_router = user_router.route("/users", get(query_users).post(create_user));
    let user_id_router = Router::new().route("/:id", get(get_user));

    user_router = user_router.nest("/:id", user_id_router);
    user_router.with_state(state_user)
}

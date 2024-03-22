use crate::pb::{user::User, RespVO, ResponseBody};
use crate::service::user::{self, UserRepo};
use axum::extract::{Json, Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
/// handler是endpoint/controller层
// use http::StatusCode;
use axum::Router;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string, Value};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use std::vec::Vec;
use tokio::time::sleep;
use tracing::{event, info, instrument, Level};
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
#[instrument(skip_all)]
async fn get_user(State(repo): State<UserRepo>, Path(id): Path<u64>) -> Json<RespVO<User>> {
    event!(Level::INFO, "received id={:?}", id);

    println!("received get id={:?}", id);

    // if rand::random() {
    //     sleep(Duration::new(0, 300000)).await;
    // }

    let u = repo.get_user(id);
    match u {
        Some(_u) => Json(RespVO::from_result(&_u)),
        None => Json(RespVO::from_error_info(StatusCode::NOT_FOUND, "用户不存在")),
    }
}

//State(repo): State<UserRepo>,
#[instrument(skip_all)]
async fn create_user(
    State(repo): State<UserRepo>,
    payload: Json<CreateUser>,
) -> Json<RespVO<User>> {
    event!(Level::INFO, "endpoint create user {:?}", payload);
    //"create_user".to_owned()
    // println!("received get id={:?}, repo={:?}", payload, repo);
    // (StatusCode::CREATED, Json(payload))
    let u = repo.create_user(payload.id, &payload.username);
    event!(Level::INFO, "endpoint created user {:?}", u);
    Json(RespVO::from_result(&u))
}

// 这个state放在第几个参数都可以？ why？
// Query参数，eg. /users?id=123&name=jim
async fn query_users(
    // State(repo): State<UserRepo>,
    Query(params): Query<HashMap<String, String>>,
    State(repo): State<UserRepo>,
) -> Json<RespVO<User>> {
    // "query_users".to_owned() // 不能返回Vec<String>?
    println!("query={:?}", params);
    // let u: Option<Arc<User>> = repo.get_user(1 as u64, params.get("username").unwrap());
    let id = params.get("id").unwrap().parse::<u64>().unwrap();

    let username = params.get("username").unwrap();

    let u: Option<Arc<User>> = repo.get_user(id);
    println!("query  u={:?}", u);
    // u.map(|_u|{
    //     (*_u).clone()
    // })
    match u {
        Some(arc_u) => Json(RespVO::from_result(&arc_u)),
        None => Json(RespVO::from_error_info(StatusCode::NOT_FOUND, "用户不存在")),
    }
    // Json(u.is_some())
}

pub fn register_user() -> Router {
    let state_user = UserRepo::new();

    // route是一个move函数
    let mut user_router = Router::new();

    user_router = user_router.route("/users", get(query_users).post(create_user));
    // 这里使用nest会报错
    user_router = user_router.route("/users/:id", get(get_user));
    // user_router.with_state(state_user);
    Router::new()
        .nest("/api", user_router)
        .with_state(state_user)
}

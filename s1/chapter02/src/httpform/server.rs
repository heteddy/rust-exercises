use super::request::GcdParameter;
use crate::gcd::calc;
use actix_web::{get, post, web, Handler, HttpResponse, Responder};
use serde_json::json; // 直接引用同项目中的模块

pub async fn post_gcd(p: web::Json<GcdParameter>) -> impl Responder {
    println!("{:?}",&p);
    if p.n == 0 || p.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("computing the gcd with zero is not allowed");
    }
    HttpResponse::Ok().content_type("text/json").body(
        json!({"m":p.m, "n":p.n, "gcd": calc::gcd_calc_function(p.m,p.n)}).to_string(),
    )
}

#[get("/ok")]
async fn ok() -> impl Responder {
    // 为什么这个不需要pub
    HttpResponse::Ok()
        .content_type("text/json")
        .body(json!({"status":"success","msg":"", "data": []}).to_string())
}

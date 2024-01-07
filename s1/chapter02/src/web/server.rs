use actix_web::{web, HttpResponse};

use super::request::GcdParameter;

fn post_gcd(form: web::Form<GcdParameter>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest().content_type("text/json").body("");
    }
    HttpResponse::Ok().content_type("text/json").body("")
}

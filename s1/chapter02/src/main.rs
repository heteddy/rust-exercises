#![allow(dead_code, unused_imports)]

use actix_web::{web, App, HttpServer};

mod arg;
mod gcd; // main里面虽然没有用，但是要声明mod gcd 否则不会加入到工程
mod httpform;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    // println!("{:?}", gcd::calc::gcd_calc_function(100, 2));
    let d: u64 = arg::gcd_arg();
    println!("the max gcd is {}", d);

    let server = HttpServer::new(move || {
        App::new().service(
            web::scope("/chapter02")
                .service(web::resource("/gcd").route(web::post().to(httpform::server::post_gcd)))
                .service(httpform::server::ok),
        )
    });
    server.bind("127.0.0.1:8091")?.run().await
}

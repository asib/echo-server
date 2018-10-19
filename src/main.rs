extern crate actix_web;
extern crate serde_json;
extern crate env_logger;
use std::env;
use actix_web::{server, App, Json, http};

fn index(req: Json<serde_json::Value>) -> &'static str {
    println!("{:#}", req);
    "OK!"
}

fn main() {
    env::set_var("RUST_LOG", "info");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    server::new(|| App::new().resource("/", |r| r.method(http::Method::POST).with(index)))
        .bind("127.0.0.1:3030")
        .unwrap()
        .run();
}

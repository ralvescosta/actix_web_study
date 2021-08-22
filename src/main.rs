#[macro_use]
extern crate diesel;

mod applications;
mod controllers;
mod database;
mod models;
mod repositories;
mod routes;
mod schema;
mod view_models;

use actix_web::{middleware, App, HttpServer};
use env_logger;
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(routes::something::register)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

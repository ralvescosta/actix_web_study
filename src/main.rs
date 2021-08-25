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
use applications::something::SomethingUseCase;
use env_logger;
use repositories::something::SomethingRepository;
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        let something_repository = SomethingRepository::new();
        let something_application = SomethingUseCase::new(Box::new(something_repository));
        App::new()
            .data(something_application)
            .wrap(middleware::Logger::default())
            .configure(routes::something::register)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

//async,await / paralelismo / Actor model
mod applications;
mod controllers;
mod models;

use actix_web::{middleware, App, HttpServer};
use env_logger;
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(controllers::something::index)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

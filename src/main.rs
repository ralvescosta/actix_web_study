//async,await / paralelismo / Actor model
use actix_web::{middleware, web, App, HttpServer, Responder};
use env_logger;
use std::io::Result;

async fn status() -> impl Responder {
    "{\"status\": \"UP\"}"
}

#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(status))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

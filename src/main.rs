//async,await / paralelismo / Actor model
mod application;
mod controller;
mod routes;
mod something_dto;
mod something_entity;

use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};
use env_logger;
use std::io::Result;

#[get("/hi")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello sunshine!")
}

#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let usecase = crate::application::SomethingUseCase::new();
    let controller = crate::controller::SomethingController::new(usecase);
    let route = crate::routes::SomethingRoutes::new(controller);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .configure(route.something_routes())
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

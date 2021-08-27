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

use actix_cors::Cors;
use actix_web::{http, middleware, web, App, HttpServer};
use applications::something::{ISomethingUseCase, SomethingUseCase};
use env_logger;
use repositories::something::SomethingRepository;
use std::{io::Result, sync::Arc};

#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    HttpServer::new(|| {
        /**************Cors**************/
        let cors = Cors::default()
            .allowed_origin("All")
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::ORIGIN,
                http::header::LOCATION,
                http::header::HOST,
                http::header::USER_AGENT,
                http::header::CONTENT_LENGTH,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        let something_repository = SomethingRepository::new();
        let something_application = SomethingUseCase::new(Box::new(something_repository));

        App::new()
            .app_data(web::Data::<Arc<dyn ISomethingUseCase>>::new(Arc::new(
                something_application,
            )))
            .wrap(middleware::Compress::default())
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .data(
                web::JsonConfig::default()
                    .error_handler(controllers::deserializer_error_handler::handler),
            )
            .configure(routes::something::register)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

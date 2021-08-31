#[macro_use]
extern crate diesel;

mod applications;
mod controllers;
mod database;
mod logger;
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
use slog::{o, Drain};
use slog_term;
use std::{io::Result, sync::Arc};

fn config_log() -> logger::log::MyLogger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain)
        .build()
        .filter_level(slog::Level::Trace)
        .fuse();
    let log = slog::Logger::root(drain, o!());
    logger::log::MyLogger::new(log)
}

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
        let headers = middleware::DefaultHeaders::new()
            .header("Access-Control-Allow-Origin", "*")
            .header("Content-Security-Policy", "default-src 'self';base-uri 'self';block-all-mixed-content;font-src 'self' https: data:;frame-ancestors 'self';img-src 'self' data:;object-src 'none';script-src 'self';script-src-attr 'none';style-src 'self' https: 'unsafe-inline';upgrade-insecure-requests")
            .header("X-DNS-Prefetch-Control", "	off")
            .header("Expect-CT", "max-age=0")
            .header("X-Frame-Options", "SAMEORIGIN")
            .header("Strict-Transport-Security", "max-age=15552000; includeSubDomains")
            .header("X-Download-Options", "noopen")
            .header("X-Content-Type-Options", "nosniff")
            .header("X-Permitted-Cross-Domain-Policies", "none")
            .header("Referrer-Policy", "no-referrer")
            .header("X-XSS-Protection","0")
            .header("ETag", "W/\"213-XP2qvFfd8eh4EzgQSHCwnbPqiP4\"")
            .header("Vary", "Accept-Encoding");
        let log = config_log();

        let something_repository = SomethingRepository::new();
        let something_application = SomethingUseCase::new(Box::new(something_repository));
        App::new()
            .data(log)
            .app_data(web::Data::<Arc<dyn ISomethingUseCase>>::new(Arc::new(
                something_application,
            )))
            .wrap(middleware::Compress::default())
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(headers)
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

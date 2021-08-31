use std::sync::Arc;

use actix_web::{post, web, HttpResponse, Responder};

use crate::{
    applications::something::ISomethingUseCase, logger::log::MyLogger,
    view_models::something::SomethingViewModel,
};

#[post("/something")]
pub async fn something(
    use_case: web::Data<Arc<dyn ISomethingUseCase>>,
    log: web::Data<MyLogger>,
    body: web::Json<SomethingViewModel>,
) -> impl Responder {
    let result = use_case.perform(body.0).await;
    log.debug(&result);
    HttpResponse::Ok().json(result)
}

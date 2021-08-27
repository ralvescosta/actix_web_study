use std::sync::Arc;

use actix_web::{post, web, HttpResponse, Responder};

use crate::{
    applications::something::ISomethingUseCase, view_models::something::SomethingViewModel,
};

#[post("/something")]
pub async fn something(
    use_case: web::Data<Arc<dyn ISomethingUseCase>>,
    body: web::Json<SomethingViewModel>,
) -> impl Responder {
    let result = use_case.perform(body.0).await;
    HttpResponse::Ok().json(result)
}

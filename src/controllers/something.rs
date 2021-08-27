use std::sync::Arc;

use actix_web::{post, web, HttpResponse, Responder};

use crate::{
    applications::something::SomethingUseCase, view_models::something::SomethingViewModel,
};

#[post("/something")]
pub async fn something(
    use_case: web::Data<Arc<SomethingUseCase>>,
    body: web::Json<SomethingViewModel>,
) -> impl Responder {
    let result = use_case.perform(body.0).await;
    HttpResponse::Ok().json(result)
}

use actix_web::{get, HttpResponse, Responder};

use crate::{
    applications::something::SomethingUseCase, view_models::something::SomethingViewModel,
};

#[get("/hi")]
pub async fn index() -> impl Responder {
    let result = crate::applications::something::perform(SomethingViewModel {
        alias: String::from("Alias"),
        href: String::from("Ref"),
        name: String::from("Name"),
        number: 16,
    })
    .await;
    HttpResponse::Ok().json(result)
}

pub async fn something(usecase: &SomethingUseCase) -> impl Responder {
    let result = usecase.perform().await;
    HttpResponse::Ok().json(result)
}

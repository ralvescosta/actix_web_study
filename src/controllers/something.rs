use actix_web::{get, HttpResponse, Responder};

use crate::models::something::SomethingModel;

#[get("/hi")]
pub async fn index() -> impl Responder {
    let result = crate::applications::something::perform(SomethingModel {
        alias: String::from("Alias"),
        href: String::from("Ref"),
        name: String::from("Name"),
        number: 16,
    });
    HttpResponse::Ok().json(result)
}

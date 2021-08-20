use crate::applications::something::SomethingUseCase;
use actix_web::{get, HttpResponse, Responder};

#[get("/hi")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello sunshine!")
}

pub struct SomethingController {
    app: SomethingUseCase,
}

impl SomethingController {
    pub fn indexs(&self) {}
}

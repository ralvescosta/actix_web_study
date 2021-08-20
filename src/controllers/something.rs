use crate::{applications::something::SomethingUseCase, models::something::SomethingModel};
use actix_web::{get, web, HttpResponse, Responder};
// use futures::{future::result, Future};

// #[get("/hi")]
// async fn index() -> impl Responder {
//     HttpResponse::Ok().body("Hello sunshine!")
// }

pub struct SomethingController {
    app: SomethingUseCase,
}

impl SomethingController {
    // pub async fn index(&self) -> impl Future<Item = HttpResponse, Error = Error> {
    //     |body| String::from("Oii, eu sou o Goku!")
    // }
}

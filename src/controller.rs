use actix_web::web;

use crate::{application::SomethingUseCase, something_dto::SomethingDto};

pub struct SomethingController {
    usecase: SomethingUseCase,
}

impl SomethingController {
    pub fn new(usecase: SomethingUseCase) -> SomethingController {
        SomethingController { usecase }
    }
}

pub async fn get(body: web::Json<SomethingDto>) -> String {
    let dto = SomethingDto {
        alias: String::from("Alias"),
        href: String::from("HRef"),
        name: String::from("Name"),
        number: 28,
    };
    // self.usecase.perform(dto);

    String::from("")
}

use actix_web::web;

use crate::{application::SomethingUseCase, something_dto::SomethingDto};

#[derive(Clone)]
pub struct SomethingController {
    usecase: SomethingUseCase,
}

impl SomethingController {
    pub fn new(usecase: SomethingUseCase) -> SomethingController {
        SomethingController { usecase }
    }
}

pub async fn get(dto: web::Json<SomethingDto>) -> String {
    String::from("")
}

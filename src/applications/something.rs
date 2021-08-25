use view_models::something::SomethingViewModel;

use crate::{
    models::something::SomethingModel, repositories::something::SomethingRepository, view_models,
};

pub async fn perform(dto: SomethingViewModel) -> SomethingModel {
    crate::repositories::something::get_somethings();

    return SomethingModel {
        id: 1,
        alias: dto.alias,
        href: dto.href,
        name: dto.name,
        number: dto.number,
    };
}

#[derive(Debug, Clone)]
pub struct SomethingUseCase {
    repo: Box<SomethingRepository>,
}

impl SomethingUseCase {
    pub fn new(repo: Box<SomethingRepository>) -> SomethingUseCase {
        SomethingUseCase { repo }
    }
    pub async fn perform(&self) -> SomethingModel {
        self.repo.something();
        SomethingModel {
            id: 1,
            alias: String::from("Alias"),
            href: String::from("HRef"),
            name: String::from("Name"),
            number: 10,
        }
    }
}

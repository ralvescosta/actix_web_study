use view_models::something::SomethingViewModel;

use crate::{
    models::something::SomethingModel, repositories::something::SomethingRepository, view_models,
};

#[derive(Debug, Clone)]
pub struct SomethingUseCase {
    repo: Box<SomethingRepository>,
}

impl SomethingUseCase {
    pub fn new(repo: Box<SomethingRepository>) -> SomethingUseCase {
        SomethingUseCase { repo }
    }
    pub async fn perform(&self, view_model: SomethingViewModel) -> SomethingModel {
        self.repo.something();
        SomethingModel {
            id: 1,
            alias: view_model.alias,
            href: view_model.href,
            name: view_model.name,
            number: view_model.number,
        }
    }
}

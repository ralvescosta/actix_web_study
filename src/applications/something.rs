use async_trait::async_trait;

use crate::{
    models::something::SomethingModel, repositories::something::SomethingRepository,
    view_models::something::SomethingViewModel,
};

#[async_trait]
pub trait ISomethingUseCase {
    async fn perform(&self, view_model: SomethingViewModel) -> SomethingModel;
}

#[derive(Debug, Clone)]
pub struct SomethingUseCase {
    repo: Box<SomethingRepository>,
}

#[async_trait]
impl ISomethingUseCase for SomethingUseCase {
    async fn perform(&self, view_model: SomethingViewModel) -> SomethingModel {
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

impl SomethingUseCase {
    pub fn new(repo: Box<SomethingRepository>) -> SomethingUseCase {
        SomethingUseCase { repo }
    }
}

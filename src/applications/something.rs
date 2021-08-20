use models::something::SomethingModel;

use crate::models;

#[derive(Clone)]
pub struct SomethingUseCase {}

impl SomethingUseCase {
    pub fn new() -> SomethingUseCase {
        SomethingUseCase {}
    }
    pub fn perform(&self, dto: SomethingModel) -> SomethingEntity {
        return SomethingEntity {
            id: 1,
            alias: dto.alias,
            href: dto.href,
            name: dto.name,
            number: dto.number,
        };
    }
}

pub struct SomethingEntity {
    pub id: u32,
    pub name: String,
    pub number: i32,
    pub alias: String,
    pub href: String,
}

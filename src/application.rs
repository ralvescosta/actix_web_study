use crate::{something_dto::SomethingDto, something_entity::SomethingEntity};

pub struct SomethingUseCase {}

impl SomethingUseCase {
    pub fn perform(&self, dto: SomethingDto) -> SomethingEntity {
        return SomethingEntity {
            id: 1,
            alias: dto.alias,
            href: dto.href,
            name: dto.name,
            number: dto.number,
        };
    }
}

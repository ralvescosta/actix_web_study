use models::something::SomethingModel;
use serde::Serialize;

use crate::models;

pub fn perform(dto: SomethingModel) -> SomethingEntity {
    return SomethingEntity {
        id: 1,
        alias: dto.alias,
        href: dto.href,
        name: dto.name,
        number: dto.number,
    };
}

#[derive(Serialize)]
pub struct SomethingEntity {
    pub id: u32,
    pub name: String,
    pub number: i32,
    pub alias: String,
    pub href: String,
}

use view_models::something::SomethingViewModel;

use crate::{models::something::SomethingModel, view_models};

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

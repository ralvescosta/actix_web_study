use crate::{database::establish_connection, models::something::SomethingModel};

extern crate diesel;
use diesel::prelude::*;

pub fn get_somethings() -> Vec<SomethingModel> {
    use crate::schema::somethings::dsl::*;

    let connection = establish_connection();

    somethings
        .filter(name.eq("name"))
        .limit(5)
        .load(&connection)
        .expect("Error loading somethings")
}

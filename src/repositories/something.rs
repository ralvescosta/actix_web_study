use crate::{
    database::establish_connection,
    models::{new_something::NewSomething, something::SomethingModel},
};

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

pub fn create<'a>(name: &'a str, number: i32, alias: &'a str, href: &'a str) {
    use crate::schema::somethings;

    let connection = establish_connection();

    let new_something = NewSomething {
        alias,
        href,
        name,
        number,
    };

    diesel::insert_into(somethings::table)
        .values(&new_something)
        .execute(&connection)
        .expect("Error saving new post");
}

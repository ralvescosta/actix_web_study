use diesel::Queryable;
use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct SomethingModel {
    pub id: i32,
    pub name: String,
    pub number: i32,
    pub alias: String,
    pub href: String,
}

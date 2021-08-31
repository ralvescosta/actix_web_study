use std::fmt::Display;

use diesel::Queryable;
use serde::Serialize;

#[derive(Serialize, Queryable, Debug)]
pub struct SomethingModel {
    pub id: i32,
    pub name: String,
    pub number: i32,
    pub alias: String,
    pub href: String,
}

impl Display for SomethingModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

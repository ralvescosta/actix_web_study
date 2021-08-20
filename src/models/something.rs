use serde::Deserialize;

#[derive(Deserialize)]
pub struct SomethingModel {
    pub name: String,
    pub number: i32,
    pub alias: String,
    pub href: String,
}

use crate::schema::somethings;
use diesel::Insertable;

#[derive(Insertable)]
#[table_name = "somethings"]
pub struct NewSomething<'a> {
    pub name: &'a str,
    pub number: i32,
    pub alias: &'a str,
    pub href: &'a str,
}

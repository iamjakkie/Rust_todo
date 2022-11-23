use diesel::prelude::*;

#[derive(Queryable)]
pub struct Todo {
    pub id: i32,
    pub name: String,
    pub description: String,
}
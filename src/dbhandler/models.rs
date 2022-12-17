use diesel::prelude::*;
use crate::dbhandler::schema::todos;

#[derive(Queryable)]
pub struct Todo {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo<'a> {
    pub name: &'a str,
    pub description: &'a str,
}
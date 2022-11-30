use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::{Todo, NewTodo};

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn show_todos() {
    use self::schema::todos::dsl::*;

    let connection = &mut establish_connection();
    let results = todos
        .limit(5)
        .load::<Todo>(connection)
        .expect("Error loading posts");

    println!("Displaying {} todos", results.len());
    println!("Id\tName:Description");
    println!("=========================");
    for todo in results {
        print!("{}\t", todo.id);
        print!("{}:", todo.name);
        print!("{}\n", todo.description);
    }
}

pub fn create_todo(name: &str, description: &str) -> Todo {
    use crate::dbhandler::schema::todos;

    let new_todo = NewTodo { name, description };

    let connection = &mut establish_connection();


    diesel::insert_into(todos::table)
        .values(&new_todo)
        .get_result(connection)
        .expect("Error saving new post")
}

pub fn delete_todo(del_id: i32) {
    use self::schema::todos::dsl::*;

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(todos.filter(id.eq(&del_id)))
        .execute(connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::*;

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
    for todo in results {
        println!("{}", todo.name);
        println!("-----------\n");
        println!("{}", todo.description);
    }
}
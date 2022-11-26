mod todo;
mod dbhandler;

use std::time::{SystemTime};
use std::io::Write;
use crate::dbhandler::models::Todo;
use crate::dbhandler::{show_todos, create_todo, delete_todo};

fn prompt(name:&str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");

    return line.trim().to_string()
}



fn main() {
    loop {
        let input=prompt("> ");
        match input.as_str() {
            "help" => { print!("This is a console app to create and view todos. \
                                Available commands:\n");}
            "show" => { show_todos(); }
            "add" => {
                print!("Insert todo's name");
                let name = prompt(": ");
                print!("Insert description");
                let desc = prompt(": ");

                let todo = create_todo(&*name, &*desc);
            }
            "remove" => {
                print!("Insert todo's id");
                let id:i32 = prompt(": ").parse().unwrap();
                delete_todo(id);
            }
            "exit" => { break; }
            _ => {println!("Wrong input");}
        }
    }
}
mod todo;
mod dbhandler;

use std::time::{SystemTime};
use std::io::Write;
use eframe::{App, Frame, NativeOptions, run_native};
use eframe::egui::{CentralPanel, Context};


use crate::dbhandler::models::Todo;
use crate::dbhandler::{show_todos, create_todo, delete_todo, get_todos};

fn prompt(name:&str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");

    return line.trim().to_string()
}
#[derive(Default)]
struct Todos{
    todos: Vec<Todo>
}

impl Todos {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            todos: get_todos()
        }
    }
}

impl App for Todos {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            for todo in &self.todos{
                ui.label(&todo.id.to_string());
                ui.label(&todo.name);
                ui.label(&todo.description);
            }
        });
    }
}

fn main() {
    let native_options = NativeOptions::default();
    run_native("Todos", native_options, Box::new(|cc| Box::new(Todos::new(cc))));

    // loop {
    //     let input=prompt("> ");
    //     match input.as_str() {
    //         "help" => { print!("This is a console app to create and view todos. \
    //                             Available commands:\n");}
    //         "show" => { show_todos(); }
    //         "add" => {
    //             print!("Insert todo's name");
    //             let name = prompt(": ");
    //             print!("Insert description");
    //             let desc = prompt(": ");
    //
    //             let todo = create_todo(&*name, &*desc);
    //         }
    //         "remove" => {
    //             print!("Insert todo's id");
    //             let id:i32 = prompt(": ").parse().unwrap();
    //             delete_todo(id);
    //         }
    //         "exit" => { break; }
    //         _ => {println!("Wrong input");}
    //     }
    // }
}
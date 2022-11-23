mod todo;

use std::time::{SystemTime};
use std::io::Write;

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
            "show" => { }
            "add" =>
            "exit" => { break; }
            _ => {println!("Wrong input");}
        }
    }
}
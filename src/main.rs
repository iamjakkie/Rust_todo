mod todo;
mod dbhandler;

use std::time::{SystemTime};
use std::io::Write;
use eframe::{App, egui, Frame, NativeOptions, run_native};
use eframe::egui::{Button, CentralPanel, Color32, Context, Hyperlink, ScrollArea, Separator, TextEdit, TopBottomPanel};
use eframe::egui::widgets::Label;

use crate::dbhandler::models::Todo;
use crate::dbhandler::{show_todos, create_todo, delete_todo, get_todos};

const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);

fn prompt(name:&str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");

    return line.trim().to_string()
}
#[derive(Default)]
struct Todos{
    todos: Vec<Todo>,
    window_show: bool
}

impl Todos {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            todos: get_todos(),
            window_show: false
        }
    }

    fn render_todo_cards(&self, ui: &mut eframe::egui::Ui) {
        for todo in &self.todos{
            //name
            ui.add_space(PADDING);
            let title: String = format!("{} -> {}",todo.id, todo.name);
            ui.colored_label(WHITE, title);
            //desc
            ui.add_space(PADDING);
            // let label = eframe::egui::widgets::Label::new(&todo.description).text_layout(Button)
            ui.label(&todo.description);
        }
    }

    fn render_footer(&self, ui: &mut eframe::egui::Ui, ctx: &Context) {
        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.);
                ui.add(Label::new("Jakub Kielbasiewicz"));
                ui.add(Hyperlink::new("https://github.com/iamjakkie"));
                ui.add_space(10.);
            })
        });
    }
}

impl App for Todos {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        if self.window_show {
            egui::Window::new("Some modal")
                .show(ctx, |ui| {
                    let mut title:String = String::new();
                    let mut desc:&str = "";
                    ui.label("Inside");
                    ui.horizontal(|ui| {
                        ui.label("Todo title: ");
                        ui.add(TextEdit::singleline(&mut title).hint_text("Input title"))
                    });
                    ui.horizontal(|ui| {
                        ui.label("Todo description: ");
                        ui.add(TextEdit::singleline(&mut title).hint_text("Input title"))
                    });

                    if ui.button("Exit").clicked() {
                        if !title.is_empty() & !desc.is_empty() {
                            println!("{}, {}", title,desc)

                        }
                        self.window_show=false;
                    }
                });
        }
        CentralPanel::default().show(ctx, |ui| {
            render_header(ui);
            if ui.button("Add new todo").clicked() {
                println!("click");
                self.window_show = true;
            }
            let sep = Separator::default().spacing(20.);
            ui.add(sep);
            ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                self.render_todo_cards(ui);
            });
            self.render_footer(ui, ctx);
        });

    }
}

fn render_header(ui: &mut eframe::egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Todo list");
    });
    ui.add_space(PADDING);
    let sep = Separator::default().spacing(20.);
    ui.add(sep);
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
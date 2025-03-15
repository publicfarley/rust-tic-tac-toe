mod game_model;
mod cli;
mod gui;

use crate::cli::text_main;
use crate::gui::gui_main;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let usage_string = "Usage: cargo run -- {text|gui}";

    // The first argument (args[0]) is always the program name
    // Actual arguments start from args[1]
    if args.len() <= 1 {
        println!("{usage_string}");
        return;
    }

    let mode = args[1].as_str();
    match mode {
        "text" => text_main(),
        "gui" => gui_main(),
        _ => {
            println!("Wrong mode. Mode must be 'text' or 'gui', but got '{mode}` instead.");
            println!("{usage_string}");
        }
    }
}

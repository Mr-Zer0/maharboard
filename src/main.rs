use dialoguer::Select;

use std::io::{self, Write};

pub mod burmese;
pub mod gregorian;

fn main() {
    println!("********************");
    println!("*    Maharboard    *");
    println!("********************\n\n");

    let name = get_name();

    println!("Mingalar par {name}!\n");

    let calendar_types = vec!["Burmese", "Gregorian"];

    let calendar = Select::new()
        .with_prompt("Which calendar do you prefer?")
        .items(&calendar_types)
        .default(0)
        .interact()
        .unwrap();

    if calendar == 0 {
        burmese::new();
    } else {
        gregorian::new();
    }
}

/// Get the user's name just for greeting purposes
fn get_name() -> String {
    print!("Let me know your name: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Cannot read user input");
    name.trim().to_string()
}

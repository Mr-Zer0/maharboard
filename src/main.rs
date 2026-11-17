use std::io::{self, Write};

pub mod burmese;

fn main() {
    println!("********************");
    println!("*    Maharboard    *");
    println!("********************\n\n");

    let name = get_name();

    println!("Mingalar par {name}!\n");

    burmese::new();
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

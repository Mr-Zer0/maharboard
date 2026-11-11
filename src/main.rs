use std::io::{self, Write};

fn main() {
    println!("*******************");
    println!("*    Mahaboard    *");
    println!("*******************\n\n\n");

    let name = get_name();

    println!("Mingalar par {name}!");

    let birth_year = get_birth_year();

    println!("You were born in {birth_year}.");
}

fn get_name () -> String {
    print!("Let me know your name: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Cannot read user input");
    name.trim().to_string()
}

fn get_birth_year() -> u16 {
    loop {
        print!("Please, enter your Burmese birth year: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut year = String::new();
        io::stdin().read_line(&mut year).expect("Cannot read user input");

        let year: u16 = match year.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        match year {
            1200..=1388 => {
                return year;
            },
            _ => println!("Burmese birth year must be between 1200 and 1388. Please try again.")
        }
    }
}
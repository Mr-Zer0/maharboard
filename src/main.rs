use dialoguer::Select;

use std::io::{self, Write};

// const PHWAR: [&str; 7] = ["Binga", "Marana", "Ah Htun", "Thike", "Razar", "Ahdipati", "Puti"];

fn main() {
    println!("*******************");
    println!("*    Mahaboard    *");
    println!("*******************\n\n");

    let name = get_name();

    println!("Mingalar par {name}!\n");

    let birth_year = get_birth_year();

    println!("You were born in {birth_year}.\n");

    get_weekday();

    calculate(birth_year);
}

fn calculate(birth_year: u16) {
    let remainder = birth_year % 7;

    println!("Your remainder is: {remainder}");

    let mut v = vec![1, 4, 0, 3, 6, 2, 5];

    let index = v.iter().position(|&x| x == remainder).unwrap();

    println!("The index: {index}");

    v.rotate_left(index);

    println!("The rotated vector: {:?}", v);
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
            _ => println!("Burmese birth year must be between 1200 and 1388. Please try again.\n")
        }
    }
}

fn get_weekday() -> u8 {
    let weekdays = vec!["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];

    let day = Select::new()
        .with_prompt("Select your birth weekday")
        .items(&weekdays)
        .default(0)
        .interact()
        .unwrap();

    println!("You are {}-born.\n", weekdays[day]);

    day as u8
}

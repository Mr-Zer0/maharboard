use dialoguer::Select;

use std::io::{self, Write};

const PHWAR: [&str; 7] = [
    "Binga", "Marana", "Ah Htun", "Thike", "Razar", "Ahdipati", "Puti",
];

pub fn new() {
    println!("You are using burmese calendar!");

    let birth_year = get_birth_year();

    println!("You were born in {birth_year}.\n");

    let birth_weekday = get_weekday();

    calculate(birth_year, birth_weekday);
}

/// Get the Burmese birth year from the user
fn get_birth_year() -> u16 {
    loop {
        print!("Please, enter your Burmese birth year: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut year = String::new();
        io::stdin()
            .read_line(&mut year)
            .expect("Cannot read user input");

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
            }
            _ => println!("Burmese birth year must be between 1200 and 1388. Please try again.\n"),
        }
    }
}

/// Get the day of week the user was born on
/// Saturday = 0, Sunday = 1, Monday = 2, Tuesday = 3, Wednesday = 4, Thursday = 5, Friday = 6
/// Starting from Saturday is to align with Burmese calendar and remainder as index
fn get_weekday() -> u8 {
    let weekdays = vec![
        "Saturday",
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
    ];

    let day = Select::new()
        .with_prompt("Select your birth weekday")
        .items(&weekdays)
        .default(0)
        .interact()
        .unwrap();

    println!("You are {}-born.\n", weekdays[day]);

    day as u8
}

/// Calculate the Mahaboard sign based on the Burmese birth year and weekday
fn calculate(birth_year: u16, birth_weekday: u8) {
    let remainder = birth_year % 7;

    println!("Your remainder is: {remainder}");

    let mut v = vec![1, 4, 0, 3, 6, 2, 5];
    let index = v.iter().position(|&x| x == remainder).unwrap();

    v.rotate_left(index);

    let index = v.iter().position(|&x| x == birth_weekday as u16).unwrap();

    println!("Your Mahaboard sign is: {}", PHWAR[index]);
}

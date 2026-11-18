use chrono::{Datelike, NaiveDate};
use regex::regex;
use std::io::{self, Write};

const PHWAR: [&str; 7] = [
    "Binga", "Marana", "Ah Htun", "Thike", "Razar", "Ahdipati", "Puti",
];

pub fn new() {
    println!("You are using gregorian calendar!");

    let birthdate = get_birthdate();

    println!(
        "You were born on {}\n",
        &birthdate.format("%A, %-d %B, %C%y")
    );

    let index = calculate(&birthdate);

    println!("Your Maharboard sign is : {}", PHWAR[index]);
}

fn get_birthdate() -> NaiveDate {
    let mut birthdate = String::new();

    loop {
        birthdate.clear();

        print!("Please enter your birthday in YYYY/MM/DD format: ");
        io::stdout().flush().expect("Failed to flush stdout.");

        io::stdin()
            .read_line(&mut birthdate)
            .expect("Cannot read user input");

        if regex!(r"\d{4}\/\d{1,2}\/\d{1,2}").is_match(&birthdate) {
            break;
        } else {
            println!("Please type a valid date format.\n");
        }
    }

    let dateslice: Vec<&str> = birthdate.trim().split("/").collect();

    let year: i32 = dateslice[0]
        .trim()
        .parse()
        .expect("Year is not a valid input");
    let month: u32 = dateslice[1]
        .trim()
        .parse()
        .expect("Month is not a valid input");
    let day: u32 = dateslice[2]
        .trim()
        .parse()
        .expect("Month is not a valid input");

    let naive_date = NaiveDate::from_ymd_opt(year, month, day).unwrap();

    println!(
        "Days from Sunday: {}",
        &naive_date.weekday().number_from_sunday()
    );

    return naive_date;
}

fn calculate(birthdate: &NaiveDate) -> usize {
    let remainder = get_burmese_year(birthdate) % 7;
    let mut birth_weekday = birthdate.weekday().number_from_sunday();

    if birth_weekday == 7 {
        birth_weekday = 0
    }

    println!("Your remainder is: {remainder}");

    let mut v = vec![1, 4, 0, 3, 6, 2, 5];
    let index = v.iter().position(|&x| x == remainder).unwrap();

    v.rotate_left(index);

    v.iter().position(|&x| x == birth_weekday as usize).unwrap()
}

fn get_burmese_year(birthdate: &NaiveDate) -> usize {
    let gap = if birthdate.month() < 4 { 639 } else { 638 };

    (birthdate.year() - gap) as usize
}

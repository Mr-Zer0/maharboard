use chrono::{NaiveDate};

// Gregorian calendar date to Julian day number
fn grego_to_jdn(year: i32, month: i32, day: i32) -> i32 {
    let gdate = NaiveDate::from_ymd_opt(year, month as u32, day as u32).expect("Invalid date!");

    println!("The formatted date is : {}", gdate);

    let a: i32 = (14 - month)/12;
    let y: i32 = year + 4800 - a;
    let mut m: i32 = month + (12 * a) - 3;
    m = (153 * m) + 2;

    let jdn = day + (m / 5) + (365 * y) + (y / 4) - (y / 100) + (y / 400) - 32045;

    return jdn;
}

fn greeting() {
    println!("*******************");
    println!("*    Mahaboard    *");
    println!("*******************\n\n");
}

fn main() {
    greeting();

    let jdn = grego_to_jdn(1991, 2, 12);

    println!("Julian date number is {jdn}");
}

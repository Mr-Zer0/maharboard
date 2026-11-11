use chrono::{NaiveDate};

const SOLAR_YEAR: f32 = 365.2587565;
const LUNAR_MONTH: f32 = 29.5305879;
const MYANMAR_ERA: f32 = 1954168.050623;


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

fn get_myanmar_year(jdn: i32) -> i32 {
    let myanmar_year = ((jdn as f32 - 0.5 - MYANMAR_ERA) as f32 / SOLAR_YEAR).floor() as i32;

    return myanmar_year;
}

// ဝါထပ်နှစ် တွက်ချက်ခြင်း (ဝါကြီးထပ်၊ ဝါလေးထပ်)
fn is_wahtat(myanmar_year: i32) -> bool {
    let mut excess_days = (SOLAR_YEAR * (myanmar_year as f32 + 3739.0)) % LUNAR_MONTH;
    let ta = ((SOLAR_YEAR / 12.0) - LUNAR_MONTH) * (12.0 - 8.0);
    excess_days = if excess_days < ta { excess_days + LUNAR_MONTH } else { excess_days };
    let tw = LUNAR_MONTH - ((SOLAR_YEAR / 12.0) - LUNAR_MONTH) * 8.0;

    let waso_fullmoon = (SOLAR_YEAR * myanmar_year as f32 + MYANMAR_ERA - excess_days + 4.5 * LUNAR_MONTH - 0.5).round();

    println!("Waso fullmoon is {waso_fullmoon}");

    return excess_days >= tw;
}

fn greeting() {
    println!("*******************");
    println!("*    Mahaboard    *");
    println!("*******************\n\n");
}

fn main() {
    greeting();

    let jdn = grego_to_jdn(1991, 2, 12);
    let myanmar_year = get_myanmar_year(jdn);

    println!("Julian date number is {jdn}");
    println!("Myanmar year is {myanmar_year}");

    let wahtat = is_wahtat(myanmar_year);

    println!("Is Wahtat year? {}", if wahtat { "Yes" } else { "No" });
}

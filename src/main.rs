use std::io; // Input manager
use std::fs; // Enables modifying
// use std::thread::sleep;
use chrono::{Datelike, Timelike, Local};

fn main() {
    let events_file_path = "/home/maxthesyntax/code/rust/Calendar/src/events.json";
    let events = fs::read_to_string(events_file_path)
        .expect("Unable to read file");
    let time = Local::now();
    let month_names: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let month_name = (time.month() - 1) as usize;   
    let days_in_month = match time.month() {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        _ => 30,
    }; 

    print_date_and_time();
    println!("   {}", month_names[month_name]);
}

fn print_date_and_time() {
    let time = Local::now();

    println!(
        "Today is {:02}:{:02}:{:02}, {:02}.{:02}.{} on a {}.",
        time.hour(),
        time.minute(),
        time.second(),
        time.day(),
        time.month(),
        time.year(),
        time.weekday()
    );
}
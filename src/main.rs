use std::io; // Input manager
use std::fs; // Enables modifying
use std::str;
// use std::thread::sleep;
use chrono::{Datelike, Timelike, Local};
use colored::Colorize;

fn main() {
    let events_file_path = "/home/maxthesyntax/code/rust/Calendar/src/events.json";
    let events = fs::read_to_string(events_file_path)
        .expect("Unable to read file");
    let time = Local::now();
    let month_names: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let month_name = (time.month() - 1) as usize;   
    let days_in_month = match time.month() {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        2 => 28,
        _ => 30,
    }; 

    print_date_and_time();

    println!("   {}", month_names[month_name]);
    let mut days_to_print = days_in_month as usize;
    let mut days_to_print_in_ln = 7;
    let days_to_display: [&str; 32] = [" 0", "31", "30", "29", "28", "27", "26", "25", "24", "23", "22", "21", "20", "19", "18", "17", "16", "15", "14", "13", "12", "11", "10", " 9", " 8", " 7", " 6", " 5", " 4", " 3", " 2", " 1"];
    while days_to_print > 0 {
        if days_to_print_in_ln == 0 {
            println!("");
            days_to_print_in_ln = 7;
        }
            else {
                if str::parse(days_to_display[days_to_print]) == Ok((time.day() as usize)) {
                    print!("{} ", days_to_display[days_to_print].green().bold());
                }
                    else {
                        print!("{} ", days_to_display[days_to_print]);
                    }
                days_to_print = days_to_print - 1;
                days_to_print_in_ln = days_to_print_in_ln - 1;
            }
    }
    println!("");
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
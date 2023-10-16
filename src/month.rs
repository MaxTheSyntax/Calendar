use std::str;
use chrono::{Datelike, Local};
use colored::Colorize;

pub fn main() {
    //let events_file_path = "/home/maxthesyntax/code/rust/Calendar/src/events.json";
    //let events = fs::read_to_string(events_file_path)
    //    .expect("Unable to read file");
    let time = Local::now();
    let month_names: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let month_name = (time.month() - 1) as usize;   
    let days_in_month = match time.month() {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        2 => 28,
        _ => 30,
    }; 

    println!("   {}", month_names[month_name]);
    let mut days_to_print_in_ln = 7;
    let mut current_day = 1;
    while days_in_month as usize >= current_day {
        if days_to_print_in_ln == 0 {
            println!("");
            days_to_print_in_ln = 7;
        }
            else {
                if current_day == time.day() as usize {
                    print!("{:>2} ", current_day.to_string().green().bold());
                }
                    else {
                        print!("{:>2} ", current_day);
                    }
                days_to_print_in_ln = days_to_print_in_ln - 1;
                current_day = current_day + 1;
            }
    }
    println!();
}
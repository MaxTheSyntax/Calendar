use chrono::{Datelike, Timelike, Local};
use std::io::stdin;

mod month;
mod events;

pub fn main() {
    let mut input_string;
    let mut first_time: bool = true;

    println!();
    println!(r"   _____      _                _            ");
    println!(r"  / ____|    | |              | |           ");
    println!(r" | |     __ _| | ___ _ __   __| | __ _ _ __ ");
    println!(r" | |    / _` | |/ _ \ '_ \ / _` |/ _` | '__|");
    println!(r" | |___| (_| | |  __/ | | | (_| | (_| | |   ");
    println!(r"  \_____\__,_|_|\___|_| |_|\__,_|\__,_|_|   ");
    loop {
    println!();
    print_date_and_time();
    println!();
    if first_time == true {
        println!("What should I do?");
        first_time = false;
    }   else {
        println!("What's next?")
    }
    input_string = String::new();
    stdin().read_line(&mut input_string)
    	.ok()
        .expect("Failed to read line");
    println!();
    if input_string.trim().to_lowercase() == "month" {
        month::main();
    }   else if input_string.trim().to_lowercase() == "events" {
        events::main();
    }
    }
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
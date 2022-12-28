use std::io; // Input manager
use std::fs; // Enables modifying
// use std::thread::sleep;
use chrono::{Datelike, Timelike, Local};

fn main() {
    let current_time = Local::now();
    let events_file_path = "/home/maxthesyntax/code/rust/Calendar/src/events.json";
    let events = fs::read_to_string(events_file_path)
        .expect("Unable to read file");


    println!(
        "Today is {:02}:{:02}:{:02}, {:02}.{:02}.{} on a {}.",
        current_time.hour(),
        current_time.minute(),
        current_time.second(),
        current_time.day(),
        current_time.month(),
        current_time.year(),
        current_time.weekday()
    );
}

fn data() {
    let parsed = json::parse(r#"
    {
        "name": "John Doe",
        "age": 43,
        "address": {
            "street": "10 Downing Street",
            "city": "London"
        },
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    }
    "#).unwrap();

    println!("{:#}", parsed["name"] );


}
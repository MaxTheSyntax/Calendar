use std::io; // Input manager
use std::time::{Duration, SystemTime};
use std::thread::sleep; 
use chrono::{Datelike, Timelike, Local};

fn main() {
    let now = Local::now();

    println!(
        "The current local time is {:02}:{:02}:{:02}, {:02}.{:02}.{} on a {}.",
        now.hour(),
        now.minute(),
        now.second(),
        now.day(),
        now.month(),
        now.year(),
        now.weekday()
    );
}
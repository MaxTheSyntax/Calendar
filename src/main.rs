use chrono::{Datelike, Timelike, Local};

mod month;

fn main() {
    println!();
    println!(r"   _____      _                _            ");
    println!(r"  / ____|    | |              | |           ");
    println!(r" | |     __ _| | ___ _ __   __| | __ _ _ __ ");
    println!(r" | |    / _` | |/ _ \ '_ \ / _` |/ _` | '__|");
    println!(r" | |___| (_| | |  __/ | | | (_| | (_| | |   ");
    println!(r"  \_____\__,_|_|\___|_| |_|\__,_|\__,_|_|   ");
    println!();
    print_date_and_time();
    println!();
    month::main();
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
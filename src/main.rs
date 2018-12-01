//extern crate advent_of_code;

use std::env;
mod day1;

fn main() {
    let mut args = env::args();
    args.next();

    let mut filters = Vec::new();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            arg => filters.push(arg.to_string()),
        }
    }

    let filters: Vec<&str> = filters.iter().map(|s| s.as_str()).collect();

    for filter in &filters {
        run_day(filter);
    }
}

fn run_day(day: &str) {
    match day {
        "day1" => {
            println!("Day 1");
            let result = day1::run();
            println!("Puzzle 1: {}", result);
            let result = day1::run2();
            println!("Puzzle 2: {}", result);
        },
        _ => {
            println!("No Matching Day!");
        }
    }
}
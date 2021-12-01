#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    rust_2018_idioms
)]
#![windows_subsystem = "console"]
#![doc = include_str!("../README.md")]

mod day_1;
mod util;

fn main() {
    // Day 1
    let input_day1 = split_parse!("day_1/input.txt", char::is_whitespace, usize);
    println!("{}", day_1::sonar_sweep_1(&input_day1));
    println!("{}", day_1::sonar_sweep_2(&input_day1));
}

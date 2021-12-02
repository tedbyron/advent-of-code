#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    rust_2018_idioms
)]
#![allow(dead_code)]
#![windows_subsystem = "console"]
#![doc = include_str!("../README.md")]

mod day_1;
mod day_2;

fn main() {
    // Day 1
    // let input_day1 = day_1::collect_input(include_str!("day_1/input.txt"));
    // println!("{}", day_1::sonar_sweep_1(&input_day1)); // 1215
    // println!("{}", day_1::sonar_sweep_2(&input_day1)); // 1150

    // Day 2
    let input_day2 = day_2::collect_input(include_str!("day_2/input.txt"));
    println!("{}", day_2::dive(&input_day2, day_2::Submarine::default())); // 1694130
    println!("{}", day_2::dive(&input_day2, day_2::Submarine::with_aim())); // 1698850445
}

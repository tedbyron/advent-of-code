#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    rust_2018_idioms
)]
#![allow(dead_code)]
#![windows_subsystem = "console"]

mod day_1;

fn main() {
    // println!("{}", day_1::sonar_sweep_1());
    println!("{}", day_1::sonar_sweep_2());
}

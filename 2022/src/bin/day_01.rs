#![warn(clippy::all, clippy::nursery, rust_2018_idioms)]

fn main() {
    let mut elves = include_str!("../inputs/day_01.txt")
        .split("\n\n")
        .map(|elf| elf.split('\n').flat_map(str::parse::<i32>).sum::<i32>())
        .collect::<Vec<_>>();
    elves.select_nth_unstable_by(2, |a, b| b.cmp(a));

    println!("{}", elves[0]);
    println!("{}", elves[..3].iter().sum::<i32>());
}

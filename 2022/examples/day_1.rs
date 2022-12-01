#![warn(clippy::all, clippy::nursery, rust_2018_idioms)]

fn main() {
    let input = include_str!("day_1.txt");
    let mut elves = input
        .split("\n\n")
        .map(|elf| elf.split('\n').flat_map(str::parse::<i32>).sum::<i32>())
        .collect::<Vec<_>>();
    elves.sort_unstable_by(|a, b| b.cmp(a));

    println!("{}", elves[0]);
    println!("{}", elves[0..3].iter().sum::<i32>());
}

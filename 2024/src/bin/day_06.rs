use std::collections::HashSet;

fn main() {
    let input = advent_of_code_2024::read_input!();
    let start_idx = input.chars().position(|c| c == '^').unwrap();
    let input = input
        .lines()
        .map(|l| l.chars().collect::<Box<[_]>>())
        .collect::<Box<_>>();

    let p1 = HashSet::<(usize, usize)>::new();
    let (mut x, mut y) = (start_idx % input[0].len(), start_idx / input[0].len());
    println!("{} {}", x, y);
    // while (0..input.len()).contains(&y) && (0..input[0].len()).contains(&x) {}

    // println!("part 1: {}", p1.len());
}

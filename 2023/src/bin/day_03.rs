#![feature(let_chains)]

use advent_of_code_2023::read_input;

fn main() -> anyhow::Result<()> {
    #[rustfmt::skip]
    let deltas = [
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];
    let input = read_input(file!())?;
    let input: Vec<Vec<char>> = input
        .lines()
        .map(str::chars)
        .map(Iterator::collect)
        .collect();
    let is_part_number = |i: usize, j: usize| {
        deltas.iter().any(|&(di, dj)| {
            if let Ok(i) = usize::try_from(i as i32 + di)
                && let Ok(j) = usize::try_from(j as i32 + dj)
                && let Some(row) = input.get(i)
                && let Some(c) = row.get(j)
            {
                !c.is_ascii_digit() && c != &'.'
            } else {
                false
            }
        })
    };

    let len = input[0].len();
    let a = input.iter().enumerate().fold(0, |sum, (i, line)| {
        let (line_sum, _) =
            line.iter()
                .enumerate()
                .fold((0, Vec::new()), |(mut line_sum, mut group), (j, c)| {
                    if c.is_ascii_digit() {
                        group.push((j, c));
                    }

                    if (!c.is_ascii_digit() || j == len - 1) && !group.is_empty() {
                        if group.iter().any(|&(j, _)| is_part_number(i, j)) {
                            line_sum += group
                                .iter()
                                .map(|(_, c)| c.to_digit(10).unwrap())
                                .fold(0, |acc, n| acc * 10 + n);
                        }

                        group = Vec::new();
                    }

                    (line_sum, group)
                });

        sum + line_sum
    });

    println!("part 1: {a}");
    // println!("part 2: {b}");

    Ok(())
}

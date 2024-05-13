use std::collections::HashMap;

use advent_of_code_2023::read_input;
use nom::{
    bytes::complete::tag,
    character::complete::alpha1,
    sequence::{delimited, terminated, tuple},
    IResult,
};

fn parse_line(s: &str) -> IResult<&str, (&str, &str, &str)> {
    let (s, name) = terminated(alpha1, tag(" = "))(s)?;
    let (s, (l, _, r)) = delimited(tag("("), tuple((alpha1, tag(", "), alpha1)), tag(")"))(s)?;
    Ok((s, (name, l, r)))
}

fn main() -> anyhow::Result<()> {
    let input = read_input!()?;
    let mut input = input.lines();
    let (instructions, nodes) = (
        input.next().unwrap().chars().collect::<Box<_>>(),
        &input
            .flat_map(parse_line)
            .map(|(_, (name, l, r))| (name, (l, r)))
            .collect::<HashMap<_, _>>(),
    );
    let (mut a, mut node) = (0, "AAA");
    while node != "ZZZ" {
        node = match instructions[a % instructions.len()] {
            'L' => nodes[node].0,
            'R' => nodes[node].1,
            _ => unreachable!(),
        };
        a += 1;
    }

    println!("{a}");

    Ok(())
}

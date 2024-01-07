use advent_of_code_2023::read_input;
use itertools::Itertools;
use rayon::prelude::*;

fn main() -> anyhow::Result<()> {
    let input = read_input(file!())?;
    let mut input = input.split("\n\n");
    let seeds = input
        .next()
        .map(|s| {
            s.split(':')
                .last()
                .map(|s| s.split(' ').flat_map(str::parse::<i64>))
                .unwrap()
                .collect::<Vec<_>>()
        })
        .unwrap();
    let maps = input
        .map(|s| {
            s.lines()
                .skip(1)
                .filter_map(|l| {
                    let (destination, source, len) =
                        l.split(' ').flat_map(str::parse::<i64>).next_tuple()?;
                    Some((source..source + len, destination - source))
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let in_range = |seed| {
        maps.iter().fold(seed, |seed, map| {
            if let Some((_, delta)) = map.iter().find(|(source, _)| source.contains(&seed)) {
                seed + delta
            } else {
                seed
            }
        })
    };
    let a = seeds.par_iter().copied().map(in_range).min().unwrap();
    let seeds = seeds
        .into_iter()
        .tuples()
        .map(|(a, b)| a..a + b)
        .collect::<Vec<_>>();
    let b = seeds
        .into_par_iter()
        .flat_map(|range| range.into_iter())
        .map(in_range)
        .min()
        .unwrap();

    println!("{a}");
    println!("{b}");

    Ok(())
}

use std::{cmp::Ordering, collections::HashMap};

use advent_of_code_2023::read_input;
use itertools::Itertools;

struct ParsedHand {
    values: Box<[usize]>,
    counts: HashMap<char, usize>,
}

fn parse_hand(hand: &str) -> ParsedHand {
    ParsedHand {
        values: hand
            .chars()
            .filter_map(|c| "23456789TJQKA".find(c))
            .collect(),
        counts: hand.chars().counts(),
    }
}

fn hand_value(counts: &HashMap<char, usize>) -> usize {
    match &*counts.values().sorted_unstable().join("") {
        "5" => 6,     // Five of a kind
        "14" => 5,    // Four of a kind
        "23" => 4,    // Full house
        "113" => 3,   // Three of a kind
        "122" => 2,   // Two Pair
        "1112" => 1,  // One Pair
        "11111" => 0, // High Card
        _ => unreachable!(),
    }
}

fn cmp(a: &ParsedHand, b: &ParsedHand) -> Ordering {
    match hand_value(&a.counts).cmp(&hand_value(&b.counts)) {
        Ordering::Equal => {
            for i in 0..5 {
                match a.values[i].cmp(&b.values[i]) {
                    Ordering::Equal => continue,
                    ord => return ord,
                }
            }
            unreachable!();
        }
        ord => ord,
    }
}

fn main() -> anyhow::Result<()> {
    let mut input = read_input!()?
        .lines()
        .filter_map(|l| {
            let (hand, bid) = l.split_ascii_whitespace().collect_tuple::<(_, _)>()?;
            let parsed_hand = parse_hand(hand);
            Some((parsed_hand, bid.parse::<usize>().ok()?))
        })
        .collect::<Box<_>>();

    input.sort_unstable_by(|a, b| cmp(&a.0, &b.0));
    let a = input
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum::<usize>();

    println!("{a}");
    // println!("{b}");

    Ok(())
}

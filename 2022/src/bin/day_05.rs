use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day_05.txt")?;
    let (crates, procedure) = input.split_once("\n\n").unwrap();

    #[allow(clippy::needless_collect)]
    let crates = crates
        .lines()
        .take(crates.len() - 1)
        .map(|line| {
            line.chars()
                .enumerate()
                .skip(1)
                .step_by(4)
                .filter_map(|(i, c)| {
                    if c.is_ascii_alphabetic() {
                        Some(((i + 3) / 4, c))
                    } else {
                        None
                    }
                })
        })
        .collect::<Vec<_>>();

    let mut crates1 = crates.into_iter().rfold(HashMap::new(), |mut map, line| {
        for (i, c) in line {
            map.entry(i).or_insert_with(Vec::new).push(c);
        }
        map
    });
    let mut crates2 = crates1.clone();

    let procedure = procedure.lines().map(|line| {
        line.split_ascii_whitespace()
            .flat_map(str::parse::<usize>)
            .collect::<Vec<_>>()
    });

    for instr in procedure {
        let entry = crates1.entry(instr[1]).or_default();
        let moved = entry
            .drain(entry.len() - instr[0]..)
            .rev()
            .collect::<Vec<_>>();
        crates1.entry(instr[2]).or_default().extend(moved);

        let entry = crates2.entry(instr[1]).or_default();
        let moved = entry.drain(entry.len() - instr[0]..).collect::<Vec<_>>();
        crates2.entry(instr[2]).or_default().extend(moved);
    }

    println!("{}", get_top(crates1));
    println!("{}", get_top(crates2));

    Ok(())
}

fn get_top(crates: HashMap<usize, Vec<char>>) -> String {
    let mut top_crates = crates
        .iter()
        .fold(Vec::new(), |mut crates, (&stack, items)| {
            crates.push((stack, items.last().unwrap()));
            crates
        });
    top_crates.sort_by(|(stack1, _), (stack2, _)| stack1.cmp(stack2));

    top_crates
        .into_iter()
        .fold(String::new(), |mut s, (_, item)| {
            s.push(*item);
            s
        })
}

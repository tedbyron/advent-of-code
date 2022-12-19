use std::collections::HashSet;
use std::ops::{Add, Sub};

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day_09.txt")?;
    let (mut h, mut t): ((i32, i32), (i32, i32)) = ((0, 0), (0, 0));
    let mut locations = HashSet::new();
    locations.insert(t);
    println!("{:?}, {:?}", h, t);

    for (dir, n) in input.lines().map(|l| l.split_once(' ').unwrap()) {
        let n = n.parse::<i32>()?;
        match dir {
            "U" => p1(&mut h.1, &mut t.1, i32::add, n),
            "D" => p1(&mut h.1, &mut t.1, i32::sub, n),
            "L" => p1(&mut h.0, &mut t.0, i32::sub, n),
            "R" => p1(&mut h.0, &mut t.0, i32::add, n),
            _ => unreachable!(),
        }
        println!("{:?}, {:?}", h, t);
        locations.insert(t);
    }
    println!("{}, {}", locations.len(), 3);
    Ok(())
}

fn p1<F>(h: &mut i32, t: &mut i32, op: F, n: i32)
where
    F: Fn(i32, i32) -> i32,
{
    *h += n;
    let diff: i32 = *t - *h;
    if diff.abs() > 1 {
        *t = op(*h, diff.signum());
    }
}

struct P1 {
    cycle_count: i32,
    signal_strength: i32,
    sum: i32,
}

impl P1 {
    fn cycle(&mut self) {
        self.cycle_count += 1;
        if [20, 60, 100, 140, 180, 220].contains(&self.cycle_count) {
            self.sum += self.cycle_count * self.signal_strength
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day_10.txt")?;
    let mut p1 = P1 {
        cycle_count: 0,
        signal_strength: 1,
        sum: 0,
    };

    for line in input
        .lines()
        .map(str::split_ascii_whitespace)
        .map(Iterator::collect::<Vec<_>>)
    {
        match *line {
            ["noop"] => p1.cycle(),
            ["addx", v] => {
                p1.cycle();
                p1.cycle();
                p1.signal_strength += v.parse::<i32>()?;
            }
            _ => unreachable!(),
        }
    }

    println!("{}, {}", p1.sum, p2);
    Ok(())
}

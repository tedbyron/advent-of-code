use advent_of_code_2023::input;

const NUMBERS: &[&str] = &[
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

fn main() -> anyhow::Result<()> {
    let input = input(file!())?;
    let (a, b) = input
        .lines()
        .filter_map(|l| {
            let digits1 = l.matches(char::is_numeric).collect::<Vec<_>>();
            let digits2 = NUMBERS.iter().map(|&n| l.matches(n).collect::<Vec<_>>());

            Some((
                format!("{}{}", digits1.first()?, digits1.last()?)
                    .parse::<i32>()
                    .ok()?,
                42,
            ))
        })
        .reduce(|(a, b), (l, r)| (a + l, b + r))
        .unwrap();

    println!("part 1: {a}");
    println!("part 2: {b}");

    Ok(())
}

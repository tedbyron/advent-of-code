//! Sonar sweep thingy.

fn depths() -> Vec<usize> {
    include_str!("input.txt")
        .split(char::is_whitespace)
        .filter_map(|s| s.parse().ok())
        .collect()
}

pub fn sonar_sweep_1() -> usize {
    depths().windows(2).filter(|w| w[0] < w[1]).count()
}

pub fn sonar_sweep_2() -> usize {
    depths()
        .windows(3)
        .zip(depths().windows(3).skip(1))
        .filter(|(a, b)| a.iter().sum::<usize>() < b.iter().sum())
        .count()
}

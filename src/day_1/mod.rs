//! [Day 1](https://adventofcode.com/2020/day/1): Sonar sweep thingy.

/// Count number of times that a depth increases from the previous measurement.
pub fn sonar_sweep_1(depths: &[usize]) -> usize {
    depths.windows(2).filter(|w| w[0] < w[1]).count()
}

/// Count the number of times the sum of a 3-measurement window increases from the previous sum of a
/// 3-measurement window.
pub fn sonar_sweep_2(depths: &[usize]) -> usize {
    let windows = depths.windows(3).map(|w| w.iter().sum());
    sonar_sweep_1(&windows.collect::<Vec<_>>())
}

// /// No extra allocations version of `sonar_sweep_2`.
// pub fn sonar_sweep_2(depths: &[usize]) -> usize {
//     depths
//         .windows(3)
//         .zip(depths.windows(3).skip(1))
//         .filter(|(a, b)| a.iter().sum::<usize>() < b.iter().sum())
//         .count()
// }

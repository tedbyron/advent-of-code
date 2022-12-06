use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let input = std::fs::read("inputs/day_06.txt")?;

    let unique_window_pos = |size: usize| {
        input
            .windows(size)
            .position(|w| HashSet::<&u8>::from_iter(w).len() == w.len())
            .unwrap()
            + size
    };

    println!("{}", unique_window_pos(4));
    println!("{}", unique_window_pos(14));

    Ok(())
}

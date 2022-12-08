use nalgebra::{DMatrix, Rotation};

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day_08.txt")?;
    let m = DMatrix::from_row_iterator(
        input.lines().count(),
        input.lines().next().unwrap().len(),
        input.lines().flat_map(str::bytes),
    );

    let view = |a| {};

    let (rows, cols) = m.shape();

    // println!("{} {}", p1, p2);
    Ok(())
}

fn to_string<'a>(m: impl Iterator<Item = &'a u8>) -> String {
    m.map(|&b| char::from(b)).collect()
}

use itertools::{izip, Itertools};

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day_08.txt")?;
    let trees = input.lines().flat_map(str::bytes).collect_vec();
    let (w, h) = (input.lines().next().unwrap().len(), input.lines().count());

    let solve = |x1: usize, xn, amount, x_step, y_step| {
        let mut result = vec![(0, false); trees.len()];
        let direction = if x1 < xn { 1 } else { -1 };
        let height = |xi, y| trees[(x1 + (direction * xi) as usize) * x_step + y * y_step];

        for y in 0..amount {
            for x in 0..(1 + x1.abs_diff(xn) as i64) {
                let shorter = (0..x)
                    .rev()
                    .take_while(|xj| height(*xj, y) < height(x, y))
                    .count();
                let hidden = x > shorter as i64;
                let x = x1 + (direction * x) as usize;
                result[x * x_step + y * y_step] = (shorter + hidden as usize, !hidden);
            }
        }
        result
    };

    let u = solve(h - 1, 0, w, w, 1);
    let d = solve(0, h - 1, w, w, 1);
    let l = solve(0, w - 1, h, 1, w);
    let r = solve(w - 1, 0, h, 1, w);
    let p1 = izip!(&u, &d, &l, &r).filter(|(u, d, l, r)| u.1 | d.1 | l.1 | r.1);
    let p2 = izip!(&u, &d, &l, &r).map(|(u, d, l, r)| u.0 | d.0 | l.0 | r.0);

    println!("{}, {}", p1.count(), p2.max().unwrap());
    Ok(())
}

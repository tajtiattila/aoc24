use crate::grid::{Grid, Point, STEPS};
use std::collections::HashMap;

pub fn run(input: &str) -> anyhow::Result<String> {
    let grid = Grid::parse(input)?;
    let (s1, s2) = stars(&grid);
    Ok(format!("{s1} {s2}"))
}

fn stars(grid: &Grid<u8>) -> (usize, usize) {
    grid.positions()
        .map(|p| trailhead_score(grid, p))
        .fold((0, 0), |(acc1, acc2), (n1, n2)| (acc1 + n1, acc2 + n2))
}

fn trailhead_score(grid: &Grid<u8>, p: Point) -> (usize, usize) {
    if grid.get(p) != Some(&b'0') {
        return (0, 0);
    }
    let mut work = vec![(p, b'0')];
    let mut peaks = HashMap::new();
    while let Some((p, h)) = work.pop() {
        let h = h + 1;
        for &s in STEPS.iter() {
            let q = p + s;
            if grid.get(q) == Some(&h) {
                if h == b'9' {
                    peaks.entry(q).and_modify(|c| *c += 1).or_insert(1);
                } else {
                    work.push((q, h));
                }
            }
        }
    }
    (peaks.len(), peaks.values().sum())
}

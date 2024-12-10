use crate::grid::{Grid, Point, STEPS};
use std::collections::HashSet;

pub fn run(input: &str) -> anyhow::Result<String> {
    let grid = Grid::parse(input)?;
    let s1 = star1(&grid);
    let s2 = "";
    Ok(format!("{s1} {s2}"))
}

fn star1(grid: &Grid<u8>) -> usize {
    grid.positions().map(|p| trailhead_score(grid, p)).sum()
}

fn trailhead_score(grid: &Grid<u8>, p: Point) -> usize {
    if grid.get(p) != Some(&b'0') {
        return 0;
    }
    let mut work = vec![(p, b'0')];
    let mut num_peaks = 0;
    let mut vis = HashSet::new();
    while let Some((p, h)) = work.pop() {
        let h = h + 1;
        for &s in STEPS.iter() {
            let q = p + s;
            if grid.get(q) == Some(&h) && vis.insert(q) {
                if h == b'9' {
                    num_peaks += 1;
                } else {
                    work.push((q, h));
                }
            }
        }
    }
    num_peaks
}

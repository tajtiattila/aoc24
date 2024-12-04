use crate::grid::{CellP, Grid};
use anyhow::Result;

pub fn run(input: &str) -> anyhow::Result<String> {
    let s1 = star1(input)?;
    let s2 = "";
    Ok(format!("{s1} {s2}"))
}

fn star1(input: &str) -> Result<usize> {
    let grid = Grid::parse(input)?;

    let dirs = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];

    Ok(grid
        .positions()
        .flat_map(|p| dirs.map(|d| (p, d)))
        .filter(|(p, d)| is_grid_string(&grid, *p, *d, "XMAS"))
        .count())
}

fn grid_dir_bytes(grid: &Grid<u8>, p: CellP, dir: CellP) -> impl Iterator<Item = u8> + use<'_> {
    std::iter::successors(Some(p), move |p| {
        let q = (p.0 + dir.0, p.1 + dir.1);
        grid.is_inside(q).then_some(q)
    })
    .filter_map(|p| grid.get(p))
    .fuse()
    .copied()
}

fn is_grid_string(grid: &Grid<u8>, p: CellP, dir: CellP, s: &str) -> bool {
    std::iter::zip(s.bytes(), grid_dir_bytes(grid, p, dir))
        .filter(|(x, y)| x == y)
        .count()
        == s.len()
}

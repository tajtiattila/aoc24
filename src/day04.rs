use crate::grid::{pt, Grid, Point};

pub fn run(input: &str) -> anyhow::Result<String> {
    let grid = Grid::parse(input)?;
    let s1 = star1(&grid);
    let s2 = star2(&grid);
    Ok(format!("{s1} {s2}"))
}

fn star1(grid: &Grid<u8>) -> usize {
    let dirs = [
        pt(-1, -1),
        pt(0, -1),
        pt(1, -1),
        pt(1, 0),
        pt(1, 1),
        pt(0, 1),
        pt(-1, 1),
        pt(-1, 0),
    ];

    grid.positions()
        .flat_map(|p| dirs.map(|d| (p, d)))
        .filter(|(p, d)| is_grid_string(grid, *p, *d, "XMAS"))
        .count()
}

fn star2(grid: &Grid<u8>) -> usize {
    grid.positions().filter(|p| is_x_mas(grid, *p)).count()
}

fn grid_dir_bytes(grid: &Grid<u8>, p: Point, dir: Point) -> impl Iterator<Item = u8> + use<'_> {
    std::iter::successors(Some(p), move |p| {
        let q = *p + dir;
        grid.is_inside(q).then_some(q)
    })
    .filter_map(|p| grid.get(p))
    .fuse()
    .copied()
}

fn is_grid_string(grid: &Grid<u8>, p: Point, dir: Point, s: &str) -> bool {
    std::iter::zip(s.bytes(), grid_dir_bytes(grid, p, dir))
        .filter(|(x, y)| x == y)
        .count()
        == s.len()
}

fn is_x_mas(grid: &Grid<u8>, p: Point) -> bool {
    if grid.get(p) != Some(&b'A') {
        return false;
    }

    let at = |dx, dy| -> Option<&u8> { grid.get(p + Point::new(dx, dy)) };

    let d1 = (at(-1, -1), at(1, 1));
    let d2 = (at(1, -1), at(-1, 1));
    (d1 == (Some(&b'M'), Some(&b'S')) || d1 == (Some(&b'S'), Some(&b'M')))
        && (d2 == (Some(&b'M'), Some(&b'S')) || d2 == (Some(&b'S'), Some(&b'M')))
}

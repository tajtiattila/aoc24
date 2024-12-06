use crate::grid::{Dir, Grid, Point};
use anyhow::{anyhow, Result};

pub fn run(input: &str) -> Result<String> {
    let grid = Grid::parse(input)?;
    let start = find_start(&grid).ok_or_else(|| anyhow!("no guard"))?;
    let (s1, s2) = stars(&grid, start, Dir::North);
    Ok(format!("{s1} {s2}"))
}

fn find_start(grid: &Grid<u8>) -> Option<Point> {
    for p in grid.positions() {
        if grid.get(p) == Some(&b'^') {
            return Some(p);
        }
    }
    return None;
}

fn stars(grid: &Grid<u8>, start: Point, dir: Dir) -> (usize, usize) {
    let (visited, exit_count, _) = walk(grid, start, dir, None);

    let mut num_obstacles = 0;
    for p in grid.positions() {
        if p == start || *visited.get(p).unwrap() == 0 {
            continue;
        }

        if !walk(grid, start, dir, Some(p)).2 {
            num_obstacles += 1;
        }
    }

    (exit_count, num_obstacles)
}

fn walk(
    grid: &Grid<u8>,
    start: Point,
    mut dir: Dir,
    obstacle_at: Option<Point>,
) -> (Grid<u8>, usize, bool) {
    let mut visited = Grid::<u8>::new(grid.dimensions(), 0);
    let mut count = 0;
    let mut p = start;
    loop {
        let cell = visited.get_mut(p).unwrap();
        if *cell == 0 {
            count += 1;
        }
        let m = 1 << dir.index();
        if *cell & m != 0 {
            // stuck in a loop
            return (visited, count, false);
        }
        *cell |= m;

        let mut q = p + dir.step(1);
        if !grid.is_inside(q) {
            return (visited, count, true);
        }

        while Some(q) == obstacle_at || *grid.get(q).unwrap() == b'#' {
            dir = dir.right();
            q = p + dir.step(1);
        }
        p = q;
    }
}

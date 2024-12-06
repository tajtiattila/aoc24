use crate::grid::{Dir, Grid, Point};
use anyhow::{anyhow, Result};

pub fn run(input: &str) -> Result<String> {
    let grid = Grid::parse(input)?;
    let start = find_start(&grid).ok_or_else(|| anyhow!("no guard"))?;
    let s1 = star1(&grid, start, Dir::North);
    let s2 = "";
    Ok(format!("{s1} {s2}"))
}

fn find_start(grid: &Grid<u8>) -> Option<Point> {
    for p in grid.positions() {
        if grid.get(p) == Some(&b'^') {
            return Some(p)
        }
    }
    return None
}

fn star1(grid: &Grid<u8>, start: Point, mut dir: Dir) -> usize {
    let mut visited = Grid::new(grid.dimensions(), b'.');
    let mut count = 0;
    let mut p = start;
    loop {
        let cell = visited.get_mut(p).unwrap();
        if *cell == b'.' {
            *cell = b'X';
            count += 1;
        }

        let mut q = p + dir.step(1);
        if !grid.is_inside(q) {
            break;
        }
        
        while *grid.get(q).unwrap() == b'#' {
            dir = dir.right();
            q = p + dir.step(1);
        }
        p = q;
    }
    count
}

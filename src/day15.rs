use crate::grid::{Grid, Point};
use anyhow::{anyhow, Result};
use std::iter::successors;

pub fn run(input: &str) -> Result<String> {
    let (maps, moves) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("input: missing separator"))?;
    let (grid, start) = parse_map(maps)?;
    let s1 = box_gps_locs(grid, start, moves);
    let s2 = "";
    Ok(format!("{s1} {s2}"))
}

fn box_gps_locs(mut grid: Grid<u8>, start: Point, moves: &str) -> usize {
    moves_iter(moves).fold(start, |p, step| step_robot(&mut grid, p, step));
    grid.positions()
        .filter(|p| grid.get(*p) == Some(&b'O'))
        .map(|Point { x, y }| (x + y * 100) as usize)
        .sum()
}

fn step_robot(grid: &mut Grid<u8>, p0: Point, step: Point) -> Point {
    let p1 = p0 + step;
    match grid.get(p1) {
        Some(&b'.') => p1,
        Some(&b'O') => {
            if try_push_boxes(grid, p1, step) {
                p1
            } else {
                p0
            }
        }
        _ => p0,
    }
}

fn try_push_boxes(grid: &mut Grid<u8>, p: Point, step: Point) -> bool {
    let r = successors(Some(p), |&p| Some(p + step))
        .map(|p| (p, grid.get(p)))
        .find(|(_, xc)| *xc != Some(&b'O'));
    if let Some((q, qc)) = r {
        if qc == Some(&b'.') {
            *grid.get_mut(q).unwrap() = b'O';
            *grid.get_mut(p).unwrap() = b'.';
            return true;
        }
    }
    false
}

fn parse_map(input: &str) -> Result<(Grid<u8>, Point)> {
    let mut grid = Grid::parse(input)?;
    let start = grid
        .positions()
        .find(|p| grid.get(*p) == Some(&b'@'))
        .ok_or_else(|| anyhow!("input: missing start"))?;
    *grid.get_mut(start).unwrap() = b'.';
    Ok((grid, start))
}

fn moves_iter(moves: &str) -> impl Iterator<Item = Point> + use<'_> {
    moves.chars().filter_map(|c| match c {
        '^' => Some(Point::new(0, -1)),
        '<' => Some(Point::new(-1, 0)),
        '>' => Some(Point::new(1, 0)),
        'v' => Some(Point::new(0, 1)),
        _ => None,
    })
}

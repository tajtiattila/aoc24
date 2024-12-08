use crate::grid::{Grid, Point};
use std::collections::{HashMap, HashSet};

pub fn run(input: &str) -> anyhow::Result<String> {
    let grid = Grid::parse(input)?;
    let s1 = star1(&grid);
    let s2 = "";
    Ok(format!("{s1} {s2}"))
}

fn star1(grid: &Grid<u8>) -> usize {
    let antennas = find_antennas(grid);
    let antinode_locs = antennas
        .values()
        .flat_map(|a| find_antinodes(a).filter(|p| grid.is_inside(*p)))
        .collect::<HashSet<Point>>();
    antinode_locs.len()
}

fn find_antennas(grid: &Grid<u8>) -> HashMap<u8, Vec<Point>> {
    grid.positions()
        .filter_map(|p| {
            let c = *grid.get(p)?;
            (c as char).is_ascii_alphanumeric().then_some((c, p))
        })
        .fold(HashMap::new(), |mut acc, (c, p)| {
            acc.entry(c).and_modify(|e| e.push(p)).or_insert(vec![p]);
            acc
        })
}

fn find_antinodes(antennas: &[Point]) -> impl Iterator<Item = Point> + use<'_> {
    antennas
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(i, x)| antennas.iter().skip(i + 1).copied().map(move |y| (x, y)))
        .flat_map(|(x, y)| {
            let d = x - y;
            [y - d, x + d]
        })
}

use crate::grid::{Grid, Point};
use std::collections::{HashMap, HashSet};

pub fn run(input: &str) -> anyhow::Result<String> {
    let grid = Grid::parse(input)?;
    let antennas = find_antennas(&grid);
    let s1 = star(&grid, &antennas, 1);
    let s2 = star(&grid, &antennas, 2);
    Ok(format!("{s1} {s2}"))
}

fn star(grid: &Grid<u8>, antennas: &HashMap<u8, Vec<Point>>, star: i32) -> usize {
    let antenna_pairs = antennas.values().flat_map(|locs| {
        locs.iter()
            .copied()
            .enumerate()
            .flat_map(|(i, x)| locs.iter().skip(i + 1).copied().map(move |y| (x, y)))
    });

    let antinode_locs: HashSet<Point> = match star {
        1 => antenna_pairs
            .flat_map(|(x, y)| antinodes_1(grid, x, y))
            .collect(),
        2 => antenna_pairs
            .flat_map(|(x, y)| antinodes_2(grid, x, y))
            .collect(),
        _ => {
            panic!("invalid star");
        }
    };

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

fn antinodes_1(grid: &Grid<u8>, x: Point, y: Point) -> impl Iterator<Item = Point> + use<'_> {
    let d = x - y;
    [y - d, x + d].into_iter().filter(|p| grid.is_inside(*p))
}

fn antinodes_2(grid: &Grid<u8>, x: Point, y: Point) -> impl Iterator<Item = Point> + use<'_> {
    let d = x - y;
    let itx = std::iter::successors(Some(x), move |p| {
        let q = *p + d;
        grid.is_inside(q).then_some(q)
    });
    let ity = std::iter::successors(Some(y), move |p| {
        let q = *p - d;
        grid.is_inside(q).then_some(q)
    });
    itx.chain(ity)
}

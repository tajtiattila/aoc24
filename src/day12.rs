use crate::grid::{Grid, Point};
use std::collections::BTreeMap;

pub fn run(input: &str) -> anyhow::Result<String> {
    let garden = parse_garden(input)?;
    let (s1, s2) = fence_cost(&garden);
    Ok(format!("{s1} {s2}"))
}

// (direction, bit mask)
const N: (Point, u8) = (Point::new(0, -1), 1);
const S: (Point, u8) = (Point::new(0, 1), 2);
const E: (Point, u8) = (Point::new(1, 0), 4);
const W: (Point, u8) = (Point::new(-1, 0), 8);

pub const FENCES: &[(Point, u8); 4] = &[N, S, E, W];

fn parse_garden(input: &str) -> anyhow::Result<Grid<u16>> {
    let g = Grid::parse(input)?;
    let mut r = Grid::new(g.dimensions(), 0);
    for p in g.positions() {
        let plot = *g.get(p).unwrap();
        let fence_mask = FENCES.iter().fold(0u8, |acc, (s, m)| {
            if g.get(p) != g.get(p + *s) {
                acc | *m
            } else {
                acc
            }
        });
        *r.get_mut(p).unwrap() = plot as u16 + ((fence_mask as u16) << 8);
    }
    Ok(r)
}

fn fence_cost(garden: &Grid<u16>) -> (usize, usize) {
    let mut vis = Grid::new(garden.dimensions(), 0u8);
    garden.positions().fold((0, 0), |(al, ar), p| {
        let (l, r) = flood_step(garden, &mut vis, p);
        (al + l, ar + r)
    })
}

fn flood_step(garden: &Grid<u16>, vis: &mut Grid<u8>, p: Point) -> (usize, usize) {
    if vis.get(p) != Some(&0) {
        return (0, 0);
    }

    let plot = garden.get(p).unwrap() & 0xFF;
    let mut stack = vec![p];
    *vis.get_mut(p).unwrap() = 1;
    let mut fences = Fences::new();

    let mut n_area = 0;
    let mut n_perim = 0;
    while let Some(p) = stack.pop() {
        n_area += 1;
        let fence_mask = (garden.get(p).unwrap() >> 8) as u8;
        fences.add(p, fence_mask);
        n_perim += fence_mask.count_ones() as usize;
        for q in FENCES.iter().map(|(s, _)| p + *s) {
            let vr = vis.get_mut(q);
            let gr = garden.get(q).map(|v| v & 0xFF);
            if vr == Some(&mut 0) && gr == Some(plot) {
                *vr.unwrap() = 1;
                stack.push(q);
            }
        }
    }
    (n_area * n_perim, n_area * fences.count_sides())
}

struct Fences {
    map: BTreeMap<(i32, i32), u8>,
    ymax: i32,
}

impl Fences {
    fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            ymax: 0,
        }
    }

    fn add(&mut self, p: Point, fence: u8) {
        if fence != 0 {
            self.map.insert(p.xy(), fence);
            self.ymax = self.ymax.max(p.y + 1);
        }
    }

    fn count_sides(&self) -> usize {
        #[derive(Copy, Clone)]
        struct Col {
            x_cont: i32,
            north: bool,
            south: bool,
        }
        #[derive(Copy, Clone)]
        struct Row {
            y_cont: i32,
            east: bool,
            west: bool,
        }
        const START_ROW: Row = Row {
            y_cont: -1,
            east: false,
            west: false,
        };
        let mut cols = vec![
            Col {
                x_cont: -1,
                north: false,
                south: false
            };
            self.ymax as usize + 1
        ];
        let mut last_x = -1;
        let mut row = START_ROW;
        let mut n_sides = 0;
        for (p, fence) in &self.map {
            let (x, y) = *p;
            if x != last_x {
                row = START_ROW;
                last_x = x;
            }

            let has_north = (fence & N.1) != 0;
            let has_south = (fence & S.1) != 0;
            let has_east = (fence & E.1) != 0;
            let has_west = (fence & W.1) != 0;

            let col = &mut cols[y as usize];
            let cont_north = col.x_cont == x && col.north;
            let cont_south = col.x_cont == x && col.south;
            let cont_east = row.y_cont == y && row.east;
            let cont_west = row.y_cont == y && row.west;

            col.x_cont = x + 1;
            row.y_cont = y + 1;
            col.north = has_north;
            col.south = has_south;
            row.east = has_east;
            row.west = has_west;

            if has_north && !cont_north {
                n_sides += 1;
            }
            if has_south && !cont_south {
                n_sides += 1;
            }
            if has_east && !cont_east {
                n_sides += 1;
            }
            if has_west && !cont_west {
                n_sides += 1;
            }
        }
        n_sides
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = r#"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"#
        .trim();
        let garden = parse_garden(input).unwrap();
        assert_eq!(fence_cost(&garden), (1930, 1206));
    }
}

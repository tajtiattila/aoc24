use crate::grid::{Grid, Point, STEPS};

pub fn run(input: &str) -> anyhow::Result<String> {
    let garden = parse_garden(input)?;
    let s1 = fence_cost(&garden);
    let s2 = "";
    Ok(format!("{s1} {s2}"))
}

fn parse_garden(input: &str) -> anyhow::Result<Grid<u16>> {
    let g = Grid::parse(input)?;
    let mut r = Grid::new(g.dimensions(), 0);
    for p in g.positions() {
        let plot = *g.get(p).unwrap();
        let n_fences = STEPS.iter().fold(0u16, |acc, s| {
            if g.get(p) != g.get(p+*s) {
                acc + 1
            } else {
                acc
            }
        });
        *r.get_mut(p).unwrap() = plot as u16 + (n_fences << 8);
    }
    Ok(r)
}

fn fence_cost(garden: &Grid<u16>) -> usize {
    let mut vis = Grid::new(garden.dimensions(), 0u8);
    garden.positions().fold(0, |acc, p| acc+flood_step(garden, &mut vis, p))
}

fn flood_step(garden: &Grid<u16>, vis: &mut Grid<u8>, p: Point) -> usize {
    if vis.get(p) != Some(&0) {
        return 0;
    }

    let plot = garden.get(p).unwrap() & 0xFF;
    let mut stack = vec![p];
    *vis.get_mut(p).unwrap() = 1;

    let mut n_area = 0;
    let mut n_perim = 0;
    while let Some(p) = stack.pop() {
        n_area += 1;
        n_perim += (garden.get(p).unwrap() >> 8) as usize;
        for q in STEPS.iter().map(|s| p+*s) {
            let vr = vis.get_mut(q);
            let gr = garden.get(q).map(|v| v & 0xFF);
            if vr == Some(&mut 0) && gr == Some(plot) {
                *vr.unwrap() = 1;
                stack.push(q);
            }
        }
    }
    n_area * n_perim
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
"#.trim();
        let garden = parse_garden(input).unwrap();
        assert_eq!(fence_cost(&garden), 1930);
    }
}

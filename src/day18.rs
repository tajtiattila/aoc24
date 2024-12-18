use crate::grid::{Grid, Point, STEPS};
use crate::Cli;
use std::collections::VecDeque;

pub fn run(input: &str) -> anyhow::Result<String> {
    let s1 = star1(input, 70, 1024);
    let s2 = match star2(input, 70, Cli::global().verbose) {
        Some((x, y)) => format!("{x},{y}"),
        None => String::from("failed"),
    };
    Ok(format!("{s1} {s2}"))
}

fn star1(input: &str, dim: i32, len: usize) -> usize {
    let dims = (dim + 1, dim + 1);
    let grid = coords(input)
        .take(len)
        .fold(Grid::new(dims, b'.'), |mut grid, p| {
            if let Some(c) = grid.get_mut(p) {
                *c = b'#';
            }
            grid
        });

    let mut reach = Grid::new(dims, NOT_REACHED);
    flood(&grid, &mut reach);
    reach.get(Point::new(0, 0)).copied().unwrap_or_default() as usize
}

fn star2(input: &str, dim: i32, verbose: bool) -> Option<(i32, i32)> {
    let dims = (dim + 1, dim + 1);
    let mut grid = Grid::new(dims, b'.');
    let mut reach = Grid::new(dims, NOT_REACHED);

    flood(&grid, &mut reach);

    let start = Point::new(0, 0);
    let mut block = None;

    for p in coords(input) {
        let p_reach = *reach.get(p).unwrap();
        if p_reach != NOT_REACHED {
            if let Some(c) = grid.get_mut(p) {
                *c = b'#';
            }
            flood(&grid, &mut reach);
        }
        if block.is_none() && *reach.get(start).unwrap() == NOT_REACHED {
            block = Some(p.xy());
        }

        if verbose {
            println!(
                "{:?} {} {}",
                p.xy(),
                p_reach,
                if *reach.get(start).unwrap() != NOT_REACHED {
                    "ok"
                } else {
                    "blocked"
                }
            );
        }
    }
    block
}

fn coords(input: &str) -> impl Iterator<Item = Point> + use<'_> {
    input.lines().filter_map(|line| {
        let (x, y) = line.split_once(',')?;
        Some(Point::new(x.parse::<i32>().ok()?, y.parse::<i32>().ok()?))
    })
}

const NOT_REACHED: u16 = u16::MAX;

fn flood(grid: &Grid<u8>, vis: &mut Grid<u16>) {
    vis.fill(NOT_REACHED);

    let (dx, dy) = grid.dimensions();
    let goal = Point::new(dx - 1, dy - 1);

    let mut stk = VecDeque::from([(goal, 0u16)]);
    while let Some((p, n)) = stk.pop_front() {
        *vis.get_mut(p).unwrap() = n;
        for &step in STEPS {
            let q = p + step;
            if grid.get(q) == Some(&b'.') {
                if let Some(qv) = vis.get_mut(q) {
                    if *qv == NOT_REACHED {
                        *qv = n + 1;
                        stk.push_back((q, n + 1));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let sample = r#"
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"#
        .trim();

        assert_eq!(star1(sample, 6, 12), 22);
        assert_eq!(star2(sample, 6, true), Some((6, 1)));
    }
}

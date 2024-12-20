use crate::grid::{Grid, Point, STEPS};
use anyhow::{anyhow, Result};
use std::collections::{HashMap, VecDeque};

pub fn run(input: &str) -> Result<String> {
    let track = Track::parse(input)?;
    let path = find_path(&track);
    let s1 = find_cheats(track.grid.dimensions(), &path)
        .values()
        .filter(|&n| *n >= 100)
        .count();
    let s2 = find_cheats_new(&path, 20, 100, |d| d >= 100);
    Ok(format!("{s1} {s2}"))
}

struct Track {
    grid: Grid<u8>,
    start: Point,
    end: Point,
}

impl Track {
    fn parse(input: &str) -> Result<Self> {
        let grid = Grid::parse(input)?;
        let start = grid
            .iter()
            .find(|(_, c)| *c == &b'S')
            .ok_or_else(|| anyhow!("no start"))?
            .0;
        let end = grid
            .iter()
            .find(|(_, c)| *c == &b'E')
            .ok_or_else(|| anyhow!("no end"))?
            .0;
        Ok(Self { grid, start, end })
    }
}

const NEW: u16 = u16::MAX;

fn find_path(t: &Track) -> Vec<Point> {
    let mut vis = Grid::new(t.grid.dimensions(), NEW);
    *vis.get_mut(t.start).unwrap() = 0;
    let mut work = VecDeque::from([(t.start, 0)]);

    // fill vis with distance from start
    'fill: while let Some((p, pd)) = work.pop_front() {
        for &step in STEPS {
            let q = p + step;
            if let Some(qd) = vis.get_mut(q) {
                if *qd == NEW && t.grid.get(q) != Some(&b'#') {
                    *qd = pd + 1;
                    work.push_back((q, *qd));
                }
            }
            if q == t.end {
                break 'fill;
            }
        }
    }

    // mark path by backtracking from end
    let mut path = vec![t.end];
    while let Some(&p) = path.last() {
        let pd = *vis.get(p).unwrap();
        if pd == 0 || pd == NEW {
            break;
        }
        for &step in STEPS {
            let q = p + step;
            if let Some(qd) = vis.get(q) {
                if *qd < pd {
                    path.push(q);
                }
            }
        }
    }
    path.reverse();
    path
}

fn find_cheats_new(
    path: &[Point],
    duration: usize,
    min_save: usize,
    mut check: impl FnMut(usize) -> bool,
) -> usize {
    path.iter()
        .enumerate()
        .flat_map(|(pi, &p)| {
            path.iter()
                .enumerate()
                .skip(pi + min_save)
                .map(move |(qi, &q)| (pi, p, qi, q))
        })
        .filter(|(pi, p, qi, q)| {
            let d_normal = qi - pi;
            let d_cheat = manhattan_dist(*p, *q);
            d_cheat <= duration && check(d_normal - d_cheat)
        })
        .count()
}

fn manhattan_dist(p: Point, q: Point) -> usize {
    let (dx, dy) = (p - q).xy();
    (dx.abs() + dy.abs()) as usize
}

fn find_cheats(dim: (i32, i32), pathv: &[Point]) -> HashMap<(Point, usize), u16> {
    let path = pathv
        .iter()
        .enumerate()
        .fold(Grid::new(dim, NEW), |mut m, (i, p)| {
            *m.get_mut(*p).unwrap() = i as u16;
            m
        });

    /*
    vis.show_by(|n| {
        if *n != NEW {
            format!("{n:3}")
        } else {
            String::from(" ##")
        }
    });
    */

    // find cheat locations
    let mut cheats = HashMap::new();
    for (p, &pn) in path.iter() {
        if pn == NEW {
            continue;
        }
        for (i, &step) in STEPS.iter().enumerate() {
            let q = p + step;
            let r = q + step;
            if path.get(q) == Some(&NEW) {
                if let Some(rn) = path.get(r).copied() {
                    if rn < pn {
                        cheats.insert((q, i), pn - rn - 2);
                    }
                }
            }
        }
    }
    cheats
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let sample = r#"
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"#
        .trim();
        let track = Track::parse(sample).unwrap();
        let path = find_path(&track);

        let cheats = find_cheats(track.grid.dimensions(), &path);
        println!("{cheats:?}");

        let has_cheat = |x, y, v| {
            cheats
                .iter()
                .filter(|((p, _), &pv)| (x, y) == p.xy() && v == pv)
                .count()
                > 0
        };

        assert!(has_cheat(8, 1, 12));
        assert!(has_cheat(10, 7, 20));
        assert!(has_cheat(8, 8, 38));
        assert!(has_cheat(6, 7, 64));

        let counts = cheats
            .values()
            .fold(HashMap::<u16, usize>::new(), |mut m, &n| {
                m.entry(n).and_modify(|v| *v += 1).or_insert(1);
                m
            });
        println!("{counts:?}");
        assert_eq!(counts.get(&2), Some(&14));
        assert_eq!(counts.get(&4), Some(&14));
        assert_eq!(counts.get(&64), Some(&1));

        assert_eq!(find_cheats_new(&path, 20, 50, |d| d == 50), 32);
        assert_eq!(find_cheats_new(&path, 20, 60, |d| d == 60), 23);
        assert_eq!(find_cheats_new(&path, 20, 68, |d| d == 68), 14);
        assert_eq!(find_cheats_new(&path, 20, 76, |d| d == 76), 3);
    }
}

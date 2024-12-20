use crate::grid::{Grid, Point, STEPS};
use anyhow::{anyhow, Result};
use std::collections::{HashMap, VecDeque};

pub fn run(input: &str) -> Result<String> {
    let track = Track::parse(input)?;
    let s1 = star1(&track, 100);
    let s2 = "";
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

fn star1(t: &Track, min_dist: u16) -> usize {
    find_cheats(t).values().filter(|&n| *n >= min_dist).count()
}

fn find_cheats(t: &Track) -> HashMap<(Point, usize), u16> {
    const NEW: u16 = u16::MAX;
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
    let mut path = Grid::new(t.grid.dimensions(), NEW);
    let mut work = VecDeque::from([t.end]);
    while let Some(p) = work.pop_front() {
        let pd = *vis.get(p).unwrap();
        *path.get_mut(p).unwrap() = pd;
        if pd == 0 {
            break;
        }
        for &step in STEPS {
            let q = p + step;
            if let Some(qd) = vis.get(q) {
                if *qd < pd {
                    work.push_back(q);
                }
            }
        }
    }

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

        let cheats = find_cheats(&track);
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
    }
}

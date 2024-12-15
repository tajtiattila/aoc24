use crate::grid::{Grid, Point};
use anyhow::{anyhow, Result};
use std::iter::successors;

pub fn run(input: &str) -> Result<String> {
    let (s1, s2) = stars(input)?;
    Ok(format!("{s1} {s2}"))
}

fn stars(input: &str) -> Result<(usize, usize)> {
    let (maps, moves) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("input: missing separator"))?;
    let (grid, start) = parse_map(maps)?;
    let (wgrid, wstart) = widen(&grid, start);
    let s1 = star(grid, start, moves);
    let s2 = star(wgrid, wstart, moves);
    Ok((s1, s2))
}

fn star(mut grid: Grid<u8>, start: Point, moves: &str) -> usize {
    moves_iter(moves).fold(start, |p, step| step_robot(&mut grid, p, step));
    grid.positions()
        .filter(|p| {
            let c = grid.get(*p);
            c == Some(&b'O') || c == Some(&b'[')
        })
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
        Some(&b'[') | Some(&b']') => {
            if try_push_wide_boxes(grid, p1, step) {
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

fn try_push_wide_boxes(grid: &mut Grid<u8>, p: Point, step: Point) -> bool {
    const DBG: bool = false;
    if DBG {
        let q = p - step;
        *grid.get_mut(q).unwrap() = b'@';
        println!("{p:?} {step:?}");
        grid.show();
        *grid.get_mut(q).unwrap() = b'.';
    }
    let ok = if step.y == 0 {
        try_push_wide_boxes_h(grid, p, step)
    } else {
        try_push_wide_boxes_v(grid, p, step)
    };
    if DBG && ok {
        *grid.get_mut(p).unwrap() = b'@';
        grid.show();
        *grid.get_mut(p).unwrap() = b'.';
    }
    ok
}

fn try_push_wide_boxes_h(grid: &mut Grid<u8>, p: Point, step: Point) -> bool {
    let r = successors(Some(p), |&p| Some(p + step))
        .map(|p| grid.get(p))
        .find(|xc| !(*xc == Some(&b'[') || *xc == Some(&b']')));
    if let Some(qc) = r {
        if qc == Some(&b'.') {
            successors(Some(p), |&p| Some(p + step)).try_fold(b'.', |acc, p| {
                let c = grid.get_mut(p).unwrap();
                let xacc = *c;
                *c = acc;
                (xacc != b'.').then_some(xacc)
            });
            return true;
        }
    }
    false
}

fn try_push_wide_boxes_v(grid: &mut Grid<u8>, p: Point, step: Point) -> bool {
    let p = if grid.get(p) == Some(&b']') {
        p + Point::new(-1, 0)
    } else {
        p
    };
    let mut cy = p.y;
    let mut boxes = vec![(cy, vec![p.x])];
    while grid.is_inside(Point::new(p.x, cy)) {
        let y = cy + step.y;
        let xs = &boxes.last().unwrap().1;
        let mut next_row = vec![];
        for &x in xs {
            let k0 = match grid.get(Point::new(x, y)) {
                Some(&b'.') => true,
                Some(&b']') => {
                    if next_row.last().copied() != Some(x - 1) {
                        next_row.push(x - 1);
                    }
                    true
                }
                Some(&b'[') => {
                    next_row.push(x);
                    true
                }
                _ => false,
            };
            let k1 = match grid.get(Point::new(x + 1, y)) {
                Some(&b'.') | Some(&b']') => true,
                Some(&b'[') => {
                    next_row.push(x + 1);
                    true
                }
                _ => false,
            };
            if !(k0 && k1) {
                return false;
            }
        }

        if next_row.is_empty() {
            // no new boxes to push, nothing blocked
            let boxes = boxes
                .into_iter()
                .rev()
                .flat_map(|(y, xs)| xs.into_iter().map(move |x| Point::new(x, y)));
            push_boxes(grid, boxes, step.y);
            return true;
        }
        boxes.push((y, next_row));
        cy = y;
    }
    false
}

fn push_boxes(grid: &mut Grid<u8>, boxes: impl Iterator<Item = Point>, dy: i32) {
    for p in boxes {
        let q = p + Point::new(0, dy);
        let r = Point::new(1, 0);
        grid.swap(p, q);
        grid.swap(p + r, q + r);
    }
}

fn widen(grid: &Grid<u8>, start: Point) -> (Grid<u8>, Point) {
    let (x, y) = grid.dimensions();
    let mut w = Grid::new((2 * x, y), b'.');
    for p in grid.positions() {
        let c = *grid.get(p).unwrap();
        let (c0, c1) = if c == b'O' { (b'[', b']') } else { (c, c) };
        let (x, y) = p.xy();
        *w.get_mut(Point::new(2 * x, y)).unwrap() = c0;
        *w.get_mut(Point::new(2 * x + 1, y)).unwrap() = c1;
    }
    let (x, y) = start.xy();
    (w, Point::new(2 * x, y))
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let sample1 = r#"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"#
        .trim();

        let sample2 = r#"
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
"#
        .trim();

        let sample3 = r#"
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
"#
        .trim();

        let r1 = stars(sample1).unwrap();
        let r2 = stars(sample2).unwrap();
        let _r3 = stars(sample3).unwrap();
        assert_eq!(r1.0, 10092);
        assert_eq!(r2.0, 2028);
        assert_eq!(r1.1, 9021);
    }
}

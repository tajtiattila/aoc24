use crate::grid::{pt, Grid, Point};
use crate::Cli;
use anyhow::{anyhow, Result};
use std::cmp::Ordering;

pub fn run(input: &str) -> Result<String> {
    let robots = input.lines().map(parse_robot).collect::<Result<Vec<_>>>()?;
    let s1 = safety_factor(&robots, 101, 103, 100);
    let s2 = xmas_iter(&robots, 101, 103);
    Ok(format!("{s1} {s2}"))
}

#[derive(Copy, Clone, Debug)]
struct Robot {
    p: Point,
    v: Point,
}

fn parse_robot(line: &str) -> Result<Robot> {
    let (ps, vs) = line
        .split_once(' ')
        .ok_or_else(|| anyhow!("invalid line: {line}"))?;
    let p = parse_coords(ps, "p=")?;
    let v = parse_coords(vs, "v=")?;
    Ok(Robot { p, v })
}

fn parse_coords(s: &str, prefix: &str) -> Result<Point> {
    let (x, y) = s
        .strip_prefix(prefix)
        .and_then(|z| z.split_once(','))
        .and_then(|(xs, ys)| Some((xs.parse::<i32>().ok()?, ys.parse::<i32>().ok()?)))
        .ok_or_else(|| anyhow!("invalid {s} for {prefix}"))?;
    Ok(pt(x, y))
}

fn wrap(v: i32, w: i32) -> i32 {
    let r = v % w;
    if v >= 0 || r == 0 {
        r
    } else {
        w + r
    }
}

fn safety_factor(robots: &[Robot], dx: i32, dy: i32, nsec: i32) -> usize {
    let hx = dx / 2;
    let hy = dy / 2;
    robots
        .iter()
        .map(|&Robot { p, v }| (wrap(p.x + v.x * nsec, dx), wrap(p.y + v.y * nsec, dy)))
        .filter_map(|(x, y)| quadrant(pt(x, y), hx, hy))
        .fold([0; 4], |mut acc, quadrant| {
            acc[quadrant] += 1;
            acc
        })
        .iter()
        .product()
}

fn quadrant(p: Point, hx: i32, hy: i32) -> Option<usize> {
    let ix = match p.x.cmp(&hx) {
        Ordering::Less => 0,
        Ordering::Greater => 1,
        _ => {
            return None;
        }
    };
    let iy = match p.y.cmp(&hy) {
        Ordering::Less => 0,
        Ordering::Greater => 2,
        _ => {
            return None;
        }
    };
    Some(ix + iy)
}

fn xmas_iter(robots: &[Robot], dx: i32, dy: i32) -> usize {
    let verbose = Cli::global().verbose;
    let mut robots = robots.to_vec();
    let mut nsec = 0;
    loop {
        for r in &mut robots {
            r.p += r.v;
            if r.p.x < 0 {
                r.p.x += dx;
            } else if r.p.x >= dx {
                r.p.x -= dx;
            }
            if r.p.y < 0 {
                r.p.y += dy;
            } else if r.p.y >= dy {
                r.p.y -= dy;
            }
        }
        nsec += 1;
        if is_xmas_tree(&robots) {
            if verbose {
                println!("{nsec} sec");
                print_robots(&robots, dx, dy);
            }
            return nsec;
        }
        if verbose && nsec % 1000 == 0 {
            println!("{nsec} sec");
        }
    }
}

fn map_robots(grid: &mut Grid<u8>, robots: &[Robot]) -> usize {
    grid.fill(b'.');
    let mut ndup = 0;
    for &Robot { p, .. } in robots {
        let v = grid.get_mut(p).unwrap();
        if *v == b'.' {
            *v = b'1';
        } else if *v < b'Z' {
            ndup += 1;
            if *v == b'9' {
                *v = b'A';
            } else {
                *v += 1;
            }
        }
    }
    ndup
}

fn is_xmas_tree(robots: &[Robot]) -> bool {
    // idea: all bots at unique positions
    let mut buf = [0u128; 103];
    let pic = &mut buf;

    for &Robot { p, .. } in robots {
        let m = 1u128 << p.x;
        let v = &mut pic[p.y as usize];
        if (*v & m) != 0 {
            return false;
        }
        *v |= m;
    }
    true
}

fn print_robots(robots: &[Robot], dx: i32, dy: i32) {
    let mut grid = Grid::new((dx, dy), b'.');
    map_robots(&mut grid, robots);
    grid.show()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(wrap(3, 3), 0);
        assert_eq!(wrap(-3, 3), 0);
        assert_eq!(wrap(5, 3), 2);
        assert_eq!(wrap(-5, 3), 1);
    }
}

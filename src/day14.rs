use anyhow::{anyhow, Result};
use std::cmp::Ordering;

pub fn run(input: &str) -> Result<String> {
    let robots = input.lines().map(parse_robot).collect::<Result<Vec<_>>>()?;
    let s1 = safety_factor(&robots, 101, 103, 100);
    let s2 = "";
    Ok(format!("{s1} {s2}"))
}

type Coord = i32;

struct Robot {
    p: (Coord, Coord),
    v: (Coord, Coord),
}

fn parse_robot(line: &str) -> Result<Robot> {
    let (ps, vs) = line
        .split_once(' ')
        .ok_or_else(|| anyhow!("invalid line: {line}"))?;
    let p = parse_coords(ps, "p=")?;
    let v = parse_coords(vs, "v=")?;
    Ok(Robot { p, v })
}

fn parse_coords(s: &str, prefix: &str) -> Result<(Coord, Coord)> {
    let (x, y) = s
        .strip_prefix(prefix)
        .and_then(|z| z.split_once(','))
        .and_then(|(xs, ys)| Some((xs.parse::<Coord>().ok()?, ys.parse::<Coord>().ok()?)))
        .ok_or_else(|| anyhow!("invalid {s} for {prefix}"))?;
    Ok((x, y))
}

fn wrap(v: Coord, w: Coord) -> Coord {
    let r = v % w;
    if v >= 0 || r == 0 {
        r
    } else {
        w + r
    }
}

fn safety_factor(robots: &[Robot], dx: Coord, dy: Coord, nsec: Coord) -> usize {
    let hx = dx / 2;
    let hy = dy / 2;
    robots
        .iter()
        .map(|&Robot { p, v }| (wrap(p.0 + v.0 * nsec, dx), wrap(p.1 + v.1 * nsec, dy)))
        .filter_map(|(x, y)| {
            let ix = match x.cmp(&hx) {
                Ordering::Less => 0,
                Ordering::Greater => 1,
                _ => {
                    return None;
                }
            };
            let iy = match y.cmp(&hy) {
                Ordering::Less => 0,
                Ordering::Greater => 2,
                _ => {
                    return None;
                }
            };
            Some(ix + iy)
        })
        .fold([0; 4], |mut acc, quadrant| {
            acc[quadrant] += 1;
            acc
        })
        .iter()
        .product()
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

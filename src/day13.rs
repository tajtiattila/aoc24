use crate::grid::Point;
use anyhow::{anyhow, bail, Result};

pub fn run(input: &str) -> Result<String> {
    let machines = parse_machines(input)?;
    for m in &machines {
        if m.a.x * m.b.y == m.a.y * m.b.x {
            println!("{m:?} is colinear");
        }
    }
    let s1 = min_tokens_shift(&machines, 0);
    let s2 = min_tokens_shift(&machines, STAR_2_SHIFT);
    Ok(format!("{s1} {s2}"))
}

#[derive(Copy, Clone, Debug)]
struct Machine {
    a: Point,
    b: Point,
    prize: Point,
}

const A_PRICE: usize = 3;
const B_PRICE: usize = 1;
const STAR_2_SHIFT: i64 = 10_000_000_000_000;

fn min_tokens_shift(m: &[Machine], prize_shift: i64) -> usize {
    m.iter()
        .filter_map(|m| find_min_tokens_shift(m, prize_shift))
        .sum()
}

fn find_min_tokens_shift(machine: &Machine, prize_shift: i64) -> Option<usize> {
    let ax = machine.a.x as f64;
    let ay = machine.a.y as f64;
    let bx = machine.b.x as f64;
    let by = machine.b.y as f64;
    let px = machine.prize.x as f64 + prize_shift as f64;
    let py = machine.prize.y as f64 + prize_shift as f64;

    let m = if by.abs() > bx.abs() {
        (px - py * bx / by) / (ax - ay * bx / by)
    } else {
        (py - px * by / bx) / (ay - ax * by / bx)
    }
    .round() as i64;

    let n = if ay.abs() > bx.abs() {
        (px - py * ax / ay) / (bx - by * ax / ay)
    } else {
        (py - px * ay / ax) / (by - bx * ay / ax)
    }
    .round() as i64;

    let ax = machine.a.x as i64;
    let ay = machine.a.y as i64;
    let bx = machine.b.x as i64;
    let by = machine.b.y as i64;
    let px = machine.prize.x as i64 + prize_shift;
    let py = machine.prize.y as i64 + prize_shift;

    let (qx, qy) = (m * ax + n * bx, m * ay + n * by);
    ((px, py) == (qx, qy)).then(|| m as usize * A_PRICE + n as usize * B_PRICE)
}

fn parse_machines(input: &str) -> Result<Vec<Machine>> {
    let mut r = Vec::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let a = parse_def(Some(line), "Button A")?;
        let b = parse_def(lines.next(), "Button B")?;
        let prize = parse_def(lines.next(), "Prize")?;
        r.push(Machine { a, b, prize });
        match lines.next() {
            Some("") => {}
            None => {
                break;
            }
            Some(x) => {
                bail!("expected empty line, got {x}");
            }
        }
    }
    Ok(r)
}

fn parse_def(line: Option<&str>, prefix: &str) -> Result<Point> {
    let line = line.ok_or_else(|| anyhow!("unexpected eof"))?;
    let (ps, zs) = line
        .split_at_checked(prefix.len())
        .ok_or_else(|| anyhow!("expected {prefix} in {line}"))?;
    if ps != prefix {
        bail!(anyhow!("expected {prefix} in {line}"));
    }
    let zs = zs.trim_start_matches(':').trim();
    let (xs, ys) = zs
        .split_once(',')
        .ok_or_else(|| anyhow!("invalid line {line}"))?;
    Ok(Point::new(parse_num(xs, 'X')?, parse_num(ys, 'Y')?))
}

fn parse_num(s: &str, what: char) -> Result<i32> {
    let s = s.trim_start_matches([what, ' ', '=', '+']).trim();
    Ok(s.parse()?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = r#"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"#
        .trim();
        let m = parse_machines(input).unwrap();
        assert_eq!(m.len(), 4);
        assert_eq!(min_tokens_shift(&m, 0), 480);

        assert!(find_min_tokens_shift(&m[0], STAR_2_SHIFT).is_none());
        assert!(find_min_tokens_shift(&m[1], STAR_2_SHIFT).is_some());
        assert!(find_min_tokens_shift(&m[2], STAR_2_SHIFT).is_none());
        assert!(find_min_tokens_shift(&m[3], STAR_2_SHIFT).is_some());
    }
}

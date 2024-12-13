use crate::grid::Point;
use std::iter::successors;
use anyhow::{anyhow, bail, Result};

pub fn run(input: &str) -> Result<String> {
    let machines = parse_machines(input)?;
    let s1 = min_tokens(&machines);
    let s2 = "";
    Ok(format!("{s1} {s2}"))
}

#[derive(Copy, Clone)]
struct Machine {
    a: Point,
    b: Point,
    prize: Point,
}

const A_PRICE: usize = 3;
const B_PRICE: usize = 1;

fn min_tokens(m: &[Machine]) -> usize {
    m.iter().filter_map(find_min_tokens).sum()
}

fn find_min_tokens(m: &Machine) -> Option<usize> {
    successors(Some(Point::new(0, 0)), |p| (p.x < m.prize.x && p.y < m.prize.y).then_some(*p + m.a))
        .enumerate()
        .filter_map(|(ma, ama)| {
            let d = m.prize - ama;
            let mbx = d.x / m.b.x;
            let mby = d.y / m.b.y;
            (mbx == mby && d.x % m.b.x == 0 && d.y % m.b.y == 0).then_some((ma, mbx as usize))
        })
        .map(|(ma, mb)| A_PRICE*ma + B_PRICE*mb)
        .min()
}

fn parse_machines(input: &str) -> Result<Vec<Machine>> {
    let mut r = Vec::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let a = parse_def(Some(line), "Button A")?;
        let b = parse_def(lines.next(), "Button B")?;
        let prize = parse_def(lines.next(), "Prize")?;
        r.push(Machine{a, b, prize });
        match lines.next() {
            Some("") => {},
            None => { break; },
            Some(x) => { bail!("expected empty line, got {x}"); },
        }
    }
    Ok(r)
}

fn parse_def(line: Option<&str>, prefix: &str) -> Result<Point> {
    let line = line.ok_or_else(|| anyhow!("unexpected eof"))?;
    let (ps, zs) = line.split_at_checked(prefix.len()).ok_or_else(|| anyhow!("expected {prefix} in {line}"))?;
    if ps != prefix {
        bail!(anyhow!("expected {prefix} in {line}"));
    }
    let zs = zs.trim_start_matches(':').trim();
    let (xs, ys) = zs.split_once(',').ok_or_else(|| anyhow!("invalid line {line}"))?;
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
"#.trim();
        let m = parse_machines(input).unwrap();
        assert_eq!(min_tokens(&m), 480);
    }
}

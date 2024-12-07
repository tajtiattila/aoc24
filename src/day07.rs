use anyhow::{anyhow, Result};

pub fn run(input: &str) -> Result<String> {
    let data = parse_data(input)?;
    let s1: i64 = data
        .iter()
        .filter_map(|l| l.is_add_mul().then_some(l.l))
        .sum();
    let s2: i64 = data
        .iter()
        .filter_map(|l| l.is_add_mul_concat().then_some(l.l))
        .sum();
    Ok(format!("{s1} {s2}"))
}

#[derive(Debug)]
struct Line {
    l: i64,
    r: Vec<i64>,
}

impl Line {
    fn parse(line: &str) -> Result<Self> {
        let (l, r) = line
            .split_once(':')
            .ok_or_else(|| anyhow!("missing ':' in line {}", line))?;

        let l: i64 = l.parse()?;
        let r: Vec<i64> = r
            .split_ascii_whitespace()
            .map(|x| x.parse().map_err(anyhow::Error::new))
            .collect::<Result<Vec<i64>>>()?;
        Ok(Self { l, r })
    }

    fn is_add_mul(&self) -> bool {
        let n_ops = self.r.len() - 1;
        for ops in 0..1 << n_ops {
            let ok = self.l
                == self
                    .r
                    .iter()
                    .skip(1)
                    .fold((self.r[0], ops), |(v, ops), x| {
                        let vv = if ops & 1 == 0 { v + x } else { v * x };
                        (vv, ops >> 1)
                    })
                    .0;
            if ok {
                return true;
            }
        }
        false
    }

    fn is_add_mul_concat(&self) -> bool {
        let mut buf = [0u8; 64];
        let n_ops = self.r.len() - 1;
        let ops = &mut buf[..n_ops];
        loop {
            let value = std::iter::zip(self.r.iter().skip(1), ops.iter()).fold(
                self.r[0],
                |acc, (x, op)| match op {
                    0 => acc + x,
                    1 => acc * x,
                    2 => concat(acc, *x),
                    _ => {
                        panic!("impossible");
                    }
                },
            );
            if value == self.l {
                return true;
            }

            // next op
            let mut carry = 1;
            for x in ops.iter_mut() {
                let v = *x + carry;
                if v < 3 {
                    *x = v;
                    carry = 0;
                    break;
                } else {
                    *x = 0;
                    carry = v - 2;
                }
            }
            if carry != 0 {
                return false;
            }
        }
    }
}

fn parse_data(input: &str) -> Result<Vec<Line>> {
    input.lines().map(Line::parse).collect()
}

fn concat(l: i64, r: i64) -> i64 {
    let mut powers_of_10 = std::iter::successors(Some(1i64), |n| n.checked_mul(10));
    let m = powers_of_10.find(|n| *n > r).unwrap();
    l * m + r
}

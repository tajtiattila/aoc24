use anyhow::{anyhow, Result};
use std::cmp::Ordering;

pub fn run(input: &str) -> Result<String> {
    let pj = parse_print_job(input)?;
    let (s1, s2) = stars(&pj);
    Ok(format!("{s1} {s2}"))
}

struct PrintJob {
    ord: Vec<(i32, i32)>,
    upd: Vec<Vec<i32>>,
}

fn parse_print_job(input: &str) -> Result<PrintJob> {
    let mut pj = PrintJob {
        ord: vec![],
        upd: vec![],
    };

    let mut it = input.lines();

    for line in it.by_ref() {
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        let (x, y) = line
            .split_once('|')
            .ok_or_else(|| anyhow!("invalid line"))?;
        pj.ord.push((x.parse()?, y.parse()?));
    }

    for line in it {
        pj.upd.push(
            line.trim()
                .split(',')
                .map(|x| x.parse::<i32>())
                .collect::<Result<Vec<_>, _>>()?,
        );
    }

    Ok(pj)
}

fn stars(pj: &PrintJob) -> (usize, usize) {
    let mut pages_after = vec![0u128; 100];
    for &(x, y) in pj.ord.iter() {
        pages_after[x as usize] |= 1u128 << y;
    }

    pj.upd
        .iter()
        .map(|pr| {
            let order_ok = pr
                .iter()
                .scan(0u128, |state, n| {
                    let pred = *state;
                    *state |= 1u128 << n;
                    Some((pred, n))
                })
                .all(|(preds, n)| pages_after[*n as usize] & preds == 0);

            if order_ok {
                return (true, pr[pr.len() / 2] as usize);
            }
            let mut v = pr.clone();
            v.sort_by(|&x, &y| {
                let x_less = pages_after[x as usize] & (1 << y) == 0;
                let y_less = pages_after[y as usize] & (1 << x) == 0;
                match (x_less, y_less) {
                    (false, false) => Ordering::Equal,
                    (true, false) => Ordering::Less,
                    (false, true) => Ordering::Greater,
                    (true, true) => {
                        panic!("invalid odering");
                    }
                }
            });
            (false, v[v.len() / 2] as usize)
        })
        .fold((0, 0), |mut acc, (is_ok, v)| {
            if is_ok {
                acc.0 += v;
            } else {
                acc.1 += v;
            }
            acc
        })
}

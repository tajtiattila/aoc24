use anyhow::{anyhow, Result};
use std::collections::{BTreeMap, BTreeSet, HashSet};

pub fn run(input: &str) -> Result<String> {
    let s1 = star1(input)?;
    let s2 = "";
    Ok(format!("{s1} {s2}"))
}

type LenToPats<'a> = BTreeMap<usize, HashSet<&'a str>>;

fn star1(input: &str) -> Result<usize> {
    let mut it = input.lines();

    let ltp = len_to_pats(it.next().ok_or_else(|| anyhow!("invalid input"))?);

    it.next().ok_or_else(|| anyhow!("invalid input"))?;

    Ok(it.filter(|design| design_possible(design, &ltp)).count())
}

fn len_to_pats(line: &str) -> LenToPats {
    line.split(", ").fold(LenToPats::new(), |mut m, pat| {
            m.entry(pat.len())
                .and_modify(|s| {
                    s.insert(pat);
                })
                .or_insert(HashSet::from([pat]));
            m
    })
}

fn design_possible(design: &str, ltp: &LenToPats) -> bool {
    let mut vis = BTreeSet::from([0]);
    let mut poss = vec![0];
    while let Some(p) = poss.pop() {
        let work = &design[p..];
        for (&l, pats) in ltp.iter().take_while(|(l, _)| **l <= work.len()) {
            let cur = &work[0..l];
            if pats.contains(cur) {
                let q = p + l;
                if q == design.len() {
                    return true;
                } else if vis.insert(q) {
                    poss.push(q);
                }
            }
        }
    }
    false
}

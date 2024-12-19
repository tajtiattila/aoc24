use anyhow::{anyhow, Result};
use std::collections::{BTreeMap, HashSet};

pub fn run(input: &str) -> Result<String> {
    let (s1, s2) = stars(input)?;
    Ok(format!("{s1} {s2}"))
}

type LenToPats<'a> = BTreeMap<usize, HashSet<&'a str>>;

fn stars(input: &str) -> Result<(usize, usize)> {
    let mut it = input.lines();

    let ltp = len_to_pats(it.next().ok_or_else(|| anyhow!("invalid input"))?);

    it.next().ok_or_else(|| anyhow!("invalid input"))?;

    Ok(it.fold((0, 0), |(nok, narr), design| {
        let n = arrangements(design, &ltp);
        (nok + (n > 0) as usize, narr + n)
    }))
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

fn arrangements(design: &str, ltp: &LenToPats) -> usize {
    let mut counts = vec![0; design.len() + 1];
    counts[0] = 1;
    for p in 0..design.len() {
        let nfrom = counts[p];
        if nfrom == 0 {
            continue;
        }
        let work = &design[p..];
        for (&l, pats) in ltp.iter().take_while(|(l, _)| **l <= work.len()) {
            let cur = &work[0..l];
            if pats.contains(cur) {
                let q = p + l;
                counts[q] += nfrom;
            }
        }
    }
    counts[design.len()]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let sample = r#"
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"#
        .trim();
        let ltp = len_to_pats(sample.split_once('\n').unwrap().0);

        assert_eq!(arrangements("brwrr", &ltp), 2);
        assert_eq!(arrangements("bggr", &ltp), 1);
        assert_eq!(arrangements("gbbr", &ltp), 4);
        assert_eq!(arrangements("rrbgbr", &ltp), 6);
        assert_eq!(arrangements("bwurrg", &ltp), 1);
        assert_eq!(arrangements("brgr", &ltp), 2);
        assert_eq!(arrangements("ubwu", &ltp), 0);
        assert_eq!(arrangements("bbrgwb", &ltp), 0);

        assert_eq!(stars(sample).unwrap(), (6, 16));
    }
}

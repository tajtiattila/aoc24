use std::collections::HashMap;

pub fn run(input: &str) -> anyhow::Result<String> {
    let secrets = parse_input(input)?;
    let s1 = star1(&secrets);
    let s2 = star2(&secrets);
    Ok(format!("{s1} {s2}"))
}

fn parse_input(input: &str) -> anyhow::Result<Vec<u32>> {
    input
        .lines()
        .map(|line| line.parse::<u32>().map_err(|e| e.into()))
        .collect()
}

fn star1(secrets: &[u32]) -> usize {
    secrets
        .iter()
        .map(|s0| {
            let mut s = *s0;
            for _ in 0..SECRETS_PER_DAY {
                s = step_secret(s);
            }
            s as usize
        })
        .sum()
}

fn star2(secrets: &[u32]) -> usize {
    let mut m = HashMap::new();
    for s in secrets {
        add_changes(&mut m, &map_changes(*s));
    }
    m.values().max().copied().unwrap_or(0)
}

const SECRETS_PER_DAY: usize = 2000;
const PRUNE: u32 = 0xffffff;

fn step_secret(mut s: u32) -> u32 {
    s ^= s << 6;
    s &= PRUNE;
    s ^= s >> 5;
    s &= PRUNE;
    s ^= s << 11;
    s &= PRUNE;
    s
}

fn map_changes(s0: u32) -> HashMap<Changes, usize> {
    price_changes_iter(s0)
        .take(SECRETS_PER_DAY)
        .skip(4)
        .fold(HashMap::new(), |mut acc, (c, p)| {
            acc.entry(c).or_insert(p);
            acc
        })
}

fn add_changes(m: &mut HashMap<Changes, usize>, add: &HashMap<Changes, usize>) {
    for (c, p) in add {
        m.entry(*c).and_modify(|sum| *sum += *p).or_insert(*p);
    }
}

fn secret_iter(s0: u32) -> impl Iterator<Item = u32> {
    std::iter::successors(Some(s0), |s| Some(step_secret(*s)))
}

fn price_changes_iter(s0: u32) -> impl Iterator<Item = (Changes, usize)> {
    secret_iter(s0)
        .map(|s| s % 10)
        .scan(0, |state, p| {
            let d = (p as i8) - (*state as i8);
            *state = p;
            Some((p, d))
        })
        .scan(Changes::new(), |state, (p, d)| {
            state.push(d);
            Some((*state, p as usize))
        })
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Changes([i8; 4]);

impl Changes {
    fn new() -> Self {
        Self([0; 4])
    }

    fn push(&mut self, d: i8) {
        self.0[0] = self.0[1];
        self.0[1] = self.0[2];
        self.0[2] = self.0[3];
        self.0[3] = d;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(step_secret(123), 15887950);
        assert_eq!(secret_iter(1).skip(SECRETS_PER_DAY).next(), Some(8685429));
        assert_eq!(
            price_changes_iter(123).skip(4).next(),
            Some((Changes([-3, 6, -1, -1]), 4))
        );

        let sample1 = parse_input("1\n10\n100\n2024\n").unwrap();
        assert_eq!(star1(&sample1), 37327623);

        let sample2 = parse_input("1\n2\n3\n2024\n").unwrap();
        assert_eq!(star2(&sample2), 23);
    }
}

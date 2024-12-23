use anyhow::anyhow;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt;

pub fn run(input: &str) -> anyhow::Result<String> {
    let network = parse_network(input)?;
    let s1 = star1(&network);
    let s2 = star2(&network);
    Ok(format!("{s1} {s2}"))
}

fn star1(network: &Network) -> usize {
    star1_triplets(network).len()
}

fn star1_triplets(network: &Network) -> HashSet<[Comp; 3]> {
    network_triplets(network, |comp| comp.char0() == 't')
}

fn network_triplets<T: FnMut(&Comp) -> bool>(
    network: &Network,
    mut filter: T,
) -> HashSet<[Comp; 3]> {
    network
        .iter()
        .filter(|(comp, _)| filter(comp))
        .flat_map(|(c0, v)| {
            v.iter()
                .enumerate()
                .flat_map(move |(i, c1)| v.iter().skip(i + 1).map(move |c2| [c0, *c1, *c2]))
        })
        .filter(|[_, c1, c2]| network.has_link(*c1, *c2))
        .map(|mut triplet| {
            triplet.sort();
            triplet
        })
        .collect()
}

fn star2(network: &Network) -> String {
    let mut work = network_triplets(network, |_| true)
        .into_iter()
        .map(BTreeSet::from)
        .collect::<Vec<CompSet>>();
    let mut vis = HashMap::new();
    while let Some(set) = work.pop() {
        let setc = set.clone();
        for comp in try_expand(network, &set) {
            let mut exp = setc.clone();
            exp.insert(comp);
            vis.entry(exp).or_insert_with_key(|k| {
                work.push(k.clone());
            });
        }
    }
    let longest = vis
        .into_iter()
        .fold((0, CompSet::new()), |acc @ (maxlen, _), (set, _)| {
            if set.len() > maxlen {
                (set.len(), set)
            } else {
                acc
            }
        })
        .1;
    longest.into_iter().fold(String::new(), |mut acc, comp| {
        if !acc.is_empty() {
            acc.push(',');
        }
        acc.push_str(comp.as_str());
        acc
    })
}

type CompSet = BTreeSet<Comp>;

fn try_expand<'a>(network: &'a Network, set: &'a CompSet) -> impl Iterator<Item = Comp> + use<'a> {
    let first = *set.iter().next().unwrap();
    network
        .get_links(first)
        .iter()
        .filter(|link| !set.contains(&link))
        .filter(|link| set.iter().skip(1).all(|x| network.has_link(*x, **link)))
        .copied()
}

#[derive(Debug, Clone)]
struct Network(HashMap<Comp, BTreeSet<Comp>>);

impl Network {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn add_link(&mut self, l: Comp, r: Comp) {
        self.add_link_impl(l, r);
        self.add_link_impl(r, l);
    }

    fn add_link_impl(&mut self, l: Comp, r: Comp) {
        self.0
            .entry(l)
            .and_modify(|v| {
                v.insert(r);
            })
            .or_insert(BTreeSet::from([r]));
    }

    fn has_link(&self, l: Comp, r: Comp) -> bool {
        self.0.get(&l).map(|v| v.contains(&r)) == Some(true)
    }

    fn get_links(&self, comp: Comp) -> &BTreeSet<Comp> {
        static EMPTY: BTreeSet<Comp> = BTreeSet::new();
        self.0.get(&comp).unwrap_or(&EMPTY)
    }

    fn iter(&self) -> impl Iterator<Item = (Comp, &BTreeSet<Comp>)> {
        self.0.iter().map(|(c, v)| (*c, v))
    }

    fn comps(&self) -> impl Iterator<Item = Comp> + use<'_> {
        self.0.keys().copied()
    }

    fn first(&self) -> Option<Comp> {
        self.0.keys().next().copied()
    }

    fn into_subnets(mut self) -> Vec<Self> {
        let mut v = vec![];
        while let Some(comp) = self.first() {
            v.push(self.extract_subnet(comp))
        }
        v
    }

    fn extract_subnet(&mut self, comp: Comp) -> Self {
        self.subnet_comps(comp)
            .into_iter()
            .fold(Network::new(), |mut acc, comp| {
                if let Some(v) = self.0.remove(&comp) {
                    acc.0.insert(comp, v);
                }
                acc
            })
    }

    fn subnet_comps(&self, comp: Comp) -> HashSet<Comp> {
        let mut work = vec![comp];
        let mut set = HashSet::new();
        while let Some(comp) = work.pop() {
            if let Some(links) = self.0.get(&comp) {
                for link in links {
                    if set.insert(*link) {
                        work.push(*link);
                    }
                }
            }
        }
        set
    }
}

fn parse_network(input: &str) -> anyhow::Result<Network> {
    input
        .lines()
        .map(|line| parse_link(line).ok_or_else(|| anyhow!("invalid line: {line}")))
        .try_fold(Network::new(), |mut acc, link| {
            let (l, r) = link?;
            acc.add_link(l, r);
            Ok(acc)
        })
}

fn parse_link(line: &str) -> Option<(Comp, Comp)> {
    let (l, r) = line.split_once('-')?;
    Some((Comp::parse(l)?, Comp::parse(r)?))
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Comp([u8; 2]);

impl Comp {
    fn parse(src: &str) -> Option<Comp> {
        (src.len() == 2 && src.chars().all(|c| ('a'..='z').contains(&c))).then(|| {
            let b = src.as_bytes();
            Comp([b[0], b[1]])
        })
    }

    fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).unwrap()
    }

    fn char0(&self) -> char {
        self.0[0] as char
    }
}

impl fmt::Debug for Comp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0[0] as char, self.0[1] as char)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let sample = r#"
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"#
        .trim();
        let network = parse_network(sample).unwrap();
        println!("{network:?}");
        let comp = |s| Comp::parse(s).unwrap();
        assert!(!network.has_link(comp("cg"), comp("ka")));
        let tris = star1_triplets(&network);
        let mut tris = tris.into_iter().collect::<Vec<_>>();
        tris.sort();
        for [a, b, c] in &tris {
            println!("{:?},{:?},{:?}", a, b, c);
        }

        assert_eq!(tris.len(), 7);

        assert_eq!(star2(&network), "co,de,ka,ta");
    }
}

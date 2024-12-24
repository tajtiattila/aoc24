use anyhow::anyhow;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;

pub fn run(input: &str) -> anyhow::Result<String> {
    let p = Problem::parse(input)?;
    // 3302442726 too low
    let s1 = star1(&p);
    let s2 = star2(&p);
    Ok(format!("{s1} {s2}"))
}

fn star1(p: &Problem) -> u64 {
    let nbits = p.circuit.output_bits();
    p.circuit.output(p.x, p.y, nbits).unwrap_or(0)
}

fn star2(p: &Problem) -> String {
    find_fix(&p.circuit, is_add)
}

fn find_fix<F: FnMut(&Circuit) -> bool>(c: &Circuit, mut f: F) -> String {
    const MAX_SWAPS: usize = 4;
    let wire_pairs = c.wire_pairs().collect::<Vec<_>>();
    let mut vis = HashSet::<SwapSet>::new();
    let mut work = vec![(SwapSet::new(), c.clone())];
    while let Some((swaps, c)) = work.pop() {
        if f(&c) {
            return swaps.wires().fold(String::new(), |acc, wire| {
                let sep = if acc.is_empty() { "" } else { "," };
                acc + &format!("{}{}", sep, wire.as_str())
            });
        }
        if swaps.len() >= MAX_SWAPS {
            continue;
        }
        for (a, b) in &wire_pairs {
            if let Some(swaps) = swaps.with(*a, *b) {
                if !vis.contains(&swaps) {
                    let mut c = c.clone();
                    c.swap(*a, *b);

                    work.push((swaps.clone(), c));
                    vis.insert(swaps);
                }
            }
        }
    }
    String::from("invalid")
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct SwapSet(BTreeMap<Wire, Wire>);

impl SwapSet {
    fn new() -> Self {
        Self(BTreeMap::new())
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn with(&self, a: Wire, b: Wire) -> Option<Self> {
        (!(self.0.contains_key(&a) || self.0.contains_key(&b))).then(|| {
            let mut m = self.0.clone();
            m.insert(a, b);
            m.insert(b, a);
            Self(m)
        })
    }

    fn wires(&self) -> impl Iterator<Item = Wire> + use<'_> {
        self.0.keys().copied()
    }
}

#[allow(unused)]
fn is_and(c: &Circuit) -> bool {
    let nbits = c.output_bits();
    if nbits == 0 {
        return false;
    }
    for i in 0..(nbits - 1) {
        let m = 1 << i;
        if c.output(m, m, nbits) != Some(m) {
            return false;
        }
        if c.output(m, 0, nbits) != Some(0) {
            return false;
        }
        if c.output(m, m - 1, nbits) != Some(0) {
            return false;
        }
    }
    true
}
fn is_add(c: &Circuit) -> bool {
    let nbits = c.output_bits();
    if nbits == 0 {
        return false;
    }
    for i in 0..(nbits - 1) {
        let m = 1 << i;
        if c.output(m, m, nbits) != Some(2 * m) {
            return false;
        }
        if c.output(m, m - 1, nbits) != Some(2 * m - 1) {
            return false;
        }
    }
    true
}

#[derive(Debug, Clone)]
struct Circuit(HashMap<Wire, Gate>);

impl Circuit {
    fn parse(input: &str) -> anyhow::Result<Self> {
        let mut m = HashMap::new();
        for line in input.lines() {
            let mut it = line.split_ascii_whitespace();
            let inv = || anyhow!("invalid: {line}");
            let a = it.next().and_then(Wire::parse).ok_or_else(inv)?;
            let op = it.next().and_then(Op::parse).ok_or_else(inv)?;
            let b = it.next().and_then(Wire::parse).ok_or_else(inv)?;
            if it.next() != Some("->") {
                return Err(inv());
            }
            let out = it.next().and_then(Wire::parse).ok_or_else(inv)?;
            m.insert(out, Gate { a, b, op });
        }
        Ok(Self(m))
    }

    fn wire_pairs(&self) -> impl Iterator<Item = (Wire, Wire)> + use<'_> {
        self.0
            .keys()
            .enumerate()
            .flat_map(|(i, &a)| self.0.keys().skip(i + 1).map(move |&b| (a, b)))
    }

    fn swap(&mut self, a: Wire, b: Wire) -> bool {
        if let (Some(ag), Some(br)) = (self.0.get(&a).copied(), self.0.get_mut(&b)) {
            let bg = *br;
            *br = ag;
            *self.0.get_mut(&a).unwrap() = bg;
            true
        } else {
            false
        }
    }

    fn output_bits(&self) -> u32 {
        self.0
            .keys()
            .filter_map(|&wire| {
                if let Some((c, n)) = wire.char_num() {
                    if c == 'z' {
                        return Some(n + 1);
                    }
                }
                None
            })
            .max()
            .unwrap_or(0)
    }

    fn output(&self, x: u64, y: u64, nbits: u32) -> Option<u64> {
        (0..nbits).try_fold(0, |acc, i| {
            let wire = Wire::parse(&format!("z{:02}", i)).unwrap();
            match self.bit(x, y, wire) {
                Lookup::Valid(v) => Some(acc | ((v as u64) << i)),
                _ => None,
            }
        })
    }

    fn bit(&self, x: u64, y: u64, wire: Wire) -> Lookup {
        let mut vis = HashMap::new();
        self.bit_impl(&mut vis, x, y, wire)
    }

    fn bit_impl(&self, vis: &mut HashMap<Wire, Lookup>, x: u64, y: u64, wire: Wire) -> Lookup {
        use Lookup::*;
        if let Some((c, n)) = wire.char_num() {
            match c {
                'x' => {
                    return Valid((x & (1 << n)) != 0);
                }
                'y' => {
                    return Valid((y & (1 << n)) != 0);
                }
                _ => {}
            }
        }
        if vis.get(&wire) == Some(&Cycle) {
            return Cycle;
        }

        let mut is_cycle = false;
        let r = vis
            .entry(wire)
            .and_modify(|v| {
                if *v == Cycle {
                    is_cycle = true;
                }
            })
            .or_insert(Cycle);
        match *r {
            Cycle => {
                if is_cycle {
                    return Cycle;
                };
            }
            _ => {
                return *r;
            }
        }
        let r = match self.0.get(&wire) {
            Some(g) => {
                let a = self.bit_impl(vis, x, y, g.a);
                let b = self.bit_impl(vis, x, y, g.b);
                if let (Valid(a), Valid(b)) = (a, b) {
                    Valid(g.result(a, b))
                } else {
                    Invalid
                }
            }
            None => Invalid,
        };
        *vis.get_mut(&wire).unwrap() = r;
        r
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Lookup {
    Cycle,
    Invalid,
    Valid(bool),
}

struct Problem {
    x: u64,
    y: u64,
    circuit: Circuit,
}

impl Problem {
    fn parse(input: &str) -> anyhow::Result<Problem> {
        let (l, r) = input
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("invalid input"))?;

        let mut x = 0;
        let mut y = 0;
        for line in l.lines() {
            let inv = || anyhow!("invalid: {line}");
            let (l, r) = line.split_once(": ").ok_or_else(inv)?;
            let wire = Wire::parse(l).ok_or_else(|| anyhow!("invalid: {line}"))?;
            let value = r.parse::<u64>().map_err(anyhow::Error::msg)?;
            match wire.char_num() {
                Some(('x', n)) => {
                    x |= value << n;
                }
                Some(('y', n)) => {
                    y |= value << n;
                }
                _ => return Err(inv()),
            }
        }

        let circuit = Circuit::parse(r)?;

        Ok(Problem { x, y, circuit })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn parse(s: &str) -> Option<Self> {
        use Op::*;
        match s {
            "AND" => Some(And),
            "OR" => Some(Or),
            "XOR" => Some(Xor),
            _ => None,
        }
    }

    fn as_str(&self) -> &'static str {
        use Op::*;
        match *self {
            And => "AND",
            Or => "OR",
            Xor => "XOR",
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Debug for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Gate {
    op: Op,
    a: Wire,
    b: Wire,
}

impl Gate {
    fn result(&self, a: bool, b: bool) -> bool {
        use Op::*;
        match self.op {
            And => a & b,
            Or => a | b,
            Xor => a ^ b,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Wire([u8; 3]);

impl Wire {
    fn parse(s: &str) -> Option<Self> {
        let ok = s.len() == 3
            && s.chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit());
        if ok {
            let arr = s.as_bytes().try_into().ok()?;
            Some(Self(arr))
        } else {
            None
        }
    }

    fn char_num(&self) -> Option<(char, u32)> {
        self.num().map(|n| (self.0[0] as char, n))
    }

    fn num(&self) -> Option<u32> {
        self.as_str()[1..].parse().ok()
    }

    fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).unwrap()
    }
}

impl fmt::Display for Wire {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Debug for Wire {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let sample1 = r#"
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
"#
        .trim();

        let sample2 = r#"
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
"#
        .trim();

        use Lookup::*;

        let p1 = Problem::parse(sample1).unwrap();
        assert_eq!(p1.circuit.output_bits(), 3);

        let cb1 = |s: &str| {
            let wire = Wire::parse(s).unwrap();
            p1.circuit.bit(p1.x, p1.y, wire)
        };
        assert_eq!(cb1("z00"), Valid(false));
        assert_eq!(cb1("z01"), Valid(false));
        assert_eq!(cb1("z02"), Valid(true));

        let p2 = Problem::parse(sample2).unwrap();
        assert_eq!(p2.circuit.output_bits(), 13);

        let s1 = |input| star1(&Problem::parse(input).unwrap());
        assert_eq!(s1(sample1), 4);
        assert_eq!(s1(sample2), 2024);

        assert!(!is_add(&p1.circuit));
        assert!(!is_add(&p2.circuit));

        let sample3 = r#"
x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00
"#
        .trim();
        let p3 = Problem::parse(sample3).unwrap();

        let mut c3 = p3.circuit.clone();
        let w = |s| Wire::parse(s).unwrap();
        c3.swap(w("z00"), w("z05"));
        c3.swap(w("z01"), w("z02"));
        println!("{c3:?}");
        assert!(is_and(&c3));
        assert_eq!(&find_fix(&p3.circuit, is_and), "z00,z01,z02,z05");
    }
}

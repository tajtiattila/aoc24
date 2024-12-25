use crate::Cli;
use anyhow::{anyhow, bail};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet};
use std::fmt;

pub fn run(input: &str) -> anyhow::Result<String> {
    let p = Problem::parse(input)?;
    let s1 = star1(&p);
    let s2 = analyze_adder(&p.circuit, Cli::global().verbose)?;
    Ok(format!("{s1} {s2}"))
}

fn star1(p: &Problem) -> u64 {
    let nbits = p.circuit.output_bits();
    p.circuit.output(p.x, p.y, nbits).unwrap_or(0)
}

fn star2(p: &Problem) -> String {
    find_fix_add(&p.circuit)
}

fn analyze_adder(c: &Circuit, verbose: bool) -> anyhow::Result<String> {
    use Op::*;

    let mut aa = AddAnalyzer::new(c, verbose);
    let nbits = c.output_bits();

    let Some(s0) = aa.find_gate(Xor, Wire::x(0), Wire::y(0)) else {
        bail!("bit 00: s0 not found");
    };
    let Some(mut carry) = aa.find_gate(And, Wire::x(0), Wire::y(0)) else {
        bail!("bit 00: c0 not found");
    };
    if verbose {
        println!(" i  s0_i  c0_i  c1_i   s_i   c_i");
        println!("00                     {}   {}", s0, carry);
    }

    for i in 1..nbits-1 {
        let FullAdder { s0, c0, c1, si, ci } = match aa.find_full_adder(i, carry) {
            Ok(a) => a,
            Err(msg) => bail!("bit {:02}: {}", i, msg),
        };
        carry = ci;

        if verbose {
            println!("{:02}   {}   {}   {}   {}   {}", i, s0, c0, c1, si, ci);
        }
    }

    Ok(aa.into_swaps())
}

struct AddAnalyzer {
    wg: HashMap<Wire, Gate>,
    gw: HashMap<Gate, Wire>,
    swaps: Vec<Wire>,
    verbose: bool,
}

impl AddAnalyzer {
    fn new(c: &Circuit, verbose: bool) -> Self {
        let wg = c.0.clone();
        let gw = wg.iter().map(|(&wire, &gate)| (gate, wire)).collect();
        Self {
            wg,
            gw,
            swaps: vec![],
            verbose,
        }
    }

    // xi XOR yi -> s0_i
    // xi AND yi -> c0_i
    // s0_i XOR c_(i-1) -> s_i (== zi)
    // s0_i AND c_(i-1) -> c1_i
    // c0_i OR c1_i -> c_i
    fn find_full_adder(&mut self, bit: u32, carry: Wire) -> Result<FullAdder, String> {
        use Op::*;
        let zi = Wire::z(bit);
        let xi = Wire::x(bit);
        let yi = Wire::y(bit);
        let Some(s0) = self.find_gate(Xor, xi, yi) else {
            return Err(format!("s0 not found"));
        };
        let Some(mut c0) = self.find_gate(And, xi, yi) else {
            return Err(format!("c0 not found"));
        };
        if c0 == zi {
            let Some(si) = self.find_gate(Xor, s0, carry) else {
                return Err(format!("c0==zi but si not found"));
            };
            self.swap(si, zi);
            c0 = si;
        }

        let err1 = match self.find_full_adder_step2(bit, carry, s0, c0) {
            Ok(a) => {
                return Ok(a);
            }
            Err(msg) => msg,
        };
        let err2 = match self.find_full_adder_step2(bit, carry, c0, s0) {
            Ok(a) => {
                self.swap(s0, c0);
                return Ok(FullAdder {
                    s0: c0,
                    c0: s0,
                    ..a
                });
            }
            Err(msg) => msg,
        };
        Err(format!("step 2 failed with {err1}/{err2}"))
    }

    fn find_full_adder_step2(
        &mut self,
        bit: u32,
        carry: Wire,
        s0: Wire,
        c0: Wire,
    ) -> Result<FullAdder, String> {
        use Op::*;
        let zi = Wire::z(bit);
        let Some(mut si) = self.find_gate(Xor, s0, carry) else {
            return Err(format!("si not found"));
        };
        if si != zi {
            self.swap(si, zi);
            si = zi;
        }
        let Some(c1) = self.find_gate(And, s0, carry) else {
            return Err(format!("c1 not found"));
        };
        let Some(ci) = self.find_gate(Or, c0, c1) else {
            return Err(format!("ci not found"));
        };
        Ok(FullAdder { s0, c0, c1, si, ci })
    }

    fn into_swaps(mut self) -> String {
        self.swaps.sort();
        self.swaps.into_iter().fold(String::new(), |acc, wire| {
            let sep = if acc.is_empty() { "" } else { "," };
            acc + &format!("{sep}{wire}")
        })
    }

    fn find_gate(&self, op: Op, a: Wire, b: Wire) -> Option<Wire> {
        self.gw
            .get(&Gate { op, a, b })
            .or_else(|| self.gw.get(&Gate { op, a: b, b: a }))
            .copied()
    }

    fn swap(&mut self, a: Wire, b: Wire) -> bool {
        if let (Some(ag), Some(br)) = (self.wg.get(&a).copied(), self.wg.get_mut(&b)) {
            let bg = *br;
            *br = ag;
            *self.wg.get_mut(&a).unwrap() = bg;

            self.gw.insert(ag, b);
            self.gw.insert(bg, a);

            if self.verbose {
                println!("  swap {} <-> {}", a, b);
            }
            self.swaps.push(a);
            self.swaps.push(b);
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct FullAdder {
    s0: Wire,
    c0: Wire,
    c1: Wire,
    si: Wire,
    ci: Wire,
}

fn nice<'a>(o: &'a Option<&'a Wire>) -> &'a str {
    o.map(|w| w.as_str()).unwrap_or(" ? ")
}

fn bad_add_bits(c: &Circuit) -> Vec<Wire> {
    let mut r = vec![];
    foreach_add_bit(c, |i, ok| {
        if !ok {
            r.push(Wire::parse(&format!("z{:02}", i)).unwrap());
        }
    });
    r
}

fn foreach_add_bit<F: FnMut(u32, bool)>(c: &Circuit, mut f: F) {
    let nbits = c.output_bits();
    let out = |a, b, m, eq| c.output(a, b, nbits).map(|n| n & m) == Some(eq);

    let bit_0_ok = out(0, 0, 1, 0) && out(1, 0, 1, 1) && out(0, 1, 1, 1) && out(1, 1, 1, 0);
    f(0, bit_0_ok);

    for i in 1..nbits - 1 {
        let m = 1 << i;
        let h = m >> 1;

        let bit_i_ok = out(0, 0, m, 0)
            && out(0, h, m, 0)
            && out(h, 0, m, 0)
            && out(0, m, m, m)
            && out(m, 0, m, m)
            && out(h, h, m, m)
            && out(m, m, m, 0)
            && out(m + h, h, m, 0)
            && out(m, m + h, m, 0);
        f(i, bit_i_ok);
    }
}

fn find_fix_add(c: &Circuit) -> String {
    let mut sets = bad_add_bits(c)
        .into_iter()
        .map(|wire| c.deps(wire))
        .collect::<Vec<_>>();
    sets.sort_by_key(|s| s.len());

    let set_pairs = sets
        .iter()
        .enumerate()
        .flat_map(|(i, s0)| sets.iter().skip(i + 1).map(move |s1| (s0, s1)));

    let wire_pairs_iter = set_pairs
        .flat_map(|(s0, s1)| s0.iter().flat_map(|w0| s1.iter().map(move |w1| (w0, w1))))
        .filter_map(|(&w0, &w1)| {
            use std::cmp::Ordering::*;
            match w0.cmp(&w1) {
                Less => Some((w0, w1)),
                Equal => None,
                Greater => Some((w1, w0)),
            }
        });

    let mut wire_pairs = vec![];
    let mut vis = HashSet::new();
    for (w0, w1) in wire_pairs_iter {
        if vis.insert((w0, w1)) {
            wire_pairs.push((w0, w1));
        }
    }

    find_fix_impl(c, &wire_pairs, score_add)
}

fn find_fix<F>(c: &Circuit, f_score: F) -> String
where
    F: FnMut(&Circuit) -> Option<usize>,
{
    let wire_pairs = c.wire_pairs().collect::<Vec<_>>();
    find_fix_impl(c, &wire_pairs, f_score)
}

fn find_fix_impl<F>(c: &Circuit, wire_pairs: &[(Wire, Wire)], mut f_score: F) -> String
where
    F: FnMut(&Circuit) -> Option<usize>,
{
    let Some(score) = f_score(c) else {
        return String::from("empty");
    };
    const MSCORE: usize = 3;
    let score = MSCORE * score;

    const MAX_SWAPS: usize = 4;
    let mut vis = HashSet::<SwapSet>::new();
    let mut work = BinaryHeap::from([HeapEntry::new(score, SwapSet::new(), c.clone(), 0)]);
    let mut counter = 0;
    while let Some(HeapEntry {
        score,
        swaps,
        circuit,
        wire_idx,
    }) = work.pop()
    {
        if let Some((next_wire_idx, a, b, new_swaps)) = wire_pairs[wire_idx..]
            .iter()
            .enumerate()
            .find_map(|(idx, (a, b))| {
                let new_swaps = swaps.with(*a, *b)?;
                (!vis.contains(&new_swaps)).then_some((wire_idx + idx + 1, *a, *b, new_swaps))
            })
        {
            let mut c = circuit.clone();
            c.swap(a, b);

            let new_score = match f_score(&c) {
                Some(n) => n,
                None => {
                    // exact match
                    println!("exact");
                    return new_swaps.wires().fold(String::new(), |acc, wire| {
                        let sep = if acc.is_empty() { "" } else { "," };
                        acc + &format!("{}{}", sep, wire.as_str())
                    });
                }
            };
            let new_score = if new_score > 0 {
                MSCORE * new_score - new_swaps.len()
            } else {
                0
            };

            println!("  {counter:6}  {a}-{b}  {new_score:5}  {new_swaps:?}");
            counter += 1;
            if new_swaps.len() < 2 * MAX_SWAPS {
                work.push(HeapEntry::new(new_score, new_swaps.clone(), c, 0));
                vis.insert(new_swaps);
            }
            if next_wire_idx < wire_pairs.len() {
                work.push(HeapEntry::new(score, swaps, circuit, next_wire_idx));
            }
        }
    }
    String::from("invalid")
}

#[derive(Debug, Clone)]
struct HeapEntry {
    score: usize,
    swaps: SwapSet,
    circuit: Circuit,
    wire_idx: usize,
}

impl HeapEntry {
    fn new(score: usize, swaps: SwapSet, circuit: Circuit, wire_idx: usize) -> Self {
        Self {
            score,
            swaps,
            circuit,
            wire_idx,
        }
    }
}

impl std::cmp::Eq for HeapEntry {}

impl std::cmp::PartialEq for HeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl std::cmp::Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl std::cmp::PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
fn score_and(c: &Circuit) -> Option<usize> {
    let nbits = c.output_bits();
    if nbits == 0 {
        return None;
    }
    (0..(nbits - 1))
        .fold(ScoreImpl::new(), |acc, i| {
            let m = 1 << i;
            acc.fold(c, nbits, m, m, m)
                .fold(c, nbits, m, 0, 0)
                .fold(c, nbits, m, m - 1, 0)
        })
        .keep_going_score()
}

fn score_add(c: &Circuit) -> Option<usize> {
    let mut n_all = 0;
    let mut n_ok = 0;
    foreach_add_bit(c, |_, ok| {
        n_all += 1;
        n_ok += ok as usize;
    });
    (n_all != n_ok).then_some(n_ok)
}

fn score_add_0(c: &Circuit) -> Option<usize> {
    let nbits = c.output_bits();
    if nbits == 0 {
        return None;
    }
    (0..(nbits - 1))
        .fold(ScoreImpl::new(), |acc, i| {
            let m = 1 << i;
            acc.fold(c, nbits, m, m, m + m)
                .fold(c, nbits, m - 1, m - 1, (m - 1) + (m - 1))
                .fold(c, nbits, m - 1, m, m + (m - 1))
        })
        .keep_going_score()
}

#[derive(Debug, Copy, Clone)]
struct ScoreImpl {
    len: u8,
    match_: u8,
    valid: u8,
}

impl ScoreImpl {
    fn new() -> Self {
        Self {
            len: 0,
            match_: 0,
            valid: 0,
        }
    }

    fn fold(self, c: &Circuit, nbits: u32, a: u64, b: u64, want: u64) -> Self {
        let Self {
            mut len,
            mut match_,
            mut valid,
        } = self;
        len += 1;
        if let Some(o) = c.output(a, b, nbits) {
            valid += 1;
            if o == want {
                match_ += 1;
            }
        }
        Self { len, match_, valid }
    }

    fn keep_going_score(&self) -> Option<usize> {
        if self.match_ == self.len {
            None
        } else {
            Some((self.match_ as usize) * 100 + (self.valid as usize))
        }
    }
}

/*
struct PackedCircuit {
    zrng: Range<u16>,
    gate: Vec<PackedGate>,

}

impl PackedCircuit {
    fn bit(&self, x: u64, y: u64) {

    }
}

#[derive(Debug, Copy, Clone)]
enum PackGate {
    Input(bool),
    And(u16, u16),
    Or(u16, u16),
    Xor(u16, u16),
}
*/

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

    fn deps(&self, wire: Wire) -> HashSet<Wire> {
        let mut r = HashSet::new();
        self.deps_impl(&mut r, wire);
        r
    }

    fn deps_impl(&self, set: &mut HashSet<Wire>, wire: Wire) {
        if let Some(g) = self.0.get(&wire) {
            if set.insert(wire) {
                self.deps_impl(set, g.a);
                self.deps_impl(set, g.b);
            }
        }
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

    fn x(n: u32) -> Self {
        Self::from_char_num('x', n)
    }

    fn y(n: u32) -> Self {
        Self::from_char_num('y', n)
    }

    fn z(n: u32) -> Self {
        Self::from_char_num('z', n)
    }

    fn from_char_num(c: char, n: u32) -> Self {
        if n >= 100 {
            panic!("invalid wire number: {n}");
        }
        let hi = b'0' + (n / 10) as u8;
        let lo = b'0' + (n % 10) as u8;
        Self([c as u8, hi, lo])
    }

    fn char_num(&self) -> Option<(char, u32)> {
        let n = self.as_str()[1..].parse().ok();
        n.map(|n| (self.0[0] as char, n))
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
        assert_eq!(score_and(&c3), None);
        assert_eq!(&find_fix(&p3.circuit, score_and), "z00,z01,z02,z05");
    }
}

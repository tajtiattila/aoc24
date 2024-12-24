use anyhow::anyhow;
use std::collections::HashMap;
use std::fmt;

pub fn run(input: &str) -> anyhow::Result<String> {
    let p = Problem::parse(input)?;
    // 3302442726 too low
    let s1 = star1(&p);
    let s2 = p.circuit.len();
    Ok(format!("{s1} {s2}"))
}

fn star1(p: &Problem) -> u64 {
    (0..u64::BITS).fold(0, |mut acc, i| {
        let wire = Wire::parse(&format!("z{:02}", i)).unwrap();
        if p.bit(wire) == Some(true) {
            acc |= 1 << i;
        }
        acc
    })
}

struct Problem {
    wires: HashMap<Wire, bool>,
    circuit: HashMap<Wire, Gate>,
}

impl Problem {
    fn parse(input: &str) -> anyhow::Result<Problem> {
        let mut it = input.lines();

        let mut wires = HashMap::new();
        for line in it.by_ref() {
            if line.is_empty() {
                break;
            }
            let (l, r) = line
                .split_once(": ")
                .ok_or_else(|| anyhow!("invalid: {line}"))?;
            let wire = Wire::parse(l).ok_or_else(|| anyhow!("invalid: {line}"))?;
            let value = r
                .parse::<u8>()
                .map(|n| n != 0)
                .map_err(anyhow::Error::msg)?;
            wires.insert(wire, value);
        }

        let mut circuit = HashMap::new();
        for line in it {
            let mut it = line.split_ascii_whitespace();
            let inv = || anyhow!("invalid: {line}");
            let a = it
                .next()
                .and_then(|part| Wire::parse(part))
                .ok_or_else(inv)?;
            let op = it.next().and_then(|part| Op::parse(part)).ok_or_else(inv)?;
            let b = it
                .next()
                .and_then(|part| Wire::parse(part))
                .ok_or_else(inv)?;
            if it.next() != Some("->") {
                return Err(inv());
            }
            let out = it
                .next()
                .and_then(|part| Wire::parse(part))
                .ok_or_else(inv)?;
            circuit.insert(out, Gate { a, b, op });
        }

        Ok(Problem { wires, circuit })
    }

    fn bit(&self, wire: Wire) -> Option<bool> {
        if let Some(x) = self.wires.get(&wire) {
            return Some(*x);
        }
        let g = self.circuit.get(&wire)?;
        let a = self.bit(g.a)?;
        let b = self.bit(g.b)?;
        Some(g.result(a, b))
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
        match self {
            &And => "AND",
            &Or => "OR",
            &Xor => "XOR",
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

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
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

        let s1 = |input| star1(&Problem::parse(input).unwrap());
        assert_eq!(s1(sample1), 4);
        assert_eq!(s1(sample2), 2024);
    }
}

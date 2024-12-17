use crate::Cli;
use anyhow::{anyhow, Result};
use std::collections::VecDeque;

pub fn run(input: &str) -> Result<String> {
    let (cpu, prog) = parse_input(input)?;
    let s1 = run1(&cpu, &prog);

    let s2 = run2(&prog);
    Ok(format!("{s1} {s2}"))
}

fn run1(cpu: &CpuState, prog: &[u8]) -> String {
    let mut cpu = *cpu;

    let mut r = String::new();
    while let Some(op) = cpu.op(prog) {
        if let Some(d) = cpu.exec(op) {
            if !r.is_empty() {
                r.push(',');
            }
            r.push(char::from_digit(d.into(), 10).unwrap());
        }
    }
    r
}

struct Node {
    // value to check
    a: Register,

    // Mask of single bit to check
    update_mask: Register,

    // Mask for the register that must match program bits
    check_mask: Register,
}

fn run2(prog: &[u8]) -> String {
    run2_impl(prog, Cli::global().verbose)
}

fn run2_impl(prog: &[u8], verbose: bool) -> String {
    let prog_bits = prog.iter().rev().fold(0, |acc, &d| (acc << 3) | (d as u64));
    let prog_mask = (1 << (prog.len() * 3)) - 1;

    const START_BIT: usize = 12;
    let mut work = VecDeque::new();
    for a in 0..1 << START_BIT {
        work.push_back(Node {
            a,
            update_mask: 1 << START_BIT,
            check_mask: 1,
        });
    }

    let show = |n: &Node, bits: Register, ok: bool| {
        if verbose {
            print!("{0:16} {0:20o}", n.a);
            if ok {
                println!(" ok");
            } else {
                let mg = bits & n.check_mask;
                let mw = prog_bits & n.check_mask;
                let digits = (n.check_mask.count_ones() / 3) + 1;
                println!(" {:w$o} != {:w$o}", mg, mw, w = digits as usize);
            }
        }
    };

    while let Some(n) = work.pop_front() {
        let (bits, len) = oct(n.a, prog);
        if len == prog.len() && (bits & prog_mask) == prog_bits {
            return format!("{}", n.a);
        }
        if (bits ^ prog_bits) & n.check_mask == 0 {
            show(&n, bits, true);
            let a1 = n.a | n.update_mask;
            let update_mask = n.update_mask << 1;
            let check_mask = (n.check_mask << 1) | 1;
            work.push_back(Node {
                a: n.a,
                update_mask,
                check_mask,
            });
            work.push_back(Node {
                a: a1,
                update_mask,
                check_mask,
            });
        } else {
            show(&n, bits, false);
        }
    }

    String::from("failed")
}

fn oct(a: Register, prog: &[u8]) -> (u64, usize) {
    let mut cpu = CpuState::new_a(a);
    let mut r = 0u64;
    let mut digits = 0;
    while let Some(op) = cpu.op(prog) {
        if let Some(d) = cpu.exec(op) {
            r |= (d as u64) << (digits * 3);
            digits += 1;
        }
    }
    (r, digits)
}

type Register = u64;

#[derive(Debug, Copy, Clone, Default)]
struct CpuState {
    ip: usize,
    a: Register,
    b: Register,
    c: Register,
}

impl CpuState {
    fn new_a(a: Register) -> Self {
        Self {
            a,
            ..Self::default()
        }
    }

    fn op(&self, prog: &[u8]) -> Option<Op> {
        if prog.len() < self.ip + 2 {
            return None;
        }
        let c = prog[self.ip];
        let v = prog[self.ip + 1];
        Some(match c {
            0 => Op::Adv(v),
            1 => Op::Bxl(v),
            2 => Op::Bst(v),
            3 => Op::Jnz(v),
            4 => Op::Bxc,
            5 => Op::Out(v),
            6 => Op::Bdv(v),
            7 => Op::Cdv(v),
            _ => {
                return None;
            }
        })
    }

    fn exec(&mut self, op: Op) -> Option<u8> {
        match op {
            Op::Adv(x) => {
                self.a >>= self.combo(x);
                self.ip += 2;
            }
            Op::Bxl(x) => {
                self.b ^= x as Register;
                self.ip += 2;
            }
            Op::Bst(x) => {
                self.b = self.combo(x) & 7;
                self.ip += 2;
            }
            Op::Jnz(x) => {
                if self.a != 0 {
                    self.ip = x as usize;
                } else {
                    self.ip += 2;
                }
            }
            Op::Bxc => {
                self.b ^= self.c;
                self.ip += 2;
            }
            Op::Out(x) => {
                self.ip += 2;
                return Some((self.combo(x) & 7) as u8);
            }
            Op::Bdv(x) => {
                self.b = self.a >> self.combo(x);
                self.ip += 2;
            }
            Op::Cdv(x) => {
                self.c = self.a >> self.combo(x);
                self.ip += 2;
            }
        }
        None
    }

    fn combo(self, v: u8) -> Register {
        match v {
            0 => 0 as Register,
            1 => 1 as Register,
            2 => 2 as Register,
            3 => 3 as Register,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => 0 as Register,
        }
    }
}

fn parse_input(input: &str) -> Result<(CpuState, Vec<u8>)> {
    let mut it = input.lines().filter_map(|s| {
        let (_, r) = s.split_once(':')?;
        Some(r.trim())
    });
    let mut nxt = || it.next().ok_or_else(|| anyhow!("invalid input"));
    let a: Register = nxt()?.parse()?;
    let b: Register = nxt()?.parse()?;
    let c: Register = nxt()?.parse()?;

    let v = nxt()?
        .split(',')
        .map(|s| s.parse::<u8>().map_err(|e| e.into()))
        .collect::<Result<Vec<_>>>()?;

    Ok((CpuState { a, b, c, ip: 0 }, v))
}

enum Op {
    Adv(u8),
    Bxl(u8),
    Bst(u8),
    Jnz(u8),
    Bxc,
    Out(u8),
    Bdv(u8),
    Cdv(u8),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn s1_works() {
        let input = r#"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"#
        .trim();

        let (cpu, prog) = parse_input(input).unwrap();
        assert_eq!(run1(&cpu, &prog), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn s2_works() {
        let input = r#"
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"#
        .trim();

        let (_, prog) = parse_input(input).unwrap();
        assert_eq!(run2_impl(&prog, true), "117440");
    }
}

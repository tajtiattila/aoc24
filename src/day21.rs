use std::collections::{HashSet, VecDeque};

pub fn run(input: &str) -> anyhow::Result<String> {
    let s1 = star1(input);
    let s2 = "";
    Ok(format!("{s1} {s2}"))
}

fn star1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let n: usize = line.trim_end_matches('A').parse().ok()?;
            let m = key_push_count(line)?;
            Some(n * m)
        })
        .sum()
}

fn key_push_count(digits: &str) -> Option<usize> {
    let mut vis = HashSet::new();
    let mut work = VecDeque::from([(State::new(), 0)]);
    while let Some((s, count)) = work.pop_front() {
        let next_count = count + 1;
        for c in DIRPAD.chars() {
            if let Some(s) = s.step(c) {
                if digits.starts_with(s.output()) && vis.insert(s) {
                    if digits == s.output() {
                        return Some(next_count);
                    }
                    work.push_back((s, next_count));
                }
            }
        }
    }
    None
}

// 379A
// ^A<<^^A>>AvvvA
// <A>Av<<AA>^AA>AvAA^A<vAAA>^A
// v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^AvA^AA<A>Av<<A>A>^AAAvA<^A>A     .

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
static NUMPAD: &str = "789456123*0A";

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
static DIRPAD: &str = "*^A<v>";

type PadPos = u16;

fn star1_entry(digits: &str) -> usize {
    let s0 = numpad_entry(digits);
    let s1 = dirpad_entry(&s0);
    let s2 = dirpad_entry(&s1);
    s2.len()
}

fn keypad_entry(keypad: &str, digits: &str) -> String {
    let mut out = String::new();
    let mut p = keypad.find('A').unwrap();
    for q in digits.chars().filter_map(|c| keypad.find(c)) {
        let px = (p%3) as i32;
        let py = (p/3) as i32;
        let qx = (q%3) as i32;
        let qy = (q/3) as i32;
        if px < qx {
            for _ in px..qx {
                out.push('>');
            }
        }
        if py < qy {
            for _ in py..qy {
                out.push('v');
            }
        } else {
            for _ in qy..py {
                out.push('^');
            }
        }
        if qx < px {
            for _ in qx..px {
                out.push('<');
            }
        }
        out.push('A');
        p = q;
    }
    out
}
fn numpad_entry(digits: &str) -> String {
    keypad_entry(NUMPAD, digits)
}

fn dirpad_entry(moves: &str) -> String {
    keypad_entry(DIRPAD, moves)
}

/*
 * 029A
 *
 * v<<A>>^A<AA>
 * <A^A
 *
 * <A^A^^>AvvvA
 *
 * < : v<<A
 * ^ : <A
 * v : v<A
 * > : vA
 */
fn dirpad_move_ack(p0: PadPos, p1: PadPos, avoid: PadPos) {
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    keys: [u8; 8],
    ikey: u16,
    r0: u16,
    r1: u16,
    r2: u16,
}

impl State {
    fn new() -> Self {
        let np = NUMPAD.find('A').unwrap() as u16;
        let dp = DIRPAD.find('A').unwrap() as u16;

        Self {
            keys: [b'\0'; 8],
            ikey: 0,
            r0: dp,
            r1: dp,
            r2: np,
        }
    }

    fn output(&self) -> &str {
        std::str::from_utf8(&self.keys[0..self.ikey as usize]).unwrap()
    }

    fn step(&self, c0: char) -> Option<Self> {
        let mut next = *self;
        match push_chain(
            c0,
            &mut [
                (&mut next.r0, DIRPAD),
                (&mut next.r1, DIRPAD),
                (&mut next.r2, NUMPAD),
            ],
        ) {
            PushResult::Ack(c) => {
                next.keys[next.ikey as usize] = c as u8;
                next.ikey += 1;
                Some(next)
            }
            PushResult::Valid => {
                Some(next)
            }
            PushResult::Invalid => {
                None
            }
        }
    }
}

enum PushResult {
    Invalid,
    Valid,
    Ack(char),
}

fn push_chain(mut c: char, robots: &mut [(&mut u16, &str)]) -> PushResult {
    use PushResult::Ack;
    for (robot, keypad) in robots {
        let r = push(robot, keypad, c);
        c = match r {
            Ack(c) => c,
            _ => {
                return r;
            }
        }
    }
    Ack(c)
}

fn push(p: &mut u16, keypad: &str, c: char) -> PushResult {
    use PushResult::*;
    let keypad = keypad.as_bytes();
    let (mut x, mut y) = (*p % 3, *p / 3);
    match c {
        '>' => {
            if x < 2 {
                x += 1;
            } else {
                return Invalid;
            }
        }
        '<' => {
            if x > 0 {
                x -= 1;
            } else {
                return Invalid;
            }
        }
        '^' => {
            if y > 0 {
                y -= 1;
            } else {
                return Invalid;
            }
        }
        'v' => {
            y += 1;
        }
        'A' => {
            return Ack(keypad[*p as usize] as char);
        }
        _ => {
            return Invalid;
        }
    }
    let q = x + 3 * y;
    let qs = q as usize;
    if qs < keypad.len() && keypad[qs] != b'*' {
        *p = q;
        Valid
    } else {
        Invalid
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn try_keys(keypresses: &str) -> Option<String> {
        keypresses
            .chars()
            .try_fold(State::new(), |s, c| s.step(c))
            .map(|s| s.output().to_owned())
    }

    #[test]
    fn it_works() {
        assert_eq!(
            try_keys("<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"),
            Some(String::from("029A"))
        );

        let sample = r#"
029A
980A
179A
456A
379A
"#
        .trim();

        let s0 = numpad_entry("029A");
        assert_eq!(&s0, "<A^A>^^AvvvA");
        let s1 = dirpad_entry(&s0);
        assert_eq!(&s1, "v<<A>>^A<A>AvA^<AA>Av<AAA>^A");
        let s2 = dirpad_entry(&s1);
        assert_eq!(s2.len(), 68);

        assert_eq!(star1_entry("029A"), 68);
        assert_eq!(star1_entry("980A"), 60);
        assert_eq!(star1_entry("179A"), 68);
        assert_eq!(star1_entry("456A"), 64);

        let s0 = numpad_entry("379A");
        assert_eq!(&s0, "^A^^<<A>>AvvvA");
        let s1 = dirpad_entry(&s0);
        assert_eq!(&s1, "<A>A<AAv<AA>>^AvAA^Av<AAA>^A");
        let s2 = dirpad_entry(&s1);
        //  <A>A<AAv<AA >>^AvAA^Av<AAA>^A
        assert_eq!(&s2, "v<<A>>^AvA^Av<<A>>^AAv<A<A>>^AA");
        // v<<A>>^AvA^Av<<A>>^AAv<A<A>>^AAvAA^<A>Av<A>^AA<A>Av<A<A>>^AAAvA^<A>A 

        assert_eq!(star1_entry("379A"), 64);

        assert_eq!(star1(sample), 126384);
    }
}

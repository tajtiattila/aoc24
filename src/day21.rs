use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

pub fn run(input: &str) -> anyhow::Result<String> {
    sim_rec("379A", 8);
    let s1 = star1(input);
    let s2 = "";
    Ok(format!("{s1} {s2}"))
}

fn star1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let n: usize = line.trim_end_matches('A').parse().ok()?;
            let m = star1_moves(line);
            Some(n * m)
        })
        .sum()
}

fn key_push_count(digits: &str) -> Option<(usize, String)> {
    let mut vis = HashSet::new();
    let mut work: VecDeque<(State, usize, Option<Rc<Link>>)> =
        VecDeque::from([(State::new(), 0, None)]);
    while let Some((s, count, pred)) = work.pop_front() {
        let next_count = count + 1;
        for c in DIRPAD.chars() {
            if let Some(s) = s.step(c) {
                if digits.starts_with(s.output()) && vis.insert(s) {
                    let node = Link::link(c, &pred);
                    if digits == s.output() {
                        return Some((next_count, Link::to_string(&node)));
                    }
                    work.push_back((s, next_count, node));
                }
            }
        }
    }
    None
}

struct Link {
    c: char,
    pred: Option<Rc<Link>>,
}

impl Link {
    fn link(c: char, pred: &Option<Rc<Link>>) -> Option<Rc<Link>> {
        Some(Rc::new(Link {
            c,
            pred: pred.clone(),
        }))
    }

    fn to_string(mut node: &Option<Rc<Link>>) -> String {
        let mut v = vec![];
        while let Some(rc) = node {
            v.push(rc.c);
            node = &rc.pred;
        }
        v.reverse();
        v.into_iter().collect()
    }
}

fn sim(keypad: &str, input: &str) -> String {
    let mut s = String::new();
    let mut p = keypad.find('A').unwrap();
    let dy = (keypad.len() / 3) as i32;
    for c in input.chars() {
        let mut px = (p % 3) as i32;
        let mut py = (p / 3) as i32;
        match c {
            '<' => {
                px -= 1;
            }
            '>' => {
                px += 1;
            }
            '^' => {
                py -= 1;
            }
            'v' => {
                py += 1;
            }
            'A' => {
                s.push(keypad.as_bytes()[p] as char);
            }
            _ => {}
        }
        if (0..3).contains(&px) && (0..dy).contains(&py) {
            p = (px + py * 3) as usize;
        }
    }
    s
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
static NUMPAD: &str = "....789.456.123..0A....";

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
static DIRPAD: &str = ".....^A.<v>....";

#[derive(Copy, Clone, Debug)]
struct Movement {
    dx: i32,
    dy: i32,
    x_first: bool,
}

impl Movement {
    fn move_chars(&self) -> (&'static str, &'static str) {
        if self.x_first {
            (self.dx_chars(), self.dy_chars())
        } else {
            (self.dy_chars(), self.dx_chars())
        }
    }

    fn dx_chars(&self) -> &'static str {
        let x = self.dx;
        if x < 0 {
            &"<<<<<"[..(-x) as usize]
        } else if x > 0 {
            &">>>>>"[..x as usize]
        } else {
            ""
        }
    }

    fn dy_chars(&self) -> &'static str {
        let y = self.dy;
        if y < 0 {
            &"^^^^^"[..(-y) as usize]
        } else if self.dy > 0 {
            &"vvvvv"[..y as usize]
        } else {
            ""
        }
    }
}

fn numpad_moves(digits: &str) -> impl Iterator<Item = char> + use<'_> {
    keypad_moves(NUMPAD, digits.chars())
}

fn dirpad_moves(input: impl Iterator<Item = char>) -> impl Iterator<Item = char> {
    keypad_moves(DIRPAD, input)
}

fn keypad_moves<I: Iterator<Item = char>>(keypad: &str, input: I) -> impl Iterator<Item = char> + use<'_, I> {
    let p = keypad.find('A').unwrap();
    input
        .scan(p, move |state, c| {
            let p = *state;
            let q = keypad.find(c)?;
            *state = q;
            let (px, py) = ((p % 4) as i32, (p / 4) as i32);
            let (qx, qy) = ((q % 4) as i32, (q / 4) as i32);
            let dx = qx - px;
            let dy = qy - py;
            // char under arm when moving x first
            let qxc = keypad.as_bytes()[(qx + py * 4) as usize];
            // char under arm when moving y first
            let qyc = keypad.as_bytes()[(px + qy * 4) as usize];
            let x_first = (dx < 0 && qxc != b'.') || qyc == b'.';
            Some(movement_chars(dx, dy, x_first))
        })
        .flatten()
}

fn movement_chars(x: i32, y: i32, x_first: bool) -> impl Iterator<Item = char> {
    let xs = if x < 0 {
        &"<<<<<"[..(-x) as usize]
    } else if x > 0 {
        &">>>>>"[..x as usize]
    } else {
        ""
    };
    let ys = if y < 0 {
        &"^^^^^"[..(-y) as usize]
    } else if y > 0 {
        &"vvvvv"[..y as usize]
    } else {
        ""
    };
    let (l, r) = if x_first { (xs, ys) } else { (ys, xs) };
    l.chars().chain(r.chars()).chain(std::iter::once('A'))
}

fn star1_moves(digits: &str) -> usize {
    numpad_moves(digits)
        .apply(dirpad_moves)
        .apply(dirpad_moves)
        .count()
}

fn sim_rec(digits: &str, n: usize) {
    let mut s = String::from(digits);
    println!("0: {}", s);
    let mut keypad = NUMPAD;
    for i in 0..n {
        let next = keypad_moves(keypad, s.chars()).collect();
        keypad = DIRPAD;
        s = next;
        println!("{}: {}", i+1, s);
    }
    println!();
}

// https://docs.rs/apply/latest/src/apply/lib.rs.html
pub trait Apply<Res> {
    fn apply<F: FnOnce(Self) -> Res>(self, f: F) -> Res where Self: Sized {
        f(self)
    }
}

impl<T: ?Sized, Res> Apply<Res> for T { }

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
        let px = (p % 4) as i32;
        let py = (p / 4) as i32;
        let qx = (q % 4) as i32;
        let qy = (q / 4) as i32;
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
            PushResult::Valid => Some(next),
            PushResult::Invalid => None,
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
    let q = match c {
        '>' => *p + 1,
        '<' => *p - 1,
        '^' => *p - 4,
        'v' => *p + 4,
        'A' => {
            return Ack(keypad[*p as usize] as char);
        }
        _ => {
            return Invalid;
        }
    };
    let qs = q as usize;
    if qs < keypad.len() && keypad[qs] != b'.' {
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
        assert_eq!(star1_moves("029A"), 68);
        assert_eq!(star1_moves("980A"), 60);
        assert_eq!(star1_moves("179A"), 68);
        assert_eq!(star1_moves("456A"), 64);
        assert_eq!(star1_moves("379A"), 64);

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

        assert_eq!(star1(sample), 126384);
        //----------------------------------------------------------------

        let s0 = numpad_entry("029A");
        assert_eq!(&s0, "<A^A>^^AvvvA");
        let s1 = dirpad_entry(&s0);
        assert_eq!(&s1, "v<<A>>^A<A>AvA^<AA>Av<AAA>^A");
        let s2 = dirpad_entry(&s1);
        assert_eq!(s2.len(), 68);

        let npc = |digits| numpad_moves(digits).collect::<String>();
        assert_eq!(&npc("029A"), "<A^A^^>AvvvA");
        assert_eq!(&npc("379A"), "^A<<^^A>>AvvvA");

        let s = key_push_count("379A").unwrap().1;
        println!("{}", s);
        let s1 = sim(DIRPAD, &s);
        println!("{}", s1);
        println!("{}", sim(DIRPAD, &s1));
        println!();
        let z = "v<<A>>^AvA^Av<<A>>^AAv<A<A>>^AAvAA^<A>Av<A>^AA<A>Av<A<A>>^AAAvA^<A>A";
        println!("{}", z);
        let z1 = sim(DIRPAD, &z);
        println!("{}", z1);
        println!("{}", sim(DIRPAD, &z1));

        let s0 = numpad_entry("379A");
        assert_eq!(&s0, "^A^^<<A>>AvvvA");
        let s1 = dirpad_entry(&s0);
        assert_eq!(&s1, "<A>A<AAv<AA>>^AvAA^Av<AAA>^A");
        let s2 = dirpad_entry(&s1);
        //  <A>A<AAv<AA >>^AvAA^Av<AAA>^A
        assert_eq!(&s2, "v<<A>>^AvA^Av<<A>>^AAv<A<A>>^AA");
        // v<<A>>^AvA^Av<<A>>^AAv<A<A>>^AAvAA^<A>Av<A>^AA<A>Av<A<A>>^AAAvA^<A>A
    }
}

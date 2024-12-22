use std::cmp::Ordering;
use std::collections::HashMap;

pub fn run(input: &str) -> anyhow::Result<String> {
    let s1 = star(1, input);
    let s2 = star(2, input);
    Ok(format!("{s1} {s2}"))
}

fn star(num: usize, input: &str) -> usize {
    let km = match num {
        1 => analyze(DIRPAD, 0),
        2 => analyze(DIRPAD, 23),
        _ => panic!("invalid star number"),
    }
    .expect("dirpad analysis failed");

    input
        .lines()
        .filter_map(|line| {
            let n: usize = line.trim_end_matches('A').parse().ok()?;
            let d = dirpad_moves(numpad_moves(line)).collect::<String>();
            let m = km.input_count(&d)?;
            Some(n * m)
        })
        .sum()
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

fn numpad_moves(digits: &str) -> impl Iterator<Item = char> + use<'_> {
    keypad_moves(NUMPAD, digits.chars())
}

fn dirpad_moves(input: impl Iterator<Item = char>) -> impl Iterator<Item = char> {
    keypad_moves(DIRPAD, input)
}

fn keypad_moves<I: Iterator<Item = char>>(
    keypad: &str,
    input: I,
) -> impl Iterator<Item = char> + use<'_, I> {
    let p = keypad.find('A').unwrap();
    input
        .scan(p, move |state, c| {
            let (q, dx, dy, x_first) = keypad_move(keypad, *state, c)?;
            *state = q;
            Some(movement_chars(dx, dy, x_first))
        })
        .flatten()
}

fn keypad_move(keypad: &str, from_pos: usize, to_key: char) -> Option<(usize, i32, i32, bool)> {
    let p = from_pos;
    let q = keypad.find(to_key)?;
    let (px, py) = ((p % 4) as i32, (p / 4) as i32);
    let (qx, qy) = ((q % 4) as i32, (q / 4) as i32);
    let dx = qx - px;
    let dy = qy - py;
    // char under arm when moving x first
    let qxc = keypad.as_bytes()[(qx + py * 4) as usize];
    // char under arm when moving y first
    let qyc = keypad.as_bytes()[(px + qy * 4) as usize];
    let x_first = (dx < 0 && qxc != b'.') || qyc == b'.';
    Some((q, dx, dy, x_first))
}

fn movement_chars(x: i32, y: i32, x_first: bool) -> impl Iterator<Item = char> {
    use Ordering::*;
    let xs = match x.cmp(&0) {
        Less => &"<<<<<"[..(-x) as usize],
        Greater => &">>>>>"[..x as usize],
        Equal => "",
    };
    let ys = match y.cmp(&0) {
        Less => &"^^^^^"[..(-y) as usize],
        Greater => &"vvvvv"[..y as usize],
        Equal => "",
    };
    let (l, r) = if x_first { (xs, ys) } else { (ys, xs) };
    l.chars().chain(r.chars()).chain(std::iter::once('A'))
}

fn move_sets(keypad: &str) -> Vec<String> {
    let mut v = vec![String::from('A')];
    for (p, cp) in keypad.chars().enumerate().filter(|(_, c)| *c != '.') {
        for cq in keypad.chars().filter(|c| *c != '.' && *c != cp) {
            let (_, dx, dy, x_first) = keypad_move(keypad, p, cq).unwrap();
            v.push(movement_chars(dx, dy, x_first).collect());
        }
    }
    v
}

#[derive(Debug, Clone)]
struct KeypadMap(HashMap<String, usize>);

impl KeypadMap {
    fn from_keypad(keypad: &str) -> Self {
        Self(
            move_sets(keypad)
                .into_iter()
                .map(|s| {
                    let n = keypad_moves(keypad, s.chars()).count();
                    (s, n)
                })
                .collect(),
        )
    }

    fn input_count(&self, input: &str) -> Option<usize> {
        input
            .split_inclusive('A')
            .try_fold(0, |acc, part| Some(acc + self.0.get(part)?))
    }

    fn next_level(&self, keypad: &str) -> Option<Self> {
        Some(Self(
            self.0
                .keys()
                .map(|s0| {
                    let s1 = keypad_moves(keypad, s0.chars()).collect::<String>();
                    self.input_count(&s1).map(|n1| (s0.clone(), n1))
                })
                .collect::<Option<HashMap<String, usize>>>()?,
        ))
    }
}

fn analyze(keypad: &str, depth: usize) -> Option<KeypadMap> {
    let mut m = KeypadMap::from_keypad(keypad);
    for _ in 0..depth {
        m = m.next_level(keypad)?;
    }
    Some(m)
}

#[cfg(test)]
mod test {
    use super::*;

    fn star1_moves(digits: &str) -> usize {
        dirpad_moves(dirpad_moves(numpad_moves(digits))).count()
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

    fn sim_dir(input: &str, depth: usize) -> String {
        let mut s = String::from(input);
        for _ in 0..depth {
            let next = dirpad_moves(s.chars()).collect();
            s = next;
        }
        s
    }

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

        assert_eq!(star(1, sample), 126384);

        let sim_eq_from_parts = |a, b, n| -> bool {
            let l = sim_dir(a, n) + &sim_dir(b, n);
            let r = sim_dir(&format!("{a}{b}"), n);
            l == r
        };
        assert!(sim_eq_from_parts("<<A", "^^A", 8));

        let m1 = analyze(DIRPAD, 0).unwrap();
        println!("{m1:?}");

        let check1 = |digits| {
            let d = dirpad_moves(numpad_moves(digits)).collect::<String>();
            println!("{d}");
            m1.input_count(&d)
        };
        assert_eq!(check1("029A"), Some(68));
        assert_eq!(check1("980A"), Some(60));
        assert_eq!(check1("179A"), Some(68));
        assert_eq!(check1("456A"), Some(64));
        assert_eq!(check1("379A"), Some(64));
    }
}

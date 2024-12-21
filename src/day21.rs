pub fn run(input: &str) -> anyhow::Result<String> {
    let s1 = star1(input);
    let s2 = "";
    Ok(format!("{s1} {s2}"))
}

fn star1(input: &str) -> usize {
    0
}

fn key_push_count(digits: &str) -> usize {
    0
}

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

#[derive(Copy, Clone)]
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

    fn str(&self) -> &str {
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
                return Some(next);
            }
            PushResult::Valid => {
                return Some(next);
            }
            PushResult::Invalid => {
                return None;
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
        let r = push(*robot, keypad, c);
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

    fn run(keypresses: &str) -> Option<String> {
        keypresses
            .chars()
            .try_fold(State::new(), |s, c| s.step(c))
            .map(|s| s.str().to_owned())
    }

    #[test]
    fn it_works() {
        assert_eq!(
            run("<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"),
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

        assert_eq!(key_push_count("029A"), 68);
        assert_eq!(key_push_count("980A"), 60);
        assert_eq!(key_push_count("179A"), 68);
        assert_eq!(key_push_count("456A"), 64);
        assert_eq!(key_push_count("379A"), 64);

        assert_eq!(star1(sample), 126384);
    }
}

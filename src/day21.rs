use crate::grid::Point;

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

#[derive(Copy, Clone)]
struct Keypad<'a> {
    buttons: &'a str,
    dy: usize,
}

impl<'a> Keypad<'a> {
    const fn new(buttons: &'a str) -> Self {
        Self{
            buttons,
            dy: buttons.len()/3,
        }
    }

    fn find_key(&self, c: char) -> Option<Point> {
        self.buttons.find(c)
            .map(|i| Point::new((i % 3) as i32, (i / 3) as i32))
    }

    fn key_at(&self, p: u8) -> Option<char> {
        let (x, y) = p.xy();
        let c = if (0..3).contains(x) && (0..self.dy as i32).contains(y) {
            self.buttons[(x + 3*y) as usize]
        } else {
            return None;
        }
        (c != '*').then_some(c)
    }
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
struct Robot {
    keypad: Keypad<'static>,
    p: Point,
}

impl Robot {
    fn new(keypad: &'static Keypad) -> Self{
        Self {
            keypad,
            p: keypad.find_key('A').unwrap(),
        }
    }
}

#[derive(Copy, Clone)]
struct State {
    r0: u16,
    r1: u16,
    r2: u16,
    keys: [8]char,
    ikey: usize,
}

impl State {
    fn new() -> Self {
        let np = NUMPAD.find('A').unwrap();
        let dp = DIRPAD.find('A').unwrap();

        Self {
            r0: np,
            r1: dp,
            r2: dp,
            keys: ['\0'; 8],
            ikey: 0,
        }
    }

    fn step(&self, c: char) -> Option<Self> {

    }
}

enum PushResult {
    Invalid,
    Valid,
    Ack
}

fn push(&mut p: u16, keypad: &str, c: char) -> PushResult {
    let mut (x, y) = (p%3, p/3);
    match c {
        '>' => if x < 2 {
            x += 1;
        } else {
            return Invalid;
        }
        '<' => if x > 0 {
            x -= 1;
        } else {
            return Invalid;
        }
        '^' => if y > 0 {
            y -= 1;
        } else {
            return Invalid;
        }
        'v' => {
            y += 1;
        }
        'A' => { return Ack; },
        _ => { return Invalid; }
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

    #[test]
    fn it_works() {
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

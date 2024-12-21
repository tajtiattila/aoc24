use crate::grid::{pt, Point};
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn run(input: &str) -> anyhow::Result<String> {
    let s1 = star1(input);
    let s2 = "";
    Ok(format!("{s1} {s2}"))
}

fn star1(input: &str) -> usize {
    input.lines().map(|line| {
        let n: usize = line.trim_end_matches('A').parse().unwrap();
        let c = key_push_count(line);
        n*c
    }).sum()
}

fn key_push_count(digits: &str) -> usize {
    let mut h = KeyPushCount(0);
    let mut h_bind = &mut h;
    enter(digits, &mut h_bind);
    h.0
}

fn key_pushes(digits: &str) -> String {
    let mut h = KeyVec(vec![]);
    let mut h_bind = &mut h;
    enter(digits, &mut h_bind);
    h.0.into_iter().collect()
}

fn enter<H: KeypadHandler>(digits: &str, h: &mut H) {
    let dirpad = new_dirpad();
    let numpad = new_numpad();
    let mut userpad = Keypad::new(&dirpad, h);
    let mut userpad_bind = &mut userpad;
    let mut r0pad = Keypad::new(&dirpad, &mut userpad_bind);
    let mut r0pad_bind = &mut r0pad;
    let mut r1pad = Keypad::new(&numpad, &mut r0pad_bind);
    let mut r1pad_bind = &mut r1pad;
    //let mut r2pad = Keypad::new(&numpad, &mut r1pad_bind);
    //let mut r2pad_bind = &mut r2pad;
    for c in digits.chars() {
        r1pad_bind.enter(c);
    }
}

trait KeypadHandler {
    fn enter(&mut self, c: char);
}

struct KeyPushCount(usize);

impl KeypadHandler for &mut KeyPushCount {
    fn enter(&mut self, _: char) {
        self.0 += 1;
    }
}
struct KeyVec(Vec<char>);

impl KeypadHandler for &mut KeyVec {
    fn enter(&mut self, c: char) {
        self.0.push(c);
    }
}

struct Keypad<'t, 'h, H: KeypadHandler> {
    typ: &'t KeypadType,
    handler: &'h mut H,

    c: char,
}

impl<'t, 'h, H: KeypadHandler> Keypad<'t, 'h, H> {
    fn new(typ: &'t KeypadType, handler: &'h mut H) -> Self {
        Self { typ, handler, c: 'A' }
    }
}

impl<H: KeypadHandler> KeypadHandler for &mut Keypad<'_, '_, H> {
    fn enter(&mut self, to_c: char) {
        self.typ.cmds(self.handler, self.c, to_c);

        self.c = to_c;
    }
}

/*
   numeric keypad
    +---+---+---+
    | 7 | 8 | 9 |
    +---+---+---+
    | 4 | 5 | 6 |
    +---+---+---+
    | 1 | 2 | 3 |
    +---+---+---+
        | 0 | A |
        +---+---+
*/
fn new_numpad() -> KeypadType {
    KeypadType::new(false, HashMap::from([
        ('0', pt(1, 3)),
        ('A', pt(2, 3)),
        ('1', pt(0, 2)),
        ('2', pt(1, 2)),
        ('3', pt(2, 2)),
        ('4', pt(0, 1)),
        ('5', pt(1, 1)),
        ('6', pt(2, 1)),
        ('7', pt(0, 0)),
        ('8', pt(1, 0)),
        ('9', pt(2, 0)),
    ]))
}

/*
    directional keypad
        +---+---+
        | ^ | A |
    +---+---+---+
    | < | v | > |
    +---+---+---+
*/
fn new_dirpad() -> KeypadType {
    KeypadType::new(false, HashMap::from([
        ('<', pt(0, 1)),
        ('v', pt(1, 1)),
        ('>', pt(2, 1)),
        ('^', pt(1, 0)),
        ('A', pt(2, 0)),
    ]))
}

struct KeypadType {
    blank_at_top: bool,
    keys: HashMap<char, Point>,
}

impl KeypadType {
    fn new(blank_at_top: bool, keys: HashMap<char, Point>) -> Self {
        Self {
            blank_at_top,
            keys,
        }
    }

    fn cmds<H: KeypadHandler>(&self, h: &mut H, from_c: char, to_c: char) {
        let Some(p) = self.keys.get(&from_c) else { return; };
        let Some(q) = self.keys.get(&to_c) else { return };
        let mut p = *p;
        if self.blank_at_top {
            while p.y < q.y {
                h.enter('v');
                p.y += 1;
            }
        } else {
            while p.y > q.y {
                h.enter('^');
                p.y -= 1;
            }
        }
        match p.x.cmp(&q.x) {
            Ordering::Less => {
                for _ in p.x..q.x {
                    h.enter('>');
                }
            }
            Ordering::Equal => {},
            Ordering::Greater => {
                for _ in q.x..p.x {
                    h.enter('<');
                }
            }
        }
        if self.blank_at_top {
            while p.y > q.y {
                h.enter('^');
                p.y -= 1;
            }
        } else {
            while p.y < q.y {
                h.enter('v');
                p.y += 1;
            }
        }
        h.enter('A');
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

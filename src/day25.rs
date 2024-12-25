use crate::grid::{Grid, Point};

pub fn run(input: &str) -> anyhow::Result<String> {
    let schems = parse_schems(input)?;
    let s1 = star1(&schems);
    Ok(format!("{s1}"))
}

fn star1(schems: &[Schem]) -> usize {
    schems
        .iter()
        .filter(|s| s.subj == Subj::Key)
        .map(|k| {
            schems
                .iter()
                .filter(|s| s.subj == Subj::Lock && k.fits(s))
                .count()
        })
        .sum()
}

fn parse_schems(input: &str) -> anyhow::Result<Vec<Schem>> {
    input
        .split("\n\n")
        .map(|s| s.trim())
        .map(|s| s.parse())
        .collect()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Subj {
    Key,
    Lock,
}

const PIN_HEIGHT: u8 = 7;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Schem {
    subj: Subj,
    pins: [u8; 5],
}

impl Schem {
    fn fits(&self, other: &Self) -> bool {
        std::iter::zip(&self.pins, &other.pins).all(|(a, b)| a + b <= PIN_HEIGHT)
    }
}

impl std::str::FromStr for Schem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let grid = Grid::parse(s)?;
        let (dx, dy) = grid.dimensions();
        let subj = if *grid.get(Point::new(0, 0)).unwrap() == b'.' {
            Subj::Key
        } else {
            Subj::Lock
        };
        let pins = (0..dx)
            .map(|x| {
                (0..dy)
                    .filter(|y| *grid.get(Point::new(x, *y)).unwrap() == b'#')
                    .count() as u8
            })
            .collect::<Vec<u8>>()
            .as_slice()
            .try_into()?;
        Ok(Schem { subj, pins })
    }
}

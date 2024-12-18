use crate::grid::{Grid, Point, STEPS};
use std::collections::VecDeque;

pub fn run(input: &str) -> anyhow::Result<String> {
    let s1 = star1(input, 70, 1024);
    let s2 = "";
    Ok(format!("{s1} {s2}"))
}

fn star1(input: &str, dim: i32, len: usize) -> usize {
    let mut grid = input
        .lines()
        .take(len)
        .filter_map(|line| {
            let (x, y) = line.split_once(',')?;
            Some(Point::new(x.parse::<i32>().ok()?, y.parse::<i32>().ok()?))
        })
        .fold(Grid::new((dim + 1, dim + 1), b'.'), |mut grid, p| {
            if let Some(c) = grid.get_mut(p) {
                *c = b'#';
            }
            grid
        });

    let goal = Point::new(dim, dim);
    let mut stk = VecDeque::from([(Point::new(0, 0), 1)]);
    while let Some((p, n)) = stk.pop_front() {
        for &step in STEPS {
            let q = p + step;
            if q == goal {
                return n;
            }
            if let Some(qc) = grid.get_mut(q) {
                if *qc == b'.' {
                    *qc = b'O';
                    stk.push_back((q, n + 1));
                }
            }
        }
    }
    grid.show();
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let sample = r#"
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"#
        .trim();

        assert_eq!(star1(sample, 6, 12), 22);
    }
}

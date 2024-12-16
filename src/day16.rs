use crate::grid::{Dir, Grid, Point};
use anyhow::{anyhow, Result};
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

pub fn run(input: &str) -> Result<String> {
    let m = parse_map(input)?;
    let s1 = shortest_path_cost(&m).ok_or_else(|| anyhow!("can't find path"))?;
    let s2 = "";
    Ok(format!("{s1} {s2}"))
}

#[derive(Clone)]
struct Map {
    grid: Grid<u8>,
    start: Point,
    end: Point,
}

fn parse_map(input: &str) -> Result<Map> {
    let mut grid = Grid::parse(input)?;
    let start = find_grid_repl(&mut grid, b'S', b'.')?;
    let end = find_grid_repl(&mut grid, b'E', b'.')?;
    Ok(Map { grid, start, end })
}

fn find_grid_repl(grid: &mut Grid<u8>, c: u8, repl: u8) -> Result<Point> {
    let p = grid
        .positions()
        .find(|p| grid.get(*p).copied() == Some(c))
        .ok_or_else(|| anyhow!("error finding {c}"))?;
    *grid.get_mut(p).unwrap() = repl;
    Ok(p)
}

fn shortest_path_cost(m: &Map) -> Option<usize> {
    let mut work = SearchSpace::new(m.grid.dimensions());
    work.push(SearchNode::new(m.start, Dir::East));
    while let Some(n) = work.pop() {
        if n.p == m.end {
            return Some(n.cost);
        }
        let ahead = n.ahead();
        if m.grid.get(ahead.p) == Some(&b'.') {
            work.push(ahead);
        }
        work.push(n.left());
        work.push(n.right());
    }
    None
}

struct SearchSpace {
    heap: BinaryHeap<Reverse<SearchNode>>,
    min_cost: Grid<[usize; 4]>,
}

impl SearchSpace {
    fn new(dimensions: (i32, i32)) -> Self {
        Self {
            heap: BinaryHeap::new(),
            min_cost: Grid::new(dimensions, [usize::MAX; 4]),
        }
    }

    fn pop(&mut self) -> Option<SearchNode> {
        self.heap.pop().map(|r| r.0)
    }

    fn push(&mut self, n: SearchNode) {
        if let Some(x) = self.min_cost.get_mut(n.p) {
            let ic = &mut x[n.dir.index() as usize];
            if n.cost < *ic {
                *ic = n.cost;
                self.heap.push(Reverse(n));
            }
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct SearchNode {
    p: Point,
    dir: Dir,
    cost: usize,
}

impl std::cmp::Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost
            .cmp(&other.cost)
            .then_with(|| self.p.x.cmp(&other.p.x))
            .then_with(|| self.p.y.cmp(&other.p.y))
            .then_with(|| self.dir.cmp(&other.dir))
    }
}

impl std::cmp::PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl SearchNode {
    fn new(p: Point, dir: Dir) -> Self {
        Self { p, dir, cost: 0 }
    }

    fn ahead(&self) -> Self {
        Self {
            p: self.p + self.dir.step(1),
            ..self.with_cost(1)
        }
    }

    fn left(&self) -> Self {
        Self {
            dir: self.dir.left(),
            ..self.with_cost(1000)
        }
    }

    fn right(&self) -> Self {
        Self {
            dir: self.dir.right(),
            ..self.with_cost(1000)
        }
    }

    fn with_cost(&self, c: usize) -> Self {
        Self {
            cost: self.cost + c,
            ..*self
        }
    }
}

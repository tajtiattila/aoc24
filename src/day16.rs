use crate::grid::{Dir, Grid, Point};
use anyhow::{anyhow, Result};
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};

pub fn run(input: &str) -> Result<String> {
    let m = parse_map(input)?;
    let (s1, s2) = stars(&m).ok_or_else(|| anyhow!("can't find path"))?;
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

fn stars(m: &Map) -> Option<(usize, usize)> {
    let mut work = SearchSpace::new(m.grid.dimensions());
    let start = SearchNode::new(m.start, Dir::East);
    work.push(start, start);
    let mut best_cost = None;
    while let Some(n) = work.pop() {
        if let Some(bc) = best_cost {
            if n.cost > bc {
                break;
            }
        }
        if n.p == m.end {
            if best_cost.is_none() {
                best_cost = Some(n.cost);
            }
            work.mark_path_tiles_to(n);
        } else {
            let ahead = n.ahead();
            if m.grid.get(ahead.p) == Some(&b'.') {
                work.push(ahead, n);
            }
            work.push(n.left(), n);
            work.push(n.right(), n);
        }
    }
    best_cost.map(|c| (c, work.count_marked()))
}

struct SearchSpace {
    heap: BinaryHeap<Reverse<SearchNode>>,
    min_cost: Grid<[usize; 4]>,
    backtraq: HashMap<SearchNode, Vec<SearchNode>>,
    mark: Grid<bool>,
}

impl SearchSpace {
    fn new(dimensions: (i32, i32)) -> Self {
        Self {
            heap: BinaryHeap::new(),
            min_cost: Grid::new(dimensions, [usize::MAX; 4]),
            backtraq: HashMap::new(),
            mark: Grid::new(dimensions, false),
        }
    }

    fn pop(&mut self) -> Option<SearchNode> {
        self.heap.pop().map(|r| r.0)
    }

    fn push(&mut self, n: SearchNode, from: SearchNode) {
        if let Some(x) = self.min_cost.get_mut(n.p) {
            let ic = &mut x[n.dir.index() as usize];
            if n.cost <= *ic {
                *ic = n.cost;
                self.heap.push(Reverse(n));
                if from != n {
                    self.backtraq
                        .entry(n)
                        .and_modify(|v| v.push(from))
                        .or_insert(vec![from]);
                }
            }
        }
    }

    fn count_marked(&self) -> usize {
        self.mark.iter().filter(|(_, c)| **c).count()
    }

    fn mark_path_tiles_to(&mut self, n: SearchNode) {
        let (m, b) = (&mut self.mark, &self.backtraq);
        let mut vis = Grid::new(m.dimensions(), 0u8);
        Self::mark_path_tiles_impl(&mut vis, m, b, n);
    }

    fn mark_path_tiles_impl(
        vis: &mut Grid<u8>,
        m: &mut Grid<bool>,
        b: &HashMap<SearchNode, Vec<SearchNode>>,
        p: SearchNode,
    ) {
        if let Some(x) = vis.get_mut(p.p) {
            let mask = 1u8 << p.dir.index();
            if (*x & mask) == 0 {
                *x |= mask;
                *m.get_mut(p.p).unwrap() = true;
                if let Some(v) = b.get(&p) {
                    for q in v {
                        Self::mark_path_tiles_impl(vis, m, b, *q);
                    }
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
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

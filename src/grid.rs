use std::collections::HashSet;

use anyhow::{bail, Result};

// Cell x and y coordinates
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }

    pub const fn xy(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

pub const fn pt(x: i32, y: i32) -> Point {
    Point::new(x, y)
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[allow(unused)]
pub const STEPS: &[Point; 4] = &[pt(0, -1), pt(0, 1), pt(-1, 0), pt(1, 0)];

#[allow(unused)]
pub const DIRS: &[Dir; 4] = &[Dir::North, Dir::South, Dir::West, Dir::East];

// Cardinal directions
#[allow(unused)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Dir {
    North,
    South,
    East,
    West,
}

#[allow(unused)]
impl Dir {
    pub fn from_xy(p: Point) -> Option<Self> {
        let (dx, dy) = p.xy();
        use std::cmp::Ordering::*;
        if dx == 0 {
            match dy.cmp(&0) {
                Less => Some(Self::North),
                Equal => None,
                Greater => Some(Self::South),
            }
        } else if dy == 0 {
            match dx.cmp(&0) {
                Less => Some(Self::West),
                Equal => None,
                Greater => Some(Self::East),
            }
        } else {
            None
        }
    }

    pub fn right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }

    pub fn index(self) -> u8 {
        match self {
            Self::North => 0,
            Self::South => 1,
            Self::East => 2,
            Self::West => 3,
        }
    }

    pub fn step(self, x: i32) -> Point {
        match self {
            Self::North => pt(0, -x),
            Self::South => pt(0, x),
            Self::East => pt(x, 0),
            Self::West => pt(-x, 0),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Grid<T> {
    dx: i32,
    dy: i32,
    m: Vec<T>,
}

#[allow(unused)]
impl<T> Grid<T> {
    pub fn positions(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.dy).flat_map(|y| (0..self.dx).map(move |x| pt(x, y)))
    }

    pub fn iter(&self) -> impl Iterator<Item = (Point, &T)> {
        std::iter::zip(self.positions(), self.m.iter())
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.m.iter()
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        self.m.chunks(self.dx as usize)
    }

    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
        self.m.chunks_mut(self.dx as usize)
    }

    pub fn as_slice(&self) -> &[T] {
        &self.m
    }

    pub fn dimensions(&self) -> (i32, i32) {
        (self.dx, self.dy)
    }

    pub fn to_xy(&self, p: usize) -> Option<Point> {
        (p < self.m.len()).then(|| {
            let i = p as i32;
            pt(i % self.dx, i / self.dx)
        })
    }

    pub fn to_index(&self, p: Point) -> Option<usize> {
        self.is_inside(p).then(|| {
            let (px, py) = p.xy();
            (px + py * self.dx) as usize
        })
    }

    pub fn is_inside(&self, p: Point) -> bool {
        let (px, py) = p.xy();
        px >= 0 && px < self.dx && py >= 0 && py < self.dy
    }

    pub fn get(&self, p: Point) -> Option<&T> {
        self.to_index(p).map(|i| &self.m[i])
    }

    pub fn get_mut(&mut self, p: Point) -> Option<&mut T> {
        self.to_index(p).map(|i| &mut self.m[i])
    }

    pub fn show_by(&self, mut f: impl FnMut(&T) -> char) {
        for row in self.m.chunks(self.dx as usize) {
            let line: String = row.iter().map(&mut f).collect();
            println!("{}", line);
        }
    }
}

#[allow(unused)]
impl<T: Clone> Grid<T> {
    pub fn new((dx, dy): (i32, i32), v: T) -> Self {
        Self {
            dx,
            dy,
            m: vec![v; (dx * dy) as usize],
        }
    }

    pub fn fill_block(&mut self, p0: Point, p1: Point, fillc: T) {
        let x0 = p0.x.min(p1.x).max(0);
        let x1 = p0.x.max(p1.x).min(self.dx);
        let y0 = p0.y.min(p1.y).max(0);
        let y1 = p0.y.max(p1.y).min(self.dy);

        let mut s = (x0 + y0 * self.dx) as usize;
        let w = (x1 - x0) as usize;
        for _ in y0..y1 {
            self.m[s..s + w].fill(fillc.clone());
            s += self.dx as usize;
        }
    }
}

#[allow(unused)]
impl Grid<u8> {
    pub fn parse(input: &str) -> Result<Self> {
        let (dx, m) = input
            .lines()
            .try_fold((0, Vec::new()), |(dx, mut v), line| {
                let bytes = line.as_bytes();
                if !v.is_empty() && dx != bytes.len() {
                    bail!("invalid line");
                }
                v.extend_from_slice(bytes);
                Ok((bytes.len(), v))
            })?;
        let dy = m.len() / dx;
        Ok(Self {
            dx: dx as i32,
            dy: dy as i32,
            m,
        })
    }

    pub fn show(&self) {
        for row in self.m.chunks(self.dx as usize) {
            println!("{}", String::from_utf8_lossy(row));
        }
    }
}

#[allow(unused)]
impl<T: PartialEq> Grid<T> {
    pub fn find(&self, what: &T) -> Option<Point> {
        self.m
            .iter()
            .position(|c| c == what)
            .and_then(|p| self.to_xy(p))
    }
}

#[allow(unused)]
impl<T: PartialEq + Clone> Grid<T> {
    pub fn flood<P>(&mut self, start: Point, value: T, mut pred: P)
    where
        P: FnMut(&T) -> bool,
    {
        if !self.is_inside(start) {
            return;
        }

        let mut stack = vec![start];
        let mut visited = HashSet::new();
        while let Some(p) = stack.pop() {
            *self.get_mut(p).unwrap() = value.clone();
            for &d in STEPS {
                let q = p + d;
                if self.is_inside(q) && !visited.contains(&q) && pred(self.get(q).unwrap()) {
                    visited.insert(q);
                    stack.push(q);
                }
            }
        }
    }
}

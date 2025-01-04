#![allow(dead_code)]

use std::convert::identity;
use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug, Ord, PartialOrd)]
pub struct Loc {
    pub r: i64,
    pub c: i64,
}
impl Loc {
    #[track_caller]
    pub fn new(r: impl TryInto<i64, Error: Debug>, c: impl TryInto<i64, Error: Debug>) -> Self {
        Loc {
            r: r.try_into().unwrap(),
            c: c.try_into().unwrap(),
        }
    }

    pub fn adj(&self) -> [Loc; 4] {
        [
            *self + DeltaLoc::new(1, 0),
            *self + DeltaLoc::new(0, 1),
            *self + DeltaLoc::new(-1, 0),
            *self + DeltaLoc::new(0, -1),
        ]
    }
}

/// A - B points from B to A
impl Sub for Loc {
    type Output = DeltaLoc;
    fn sub(self, rhs: Self) -> Self::Output {
        DeltaLoc {
            dr: self.r - rhs.r,
            dc: self.c - rhs.c,
        }
    }
}
impl Add<DeltaLoc> for Loc {
    type Output = Loc;
    fn add(self, rhs: DeltaLoc) -> Self::Output {
        Loc {
            r: self.r + rhs.dr,
            c: self.c + rhs.dc,
        }
    }
}
impl Sub<DeltaLoc> for Loc {
    type Output = Loc;
    fn sub(self, rhs: DeltaLoc) -> Self::Output {
        Loc {
            r: self.r - rhs.dr,
            c: self.c - rhs.dc,
        }
    }
}
impl AddAssign<DeltaLoc> for Loc {
    fn add_assign(&mut self, rhs: DeltaLoc) {
        *self = *self + rhs
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct DeltaLoc {
    pub dr: i64,
    pub dc: i64,
}
impl DeltaLoc {
    #[track_caller]
    pub fn new(dr: impl TryInto<i64, Error: Debug>, dc: impl TryInto<i64, Error: Debug>) -> Self {
        DeltaLoc {
            dr: dr.try_into().unwrap(),
            dc: dc.try_into().unwrap(),
        }
    }
}
impl Mul<i64> for DeltaLoc {
    type Output = DeltaLoc;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            dr: self.dr * rhs,
            dc: self.dc * rhs,
        }
    }
}

pub fn cardinal_dirs() -> impl Iterator<Item = DeltaLoc> {
    [
        DeltaLoc::new(1, 0),
        DeltaLoc::new(0, 1),
        DeltaLoc::new(-1, 0),
        DeltaLoc::new(0, -1),
    ]
    .into_iter()
}

pub struct Grid<T>(Vec<Vec<T>>);
impl<T> Grid<T> {
    pub fn w(&self) -> usize {
        self.0[0].len()
    }
    pub fn h(&self) -> usize {
        self.0.len()
    }

    pub fn cells(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, cell)| ((r, c), cell)))
    }

    pub fn cells_mut(&mut self) -> impl Iterator<Item = ((usize, usize), &mut T)> {
        self.0.iter_mut().enumerate().flat_map(|(r, row)| {
            row.iter_mut()
                .enumerate()
                .map(move |(c, cell)| ((r, c), cell))
        })
    }

    pub fn try_get_ref(&self, r: impl TryInto<usize>, c: impl TryInto<usize>) -> Option<&T> {
        let ru = r.try_into().ok()?;
        let cu = c.try_into().ok()?;
        self.0.get(ru).and_then(|row| row.get(cu))
    }
    pub fn try_get_mut(
        &mut self,
        r: impl TryInto<usize>,
        c: impl TryInto<usize>,
    ) -> Option<&mut T> {
        let ru = r.try_into().ok()?;
        let cu = c.try_into().ok()?;
        self.0.get_mut(ru).and_then(|row| row.get_mut(cu))
    }
    #[track_caller]
    pub fn get_ref(
        &self,
        r: impl TryInto<usize, Error: Debug>,
        c: impl TryInto<usize, Error: Debug>,
    ) -> &T {
        let ru = r.try_into().unwrap();
        let cu = c.try_into().unwrap();
        self.0.get(ru).unwrap().get(cu).unwrap()
    }
    #[track_caller]
    pub fn get_mut(
        &mut self,
        r: impl TryInto<usize, Error: Debug>,
        c: impl TryInto<usize, Error: Debug>,
    ) -> &mut T {
        let ru = r.try_into().unwrap();
        let cu = c.try_into().unwrap();
        self.0.get_mut(ru).unwrap().get_mut(cu).unwrap()
    }

    pub fn try_set(&mut self, r: impl TryInto<usize>, c: impl TryInto<usize>, t: T) -> Option<()> {
        let ru = r.try_into().ok()?;
        let cu = c.try_into().ok()?;
        self.0
            .get_mut(ru)
            .and_then(|row| row.get_mut(cu))
            .map(|cell| *cell = t);
        Some(())
    }
    #[track_caller]
    pub fn set(
        &mut self,
        r: impl TryInto<usize, Error: Debug>,
        c: impl TryInto<usize, Error: Debug>,
        t: T,
    ) {
        let ru = r.try_into().unwrap();
        let cu = c.try_into().unwrap();
        *self.0.get_mut(ru).unwrap().get_mut(cu).unwrap() = t;
    }
}
impl<T> Grid<T>
where
    T: PartialEq,
{
    pub fn find(&self, t: &T) -> Option<Loc> {
        for ((r, c), cell) in self.cells() {
            if cell == t {
                return Some(Loc::new(r, c));
            }
        }
        None
    }
}
impl<T: Copy> Grid<T> {
    pub fn try_get(&self, r: impl TryInto<usize>, c: impl TryInto<usize>) -> Option<T> {
        let ru = r.try_into().ok()?;
        let cu = c.try_into().ok()?;
        self.0.get(ru).and_then(|row| row.get(cu)).copied()
    }
    #[track_caller]
    pub fn get(
        &self,
        r: impl TryInto<usize, Error: Debug>,
        c: impl TryInto<usize, Error: Debug>,
    ) -> T {
        let ru = r.try_into().unwrap();
        let cu = c.try_into().unwrap();
        *self.0.get(ru).unwrap().get(cu).unwrap()
    }

    pub fn filled_with(t: T, w: usize, h: usize) -> Self {
        Grid(vec![vec![t; w]; h])
    }
}
impl<T: Clone> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Grid(self.0.clone())
    }
}

impl<T: Display> Grid<T> {
    pub fn print(&self) {
        for r in self.0.iter() {
            for c in r {
                print!("{c}");
            }
            println!();
        }
    }
}

pub fn parse_grid(input: impl AsRef<str>) -> Grid<char> {
    parse_grid_with(input, identity)
}

pub fn parse_grid_with<T>(input: impl AsRef<str>, f: impl Copy + Fn(char) -> T) -> Grid<T> {
    Grid(
        input
            .as_ref()
            .lines()
            .map(|row| row.chars().map(f).collect())
            .collect(),
    )
}

#![allow(dead_code)]

use std::convert::identity;
use std::fmt::{Debug, Display};

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
    pub fn get_ref(
        &self,
        r: impl TryInto<usize, Error: Debug>,
        c: impl TryInto<usize, Error: Debug>,
    ) -> &T {
        let ru = r.try_into().unwrap();
        let cu = c.try_into().unwrap();
        self.0.get(ru).unwrap().get(cu).unwrap()
    }
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
impl<T: Copy> Grid<T> {
    pub fn try_get(&self, r: impl TryInto<usize>, c: impl TryInto<usize>) -> Option<T> {
        let ru = r.try_into().ok()?;
        let cu = c.try_into().ok()?;
        self.0.get(ru).and_then(|row| row.get(cu)).copied()
    }
    pub fn get(
        &self,
        r: impl TryInto<usize, Error: Debug>,
        c: impl TryInto<usize, Error: Debug>,
    ) -> T {
        let ru = r.try_into().unwrap();
        let cu = c.try_into().unwrap();
        *self.0.get(ru).unwrap().get(cu).unwrap()
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

pub fn parse_grid(input: String) -> Grid<char> {
    parse_grid_with(input, identity)
}

pub fn parse_grid_with<T>(input: String, f: impl Copy + Fn(char) -> T) -> Grid<T> {
    Grid(
        input
            .lines()
            .map(|row| row.chars().map(f).collect())
            .collect(),
    )
}

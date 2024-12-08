use crate::grid::parse_grid;
use crate::util::{aoc_test, MultiMap};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::ops::{Add, Mul, Sub};

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
struct Loc {
    r: i64,
    c: i64,
}
impl Loc {
    pub fn new(r: impl TryInto<i64, Error: Debug>, c: impl TryInto<i64, Error: Debug>) -> Self {
        Loc {
            r: r.try_into().unwrap(),
            c: c.try_into().unwrap(),
        }
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

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
struct DeltaLoc {
    dr: i64,
    dc: i64,
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

pub fn part1(input: String) -> u64 {
    let grid = parse_grid(input);
    let mut antennae = HashMap::new();

    for ((r, c), cell) in grid.cells() {
        if *cell != '.' {
            antennae.insert_multi(*cell, Loc::new(r, c))
        }
    }

    let mut antinodes = HashSet::new();
    for locations in antennae.values() {
        'pair: for l1 in locations {
            for l2 in locations {
                if l1 == l2 {
                    continue 'pair;
                }
                let delta = *l1 - *l2;
                antinodes.insert(*l1 + delta);
                antinodes.insert(*l2 - delta);
            }
        }
    }

    antinodes
        .into_iter()
        .filter_map(|l| grid.try_get(l.r, l.c))
        .count() as u64
}

pub fn part2(input: String) -> u64 {
    let grid = parse_grid(input);
    let mut antennae = HashMap::new();

    for ((r, c), cell) in grid.cells() {
        if *cell != '.' {
            antennae.insert_multi(*cell, Loc::new(r, c))
        }
    }

    let mut antinodes = HashSet::new();
    for locations in antennae.values() {
        'pair: for l1 in locations {
            for l2 in locations {
                if l1 == l2 {
                    continue 'pair;
                }
                let delta = *l1 - *l2;
                for n in 0.. {
                    let n1 = *l1 + (delta * n);
                    let n2 = *l1 - (delta * n);

                    let mut hits = 0;
                    if grid.try_get(n1.r, n1.c).is_some() {
                        antinodes.insert(n1);
                        hits += 1;
                    }
                    if grid.try_get(n2.r, n2.c).is_some() {
                        antinodes.insert(n2);
                        hits += 1;
                    }

                    if hits == 0 {
                        break;
                    }
                }
            }
        }
    }

    antinodes.len() as u64
}

aoc_test!(
    "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
    14,
    34,
);

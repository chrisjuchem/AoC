use crate::grid::{Grid, Loc};
use crate::util::{aoc_test, SplitInto};
use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Copy, Clone)]
enum Cell {
    Stable,
    Corrupt,
}
impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Cell::Stable => ".",
            Cell::Corrupt => "#",
        })
    }
}

#[cfg(not(test))]
const SIZE: usize = 71;
#[cfg(not(test))]
const NBYTES: usize = 1024;
#[cfg(test)]
const SIZE: usize = 7;
#[cfg(test)]
const NBYTES: usize = 12;

pub fn solve(grid: &Grid<Cell>) -> Option<u64> {
    let mut seen = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((0, Loc::new(0, 0)));
    let target = Loc::new(SIZE - 1, SIZE - 1);

    while let Some((n, loc)) = q.pop_front() {
        if seen.contains(&loc) {
            continue;
        } else {
            seen.insert(loc);
        }
        if target == loc {
            return Some(n);
        }

        if grid.try_get(loc.r, loc.c) != Some(Cell::Stable) {
            continue;
        }

        for nbr in loc.adj() {
            q.push_back((n + 1, nbr));
        }
    }
    None
}

pub fn part1(input: String) -> u64 {
    let mut grid = Grid::filled_with(Cell::Stable, SIZE, SIZE);

    for line in input.lines().take(NBYTES) {
        let (r, c) = line.split_into(",");
        let (r, c) = (r.parse::<u64>().unwrap(), c.parse::<u64>().unwrap());
        *grid.get_mut(r, c) = Cell::Corrupt;
    }

    solve(&grid).unwrap()
}

pub fn part2(input: String) -> u64 {
    let mut grid = Grid::filled_with(Cell::Stable, SIZE, SIZE);

    let mut n = 0;
    for line in input.lines() {
        let (r, c) = line.split_into(",");
        let (r, c) = (r.parse::<u64>().unwrap(), c.parse::<u64>().unwrap());
        *grid.get_mut(r, c) = Cell::Corrupt;
        n += 1;

        if n > NBYTES && solve(&grid).is_none() {
            println!("{line}");
            return r * 1000 + c;
        }
    }
    panic!("not blocked")
}

aoc_test!(
    "5,4
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
2,0",
    22,
    6001,
);

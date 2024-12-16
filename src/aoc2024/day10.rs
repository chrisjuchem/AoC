use crate::grid::{parse_grid_with, Loc};
use crate::util::aoc_test;
use std::collections::{HashSet, VecDeque};

fn trailheads(input: String, unique: bool) -> u64 {
    let grid = parse_grid_with(input, |n| n.to_digit(10).unwrap());

    let mut score = 0;
    for ((r, c), cell) in grid.cells() {
        if *cell != 0 {
            continue;
        }

        let mut q = VecDeque::new();
        let mut seen = HashSet::new();
        let loc = Loc::new(r, c);
        q.push_back(loc);
        while let Some(loc) = q.pop_front() {
            if !unique {
                if seen.contains(&loc) {
                    continue;
                } else {
                    seen.insert(loc);
                }
            }

            let h = grid.get(loc.r, loc.c);
            if h == 9 {
                score += 1;
                continue;
            }
            for l2 in loc.adj() {
                if grid.try_get(l2.r, l2.c) == Some(h + 1) {
                    q.push_back(l2)
                }
            }
        }
    }

    score
}

pub fn part1(input: String) -> u64 {
    trailheads(input, false)
}
pub fn part2(input: String) -> u64 {
    trailheads(input, true)
}

aoc_test!(
    "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
",
    36,
    81,
);

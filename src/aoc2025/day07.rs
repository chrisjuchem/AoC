use std::collections::{HashMap, HashSet};

use crate::grid::{Loc, parse_grid};
use crate::util::{CountMap, aoc_test};

pub fn part1(input: String) -> u64 {
    let grid = parse_grid(input);

    let Some(Loc { r: 0, c: start }) = grid.find(&'S') else {
        panic!("bad start")
    };

    let mut splits = 0;
    let mut input = HashSet::from([start]);
    for r in 1..grid.h() {
        let mut output = HashSet::new();
        for beam in input {
            match grid.get(r, beam) {
                '.' => {
                    output.insert(beam);
                }
                '^' => {
                    output.insert(beam + 1);
                    output.insert(beam - 1);
                    splits += 1;
                }
                _ => panic!("unknown char"),
            }
        }
        input = output;
    }

    splits
}

pub fn part2(input: String) -> u64 {
    let grid = parse_grid(input);

    let Some(Loc { r: 0, c: start }) = grid.find(&'S') else {
        panic!("bad start")
    };

    let mut input = HashMap::from([(start, 1)]);
    for r in 1..grid.h() {
        let mut output = HashMap::new();
        for (beam, count) in input {
            match grid.get(r, beam) {
                '.' => {
                    output.insert_n(beam, count);
                }
                '^' => {
                    output.insert_n(beam + 1, count);
                    output.insert_n(beam - 1, count);
                }
                _ => panic!("unknown char"),
            }
        }
        input = output;
    }

    input.total_count()
}

aoc_test!(
    ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
    21,
    40,
);

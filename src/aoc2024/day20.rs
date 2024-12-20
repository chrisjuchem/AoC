use crate::grid::{parse_grid, DeltaLoc, Loc};
use crate::util::aoc_test;
use std::collections::HashMap;

pub fn part1(input: String) -> u64 {
    let mut grid = parse_grid(input);

    let mut start = Loc::new(0, 0);
    for ((r, c), cell) in grid.cells_mut() {
        if *cell == 'S' {
            start = Loc::new(r, c);
            *cell = '.';
        }
        if *cell == 'E' {
            *cell = '.';
        }
    }

    let mut map = HashMap::new();
    map.insert(start, 0);

    let mut current = start;
    'outer: loop {
        for n in current.adj() {
            if grid.try_get(n.r, n.c) == Some('.') && !map.contains_key(&n) {
                let dist = map[&current];
                map.insert(n, dist + 1);
                current = n;
                continue 'outer;
            }
        }
        break;
    }

    #[cfg(test)]
    let threshold = 20;
    #[cfg(not(test))]
    let threshold = 100;

    let dirs = [
        DeltaLoc::new(2, 0),
        DeltaLoc::new(-2, 0),
        DeltaLoc::new(0, 2),
        DeltaLoc::new(0, -2),
    ];

    let mut n_big_shortcuts = 0;
    for (start, start_dist) in &map {
        for dir in dirs {
            let end = *start + dir;
            if let Some(end_dist) = map.get(&end) {
                if end_dist - start_dist >= threshold + 2 {
                    n_big_shortcuts += 1;
                }
            }
        }
    }

    n_big_shortcuts
}

pub fn part2(_input: String) -> u64 {
    0
}

aoc_test!(
    "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############",
    5,
    0,
);

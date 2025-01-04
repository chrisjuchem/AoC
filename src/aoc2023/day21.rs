use crate::grid::{Loc, parse_grid};
use crate::util::aoc_test;
use std::collections::{HashMap, HashSet};

pub fn part1(input: String) -> u64 {
    let mut grid = parse_grid(input);
    let mut active = HashMap::new();
    let mut candidates = HashMap::new();
    let mut prev_active = HashMap::new();
    for r in 0..grid.h() {
        for c in 0..grid.w() {
            if grid.try_get_ref(r, c).unwrap() == &'S' {
                grid.try_set(r, c, '.');
                active.insert(Loc::new(r, c), 1);
            }
        }
    }

    #[cfg(not(test))]
    let steps = 64;
    #[cfg(test)]
    let steps = 6;

    assert_eq!(grid.h(), grid.w());
    let size = grid.h() as i64;

    for _ in 0..steps {
        // for r in 0..grid.len() {
        //     for c in 0..grid[0].len() {
        //         print!(
        //             "{}",
        //             active
        //                 .get(&Loc {
        //                     r: r as i32,
        //                     c: c as i32
        //                 })
        //                 .map(|n| n.to_string().chars().last().unwrap())
        //                 .unwrap_or_else(|| grid[r][c])
        //         )
        //     }
        //     println!()
        // }
        // println!();

        std::mem::swap(&mut active, &mut prev_active);
        candidates.clear();
        active.clear();

        for (loc, n) in prev_active.iter() {
            for candidate in loc.adj() {
                let reached: &mut u64 = candidates.entry(candidate).or_default();
                *reached = (*reached).max(*n)
            }
        }

        for (loc, n) in candidates.iter() {
            if loc.r >= size || loc.r < 0 || loc.c < 0 || loc.c >= size {
                // invalid
            } else {
                if grid.try_get(loc.r, loc.c) == Some('.') {
                    *active.entry(*loc).or_default() += n; //
                }
            }
        }
    }

    active.values().sum()
}

pub fn part2(input: String) -> usize {
    // The grid has unobstructed channels vertically and horizontally from the starting location.
    //
    // Should be able to calculate the final pattern for a central copy of the grid and for each
    // possible edge copy and then do some math about how many of each there are.
    //
    // This assumes that spillover is only relevant directly from the channels and not elsewhere,
    // and that the grids 1 away from the final edge have time to fill completely. Not sure if those
    // are actually true, but it seems possible.

    let mut grid = parse_grid(input);

    let mut frontier = HashSet::new();
    let mut next_frontier = HashSet::new();
    let mut step_times = HashMap::new();

    assert_eq!(grid.h(), grid.w());
    let size = grid.h() as i64;

    for ((r, c), cell) in grid.cells_mut() {
        if *cell == 'S' {
            *cell = '.';
            frontier.insert(Loc::new(r, c));
        }
    }

    #[cfg(not(test))]
    let steps = 26501365;
    #[cfg(test)]
    let steps = 100;

    for i in 0..=steps {
        if i % 1000 == 0 {
            println!("{i}, {}", frontier.len());
        }

        for l in frontier.drain() {
            step_times.insert(l, i);

            for n in l.adj() {
                let r = n.r.rem_euclid(size);
                let c = n.c.rem_euclid(size);
                if grid.try_get(r, c) == Some('.') && !step_times.contains_key(&n) {
                    next_frontier.insert(n);
                }
            }
        }

        std::mem::swap(&mut frontier, &mut next_frontier);
    }

    // for r in -4 * size..5 * size {
    //     for c in -4 * size..5 * size {
    //         let loc = Loc::new(r, c);
    //         print!(
    //             "{}",
    //             step_times
    //                 .get(&loc)
    //                 .map(|n| n.to_string().chars().last().unwrap())
    //                 .unwrap_or_else(|| grid.get(r.rem_euclid(size), c.rem_euclid(size)))
    //         )
    //     }
    //     println!()
    // }
    // println!();

    step_times.values().filter(|t| *t % 2 == steps % 2).count()
}

aoc_test!(
    "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
    //     "...........
    // .###....#..
    // ..#...#..#.
    // ....#..#...
    // ..#.#.#.#..
    // .....S.....
    // ...##.###..
    // .#.....#...
    // ...........
    // .##....###.
    // ...........",
    16,
    //     ".....
    // .....
    // ..S..
    // .....
    // #....",
    6536,
);

/* 22
#...##...##...##...##...##...##...##...##...#
......................1......................
.....................1.1.....................
....................1.1.1....................
#...##...##...##...##1.1##...##...##...##...#
#...##...##...##..1##.1.##1..##...##...##...#
.................1.1.1.1.1.1.................
................1.1.1.1.1.1.1................
...............1.1.1.1.1.1.1.1...............
#...##...##...##1.1##.1.##1.1##...##...##...#
#...##...##..1##.1.##1.1##.1.##1..##...##...#
............1.1.1.1.1.1.1.1.1.1.1............
...........1.1.1.1.1.1.1.1.1.1.1.1...........
..........1.1.1.1.1.1.1.1.1.1.1.1.1..........
#...##...##1.1##.1.##1.1##.1.##1.1##...##...#
#...##..1##.1.##1.1##.1.##1.1##.1.##1..##...#
.......1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.......
......1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1......
.....1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.....
#...##1.1##.1.##1.1##.1.##1.1##.1.##1.1##...#
#..1##.1.##1.1##.1.##1.1##.1.##1.1##.1.##1..#
..1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1..
.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.
..1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1..
#..1##.1.##1.1##.1.##1.1##.1.##1.1##.1.##1..#
#...##1.1##.1.##1.1##.1.##1.1##.1.##1.1##...#
.....1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.....
......1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1......
.......1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.......
#...##..1##.1.##1.1##.1.##1.1##.1.##1..##...#
#...##...##1.1##.1.##1.1##.1.##1.1##...##...#
..........1.1.1.1.1.1.1.1.1.1.1.1.1..........
...........1.1.1.1.1.1.1.1.1.1.1.1...........
............1.1.1.1.1.1.1.1.1.1.1............
#...##...##..1##.1.##1.1##.1.##1..##...##...#
#...##...##...##1.1##.1.##1.1##...##...##...#
...............1.1.1.1.1.1.1.1...............
................1.1.1.1.1.1.1................
.................1.1.1.1.1.1.................
#...##...##...##..1##.1.##1..##...##...##...#
#...##...##...##...##1.1##...##...##...##...#
....................1.1.1....................
.....................1.1.....................
......................1......................
#...##...##...##...##...##...##...##...##...#

A       B
#.1.#   #1.1#   #1.1#   #1.1#
.1.1.   1.1.1   1.1.1   1.1.1
1.1.1   .1.1.   .1.1.   .1.1.
.1.1.   1.1.1   ..1.1   1.1..
#.1.#   #1.1#   #..1#   #1..#

 CD    4
 AED   3
 BAED  2
 ABAED 1
 BABAC 0
 |-L--|
  202300.5
  4.5

  plus central B

            counting center row, not center col
           A       B
        0  101150  101149    2 1
        1  101149  101149    1 1
        2  101149  101148
        3  101148  101148
        4  101148  101147
           ...
      L-2  1       0          1 0
      L-1  0       0          0 0
 */

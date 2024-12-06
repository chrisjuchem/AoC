use crate::util::{aoc_test, parse_grid};
use std::collections::HashMap;
use std::convert::identity;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Loc {
    r: i32,
    c: i32,
}
impl Loc {
    fn adjacent(self) -> [Loc; 4] {
        [
            Loc {
                r: self.r - 1,
                c: self.c,
            },
            Loc {
                r: self.r + 1,
                c: self.c,
            },
            Loc {
                r: self.r,
                c: self.c - 1,
            },
            Loc {
                r: self.r,
                c: self.c + 1,
            },
        ]
    }
}

struct Grid(Vec<Vec<char>>);
impl Grid {
    fn get(&self, loc: Loc) -> char {
        let r = loc.r.rem_euclid(self.0.len() as i32);
        let c = loc.c.rem_euclid(self.0[0].len() as i32);
        self.0[r as usize][c as usize]
    }
}

pub fn part1(input: String) -> u64 {
    let mut grid = parse_grid(input, identity);
    let mut active = HashMap::new();
    let mut candidates = HashMap::new();
    let mut prev_active = HashMap::new();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 'S' {
                grid[r][c] = '.';
                active.insert(
                    Loc {
                        r: r as i32,
                        c: c as i32,
                    },
                    1,
                );
            }
        }
    }

    #[cfg(not(test))]
    let steps = 64;
    #[cfg(test)]
    let steps = 6;

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
            for candidate in loc.adjacent() {
                let reached: &mut u64 = candidates.entry(candidate).or_default();
                *reached = (*reached).max(*n)
            }
        }

        for (loc, n) in candidates.iter() {
            if loc.r >= grid.len() as i32 || loc.r < 0 || loc.c < 0 || loc.c >= grid[0].len() as i32
            {
                // invalid
            } else {
                if grid[loc.r as usize][loc.c as usize] == '.' {
                    *active.entry(*loc).or_default() += n; //
                }
            }
        }
    }

    active.values().sum()
}

pub fn part2(input: String) -> u64 {
    // The grid has unobstructed channels vertically and horizontally from the starting location.
    //
    // Should be able to calculate the final pattern for a central copy of the grid and for each
    // possible edge copy and then do some math about how many of each there are.
    //
    // This assumes that spillover is only relevant directly from the channels and not elsewhere,
    // and that the grids 1 away from the final edge have time to fill completely. Not sure if those
    // are actually true, but it seems possible.

    let mut grid = Grid(parse_grid(input, identity));
    let mut active = HashMap::new();
    let mut candidates = HashMap::new();
    let mut prev_active = HashMap::new();

    let (h, w) = (grid.0.len() as i32, grid.0[0].len() as i32);
    assert_eq!(h, w);
    let size = h;

    for r in 0..size {
        for c in 0..size {
            if grid.get(Loc { r, c }) == 'S' {
                grid.0[r as usize][c as usize] = '.';
                active.insert(Loc { r, c }, 1);
            }
        }
    }

    #[cfg(not(test))]
    let steps = 64;
    #[cfg(test)]
    let steps = 49i32;

    // let steps = steps.rem_euclid(&size) + size *

    for _ in 0..steps {
        // for r in -4 * size..5 * size {
        //     for c in -4 * size..5 * size {
        //         let loc = Loc { r, c };
        //         print!(
        //             "{}",
        //             active
        //                 .get(&loc)
        //                 .map(|n| n.to_string().chars().last().unwrap())
        //                 .unwrap_or_else(|| grid.get(loc))
        //         )
        //     }
        //     println!()
        // }
        // println!();

        std::mem::swap(&mut active, &mut prev_active);
        candidates.clear();
        active.clear();

        for (loc, n) in prev_active.iter() {
            for candidate in loc.adjacent() {
                let reached: &mut u64 = candidates.entry(candidate).or_default();
                *reached = (*reached).max(*n)
            }
        }

        for (loc, n) in candidates.iter() {
            if grid.get(*loc) == '.' {
                *active.entry(*loc).or_default() += n;
            }
        }
    }

    active.values().sum()
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
    1528, //fixme
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

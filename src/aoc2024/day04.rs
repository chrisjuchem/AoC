use crate::util::{aoc_test, parse_grid};
use std::convert::identity;

struct Grid<T>(Vec<Vec<T>>);
impl<T: Copy> Grid<T> {
    fn get(&self, r: impl TryInto<usize>, c: impl TryInto<usize>) -> Option<T> {
        let ru = r.try_into().ok()?;
        let cu = c.try_into().ok()?;
        self.0.get(ru).and_then(|row| row.get(cu)).copied()
    }

    fn w(&self) -> usize {
        self.0[0].len()
    }
    fn h(&self) -> usize {
        self.0.len()
    }
}

pub fn part1(_input: String) -> u64 {
    let grid = Grid(parse_grid(_input, identity));

    let dirs = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    let mut n = 0;
    for row in 0..grid.h() as i32 {
        for col in 0..grid.w() as i32 {
            if grid.get(row, col) != Some('X') {
                continue;
            };

            for (dr, dc) in dirs {
                let mut r = row;
                let mut c = col;

                r += dr;
                c += dc;
                if grid.get(r, c) != Some('M') {
                    continue;
                };
                r += dr;
                c += dc;
                if grid.get(r, c) != Some('A') {
                    continue;
                };
                r += dr;
                c += dc;
                if grid.get(r, c) != Some('S') {
                    continue;
                };
                n += 1;
            }
        }
    }

    n
}

pub fn part2(_input: String) -> u64 {
    let grid = Grid(parse_grid(_input, identity));

    let mut n = 0;
    for row in 0..grid.h() as i32 {
        for col in 0..grid.w() as i32 {
            if grid.get(row, col) != Some('A') {
                continue;
            };

            match (grid.get(row - 1, col - 1), grid.get(row + 1, col + 1)) {
                (Some('M'), Some('S')) => {}
                (Some('S'), Some('M')) => {}
                _ => continue,
            }
            match (grid.get(row - 1, col + 1), grid.get(row + 1, col - 1)) {
                (Some('M'), Some('S')) => {}
                (Some('S'), Some('M')) => {}
                _ => continue,
            }

            n += 1
        }
    }
    n
}

aoc_test!(
    "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
    18,
    9,
);

use crate::grid::parse_grid;
use crate::util::aoc_test;

pub fn part1(input: String) -> u64 {
    let grid = parse_grid(input);

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
    for ((row, col), cell) in grid.cells() {
        if *cell != 'X' {
            continue;
        };

        for (dr, dc) in dirs {
            let mut r = row as i32;
            let mut c = col as i32;

            r += dr;
            c += dc;
            if grid.try_get(r, c) != Some('M') {
                continue;
            };
            r += dr;
            c += dc;
            if grid.try_get(r, c) != Some('A') {
                continue;
            };
            r += dr;
            c += dc;
            if grid.try_get(r, c) != Some('S') {
                continue;
            };
            n += 1;
        }
    }

    n
}

pub fn part2(_input: String) -> u64 {
    let grid = parse_grid(_input);

    let mut n = 0;
    for ((r, c), cell) in grid.cells() {
        if *cell != 'A' {
            continue;
        };

        let row = r as i32;
        let col = c as i32;
        match (
            grid.try_get(row - 1, col - 1),
            grid.try_get(row + 1, col + 1),
        ) {
            (Some('M'), Some('S')) => {}
            (Some('S'), Some('M')) => {}
            _ => continue,
        }
        match (
            grid.try_get(row - 1, col + 1),
            grid.try_get(row + 1, col - 1),
        ) {
            (Some('M'), Some('S')) => {}
            (Some('S'), Some('M')) => {}
            _ => continue,
        }

        n += 1
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

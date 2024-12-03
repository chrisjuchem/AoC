use crate::util::aoc_test;
use std::cmp::min;

fn n_bits(mut n: u64) -> u64 {
    let mut count = 0;
    while n > 0 {
        if n & 1 > 0 {
            count += 1;
        }
        n = n >> 1;
    }
    count
}

pub fn reflection_idx(list: &Vec<u64>, wanted_diffs: u64) -> Option<u64> {
    //  ..
    //  .#
    //  ##
    //  ##
    //  .#

    // candidate = 3
    // refl_dist = min(3, 5-3) = 2
    // test (2,3) (1,4)
    for candidate in 1..list.len() {
        let refl_dist = min(candidate, list.len() - candidate);
        let mut diffs = 0;
        for i in 0..refl_dist {
            diffs += n_bits(list[candidate + i] ^ list[candidate - i - 1]);
        }
        if diffs == wanted_diffs {
            return Some(candidate as u64);
        }
    }
    None
}

pub fn get_reflection_codes(_input: String, differences: u64) -> u64 {
    _input
        .split("\n\n")
        .map(|grid| {
            let mut row_codes = vec![];
            let mut col_codes = vec![];

            for (r, line) in grid.lines().enumerate() {
                row_codes.push(0);
                for (c, tile) in line.chars().enumerate() {
                    if r == 0 {
                        col_codes.push(0);
                    }

                    if tile == '#' {
                        row_codes[r] += 1 << c;
                        col_codes[c] += 1 << r;
                    }
                }
            }

            match reflection_idx(&row_codes, differences) {
                None => reflection_idx(&col_codes, differences).unwrap(),
                Some(n) => n * 100,
            }
        })
        .sum()
}

pub fn part1(_input: String) -> u64 {
    get_reflection_codes(_input, 0)
}
pub fn part2(_input: String) -> u64 {
    get_reflection_codes(_input, 1)
}

aoc_test!(
    "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
    405,
    400,
);

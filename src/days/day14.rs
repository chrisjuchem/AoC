use crate::util::aoc_test;
use std::hash::{DefaultHasher, Hash, Hasher};

fn shift(grid: &mut Vec<Vec<char>>, dr: isize, dc: isize) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let r = if dr > 0 { grid.len() - i - 1 } else { i };
            let c = if dc > 0 { grid[0].len() - j - 1 } else { j };
            if grid[r][c] != 'O' {
                continue;
            }
            let (mut new_r, mut new_c) = (r, c);
            while (dr >= 0 || new_r > 0)
                && (dc >= 0 || new_c > 0)
                && (dr <= 0 || new_r < grid.len() - 1)
                && (dc <= 0 || new_c < grid[0].len() - 1)
                && grid[new_r.wrapping_add_signed(dr)][new_c.wrapping_add_signed(dc)] == '.'
            {
                new_r = new_r.wrapping_add_signed(dr);
                new_c = new_c.wrapping_add_signed(dc);
            }
            if new_r != r || new_c != c {
                grid[new_r][new_c] = 'O';
                grid[r][c] = '.';
            }
        }
    }
}
fn score(grid: &Vec<Vec<char>>) -> u64 {
    grid.iter()
        .enumerate()
        .map(|(r, row)| (grid.len() - r) * row.iter().filter(|chr| **chr == 'O').count())
        .sum::<usize>() as u64
}

pub fn part1(_input: String) -> u64 {
    let mut grid: Vec<Vec<_>> = _input.lines().map(|line| line.chars().collect()).collect();
    shift(&mut grid, -1, 0);
    score(&grid)
}

pub fn cycle(grid: &mut Vec<Vec<char>>) {
    shift(grid, -1, 0);
    shift(grid, 0, -1);
    shift(grid, 1, 0);
    shift(grid, 0, 1);
}

pub fn part2(_input: String) -> u64 {
    let mut grid: Vec<Vec<_>> = _input.lines().map(|line| line.chars().collect()).collect();

    let mut hashes = vec![];
    let mut remaining_cycles = 1_000_000_000;
    while remaining_cycles > 0 {
        cycle(&mut grid);
        remaining_cycles -= 1;

        let mut hasher = DefaultHasher::new();
        grid.hash(&mut hasher);
        let hash = hasher.finish();
        if let Some(idx) = hashes.iter().position(|h| hash.eq(h)) {
            // 0, 1, 102, 103, 104
            //102, idx = 2, len = 5
            let cycle_len = hashes.len() - idx;
            println!("found cycle, length = {}", cycle_len);
            remaining_cycles %= cycle_len;
            hashes.clear();
        }
        hashes.push(hash);
    }
    score(&grid)
}

aoc_test!(
    "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
",
    136,
    64,
);

use crate::util::{aoc_test, SplitInto};
use std::iter::repeat;

use crate::day10::{floodfill, Dir, State};

impl From<&str> for Dir {
    fn from(value: &str) -> Self {
        match value {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => panic!("bad dir"),
        }
    }
}

pub fn part1(_input: String) -> u64 {
    let dirs = _input.lines().flat_map(|row| {
        let (d, n, _) = row.split_into(" ");
        repeat(Dir::from(d)).take(n.parse().unwrap())
    });

    let digs = dirs.clone().count();
    let mut grid = vec![vec![State::In; digs / 2]; digs / 2];
    let (mut r, mut c) = (digs / 4, digs / 4);
    for d in dirs {
        let (dr, dc) = d.drdc();
        r = r.wrapping_add_signed(dr);
        c = c.wrapping_add_signed(dc);
        grid[r][c] = State::Pipe;
    }
    floodfill(&mut grid);

    grid.iter()
        .flat_map(|row| row.iter().filter(|state| **state != State::Out))
        .count() as u64
}

pub fn part2(_input: String) -> u64 {
    0
}

aoc_test!(
    "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
",
    62,
    952408144115,
);

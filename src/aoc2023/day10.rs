use crate::util::aoc_test;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Dir {
    fn drdc(&self) -> (isize, isize) {
        match self {
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
        }
    }
    fn all() -> impl Iterator<Item = Dir> {
        [Dir::Up, Dir::Left, Dir::Down, Dir::Right].into_iter()
    }
}

trait HasDirs {
    fn out_dir(&self, d: Dir) -> Option<Dir>;
}
impl HasDirs for char {
    fn out_dir(&self, in_d: Dir) -> Option<Dir> {
        match self {
            'F' => match in_d {
                Dir::Left => Some(Dir::Down),
                Dir::Up => Some(Dir::Right),
                _ => None,
            },
            '7' => match in_d {
                Dir::Right => Some(Dir::Down),
                Dir::Up => Some(Dir::Left),
                _ => None,
            },
            'J' => match in_d {
                Dir::Right => Some(Dir::Up),
                Dir::Down => Some(Dir::Left),
                _ => None,
            },
            'L' => match in_d {
                Dir::Left => Some(Dir::Up),
                Dir::Down => Some(Dir::Right),
                _ => None,
            },
            '|' => match in_d {
                Dir::Down => Some(Dir::Down),
                Dir::Up => Some(Dir::Up),
                _ => None,
            },
            '-' => match in_d {
                Dir::Left => Some(Dir::Left),
                Dir::Right => Some(Dir::Right),
                _ => None,
            },
            _ => None,
        }
    }
}

pub fn path_tiles(_input: String) -> Vec<(usize, usize)> {
    let pipes: Vec<Vec<_>> = _input.lines().map(|line| line.chars().collect()).collect();
    let (mut nextr, mut nextc) = (0, 0);
    for (r, row) in pipes.iter().enumerate() {
        for (c, pipe) in row.iter().enumerate() {
            if *pipe == 'S' {
                nextr = r;
                nextc = c;
            }
        }
    }

    let mut next_dir = Dir::Up;
    for d in Dir::all() {
        let (dr, dc) = d.drdc();
        let r = (nextr as isize + dr) as usize;
        let c = (nextc as isize + dc) as usize;
        if r > pipes.len() || c > pipes[r].len() {
            continue;
        }

        if let Some(_) = pipes[r][c].out_dir(d) {
            next_dir = d;
            nextr = r;
            nextc = c;
            break;
        }
    }

    // r,c = 0,0
    //
    // S
    // L7
    //
    // next_dir = down
    // r,c = 1,0 (next)

    let mut coords = vec![(nextr, nextc)];
    while pipes[nextr][nextc] != 'S' {
        next_dir = pipes[nextr][nextc].out_dir(next_dir).unwrap();
        let (dr, dc) = next_dir.drdc();
        nextr = (nextr as isize + dr) as usize;
        nextc = (nextc as isize + dc) as usize;
        coords.push((nextr, nextc))
    }
    coords
}

pub fn part1(input: String) -> u64 {
    path_tiles(input).len() as u64 / 2
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum State {
    Pipe,
    In,
    Out,
}
impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            State::Pipe => "+",
            State::In => "!",
            State::Out => ".",
        })
    }
}

fn floodfill(grid: &mut Vec<Vec<State>>) {
    let mut to_process = VecDeque::from([(0, 0)]);

    while !to_process.is_empty() {
        let (r, c) = to_process.pop_front().unwrap();
        if grid[r][c] != State::In {
            continue;
        }
        grid[r][c] = State::Out;
        if c + 1 < grid[r].len() {
            to_process.push_back((r, c + 1));
        }
        if r + 1 < grid.len() {
            to_process.push_back((r + 1, c));
        }
        if c > 0 {
            to_process.push_back((r, c - 1));
        }
        if r > 0 {
            to_process.push_back((r - 1, c));
        }
    }
}

pub fn part2(input: String) -> u64 {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();

    let mut double_grid = vec![vec![State::In; cols * 2 + 1]; rows * 2 + 1];

    let path = path_tiles(input);
    let (mut prevr, mut prevc) = path.last().unwrap();
    for (r, c) in path {
        double_grid[prevr + r + 1][prevc + c + 1] = State::Pipe;
        double_grid[r * 2 + 1][c * 2 + 1] = State::Pipe;
        prevr = r;
        prevc = c;
    }

    // for row in &double_grid {
    //     for s in row {
    //         print!("{s}");
    //     }
    //     println!("");
    // }

    floodfill(&mut double_grid);

    let mut inside_count = 0;
    for r in 0..rows {
        for c in 0..cols {
            if double_grid[r * 2 + 1][c * 2 + 1] == State::In {
                inside_count += 1;
            }
        }
    }
    inside_count
}

aoc_test!(
    "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
    8,
    "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
    10,
);

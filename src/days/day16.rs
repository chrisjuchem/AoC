use crate::util::aoc_test;
use std::cmp::max;
use std::collections::HashSet;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}
impl Dir {
    fn hit_char(&self, c: char) -> Vec<Dir> {
        match c {
            '-' => match self {
                Dir::Up | Dir::Down => vec![Dir::Right, Dir::Left],
                parallel => vec![*parallel],
            },
            '|' => match self {
                Dir::Right | Dir::Left => vec![Dir::Up, Dir::Down],
                parallel => vec![*parallel],
            },
            '/' => vec![match self {
                Dir::Left => Dir::Down,
                Dir::Right => Dir::Up,
                Dir::Up => Dir::Right,
                Dir::Down => Dir::Left,
            }],
            '\\' => vec![match self {
                Dir::Left => Dir::Up,
                Dir::Right => Dir::Down,
                Dir::Up => Dir::Left,
                Dir::Down => Dir::Right,
            }],
            _ => vec![*self],
        }
    }

    fn drdc(&self) -> (isize, isize) {
        match self {
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
        }
    }

    fn all() -> impl Iterator<Item = Dir> {
        [Dir::Up, Dir::Down, Dir::Right, Dir::Left].into_iter()
    }
}

#[derive(Clone)]
struct Tile {
    c: char,
    incoming_beams: HashSet<Dir>,
}

impl Tile {
    fn new(c: char) -> Self {
        Tile {
            c,
            incoming_beams: HashSet::new(),
        }
    }

    fn hit(&mut self, from_dir: Dir) -> Vec<Dir> {
        if self.incoming_beams.insert(from_dir) {
            from_dir.hit_char(self.c)
        } else {
            vec![]
        }
    }
}

fn grid_energy(mut grid: Vec<Vec<Tile>>, start_dir: Dir, i: usize) -> u64 {
    let (startr, startc) = match start_dir {
        Dir::Left => (i, grid[0].len() - 1),
        Dir::Right => (i, 0),
        Dir::Up => (grid.len() - 1, i),
        Dir::Down => (0, i),
    };

    let mut paths_to_process = vec![(start_dir, startr, startc)];
    while !paths_to_process.is_empty() {
        let (incoming_dir, r, c) = paths_to_process.pop().unwrap();
        if r >= grid.len() || c >= grid[0].len() {
            continue;
        }

        for outgoing_dir in grid[r][c].hit(incoming_dir) {
            let (dr, dc) = outgoing_dir.drdc();
            paths_to_process.push((
                outgoing_dir,
                r.wrapping_add_signed(dr),
                c.wrapping_add_signed(dc),
            ))
        }
    }

    grid.iter()
        .flat_map(|row| row.iter())
        .filter(|tile| !tile.incoming_beams.is_empty())
        .count() as u64
}

pub fn part1(_input: String) -> u64 {
    let grid: Vec<Vec<_>> = _input
        .lines()
        .map(|line| line.chars().map(Tile::new).collect())
        .collect();
    grid_energy(grid, Dir::Right, 0)
}

pub fn part2(_input: String) -> u64 {
    let grid: Vec<Vec<_>> = _input
        .lines()
        .map(|line| line.chars().map(Tile::new).collect())
        .collect();
    let grid = &grid;

    let n = max(grid.len(), grid[0].len());
    (0..n)
        .flat_map(|n| Dir::all().map(move |dir| grid_energy(grid.clone(), dir, n)))
        .max()
        .unwrap()
}

aoc_test!(
    r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
",
    46,
    51,
);

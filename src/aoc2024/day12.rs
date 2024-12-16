use crate::grid::{parse_grid_with, Grid, Loc};
use crate::util::aoc_test;
use std::collections::{HashSet, VecDeque};

#[derive(Copy, Clone)]
struct Cell {
    kind: char,
    seen: bool,
}
impl Cell {
    fn new(kind: char) -> Self {
        Self { kind, seen: false }
    }
}

fn divide(input: String) -> Vec<(HashSet<Loc>, Vec<Loc>)> {
    let mut regions = vec![];

    let mut grid: Grid<Cell> = parse_grid_with(input, Cell::new);

    for r in 0..grid.h() {
        for c in 0..grid.w() {
            let cell = grid.get(r, c);
            if cell.seen {
                continue;
            }

            let mut q = VecDeque::new();
            q.push_back(Loc::new(r, c));

            let mut area = HashSet::new();
            let mut perimeter = vec![];

            while let Some(loc) = q.pop_front() {
                let Loc { r, c } = loc;
                match grid.try_get_mut(r, c) {
                    Some(crop) if crop.kind == cell.kind => {
                        if !crop.seen {
                            crop.seen = true;
                            area.insert(loc);
                            for next in loc.adj() {
                                q.push_back(next)
                            }
                        }
                    }
                    _ => {
                        perimeter.push(loc);
                    }
                }
            }

            regions.push((area, perimeter))
        }
    }

    regions
}

pub fn part1(input: String) -> u64 {
    // println!("{:?}", divide(input));

    divide(input)
        .into_iter()
        .map(|(a, p)| a.len() * p.len())
        .sum::<usize>() as u64
}

pub fn part2(_input: String) -> u64 {
    0
}

aoc_test!(
    "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
",
    1930,
    0,
);

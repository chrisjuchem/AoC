use crate::{
    grid::{Grid, Loc, parse_grid},
    util::aoc_test,
};

pub fn part1(input: String) -> u64 {
    let grid = parse_grid(input);

    accessible(&grid).len() as u64
}

fn accessible(grid: &Grid<char>) -> Vec<Loc> {
    grid.cells()
        .filter_map(|((r, c), char)| {
            if *char != '@' {
                return None;
            }

            let loc = Loc::new(r, c);
            let adj: u8 = loc
                .adj8()
                .iter()
                .map(|l| {
                    if grid.try_get(l.r, l.c) == Some('@') {
                        1u8
                    } else {
                        0
                    }
                })
                .sum();

            (adj < 4).then_some(loc)
        })
        .collect()
}

pub fn part2(input: String) -> u64 {
    let mut grid = parse_grid(input);

    let mut count = 0;
    loop {
        let acc = accessible(&grid);
        if acc.is_empty() {
            break;
        }

        for a in acc.iter().copied() {
            grid.set(a.r, a.c, 'x');
        }

        count += acc.len() as u64;
    }

    count
}

aoc_test!(
    "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
    13,
    43,
);

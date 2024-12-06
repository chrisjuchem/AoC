use crate::grid::parse_grid;
use crate::util::aoc_test;
use std::collections::HashSet;

pub fn part1(input: String) -> u64 {
    let mut grid = parse_grid(input);
    let mut cur_r = 0;
    let mut cur_c = 0;
    let mut dy = -1;
    let mut dx = 0;
    for ((r, c), cell) in grid.cells_mut() {
        if cell == &'^' {
            cur_r = r as i32;
            cur_c = c as i32;
            *cell = 'X';
        }
    }

    loop {
        let next_r = cur_r + dy;
        let next_c = cur_c + dx;

        let Some(next_cell) = grid.try_get(next_r, next_c) else {
            break;
        };

        match next_cell {
            '#' => {
                match (dy, dx) {
                    //up to right
                    (-1, 0) => {
                        dy = 0;
                        dx = 1;
                    }
                    //right to down
                    (0, 1) => {
                        dy = 1;
                        dx = 0;
                    }
                    // down to left
                    (1, 0) => {
                        dy = 0;
                        dx = -1;
                    }
                    // left to up
                    (0, -1) => {
                        dy = -1;
                        dx = 0;
                    }
                    _ => {
                        panic!("bad dir")
                    }
                }
            }
            '.' | 'X' => {
                grid.set(next_r, next_c, 'X');
                cur_r = next_r;
                cur_c = next_c
            }
            _ => {
                panic!("unexpected char")
            }
        }
    }

    grid.cells().filter(|(_, cell)| **cell == 'X').count() as u64
}

pub fn part2(input: String) -> u64 {
    let mut orig_grid = parse_grid(input);
    let mut infinite_loops = HashSet::new();
    let (mut start_r, mut start_c) = (-1, -1);
    for ((r, c), cell) in orig_grid.cells_mut() {
        if cell == &'^' {
            start_r = r as i32;
            start_c = c as i32;
            // *cell = '.';
        }
    }

    'outer: for mut drop_n in 0.. {
        let mut grid = orig_grid.clone();
        let mut cur_r = start_r;
        let mut cur_c = start_c;
        let mut dy = -1;
        let mut dx = 0;
        let mut n = 0;
        let mut seen = HashSet::new();
        let mut drop_r = -1;
        let mut drop_c = -1;

        loop {
            let next_r = cur_r + dy;
            let next_c = cur_c + dx;

            // grid.print();
            // print!("step {n} {cur_r},{cur_c}");
            let Some(next_cell) = grid.try_get_mut(next_r, next_c) else {
                // we tried dropping but it didnt loop, try the next spot
                if n > drop_n {
                    continue 'outer;
                }

                // we didn't try dropping, which means we are out of spots
                return infinite_loops.len() as u64;
            };

            if n == drop_n {
                if ['#', '^', 'X'].contains(next_cell) {
                    // won't help, or is impossible to drop there
                    // try the next spot without resetting
                    drop_n += 1;
                } else {
                    // print!(" DROP");
                    drop_r = next_r;
                    drop_c = next_c;
                    *next_cell = 'O';
                }
            }

            let state = (cur_r, cur_c, dy, dx);
            // print!(" {state:?} {seen:?}");
            if seen.contains(&state) {
                // println!(" LOOP");
                infinite_loops.insert((drop_r, drop_c));
                // grid.print();
                // println!();
                continue 'outer;
            }
            seen.insert(state);

            match next_cell {
                '#' | 'O' => {
                    // print!(" TURN");
                    match (dy, dx) {
                        //up to right
                        (-1, 0) => {
                            dy = 0;
                            dx = 1;
                        }
                        //right to down
                        (0, 1) => {
                            dy = 1;
                            dx = 0;
                        }
                        // down to left
                        (1, 0) => {
                            dy = 0;
                            dx = -1;
                        }
                        // left to up
                        (0, -1) => {
                            dy = -1;
                            dx = 0;
                        }
                        _ => {
                            panic!("bad dir")
                        }
                    }
                }
                '.' | 'X' | '^' => {
                    // print!(" STEP");
                    grid.set(next_r, next_c, 'X');
                    cur_r = next_r;
                    cur_c = next_c
                }
                _ => {
                    panic!("unexpected char")
                }
            };
            n += 1;
            // println!();
        }
    }
    unreachable!();
}

aoc_test!(
    "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
    41,
    6,
);

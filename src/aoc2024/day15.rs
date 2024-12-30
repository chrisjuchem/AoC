use crate::grid::{DeltaLoc, Loc, parse_grid};
use crate::util::{SplitInto, aoc_test};
use std::collections::{BTreeSet, HashMap};

pub fn part1(input: String) -> usize {
    let (grid, moves): (&str, &str) = input.split_into("\n\n");
    let mut grid = parse_grid(grid);

    let mut pos = grid.find(&'@').unwrap();

    for m in moves.chars() {
        let delta = match m {
            '>' => DeltaLoc::new(0, 1),
            'v' => DeltaLoc::new(1, 0),
            '^' => DeltaLoc::new(-1, 0),
            '<' => DeltaLoc::new(0, -1),
            _ => continue,
        };

        let check = pos + delta;
        let see = grid.get(check.r, check.c);
        match see {
            '#' => continue,
            '.' => {
                *grid.get_mut(check.r, check.c) = '@';
                *grid.get_mut(pos.r, pos.c) = '.';
                pos = check;
            }
            'O' => {
                let mut check_box = check;
                while grid.get(check_box.r, check_box.c) == 'O' {
                    check_box += delta;
                }
                match grid.get(check_box.r, check_box.c) {
                    '#' => continue,
                    '.' => {
                        *grid.get_mut(check_box.r, check_box.c) = 'O';
                        *grid.get_mut(check.r, check.c) = '@';
                        *grid.get_mut(pos.r, pos.c) = '.';
                        pos = check;
                    }
                    _ => panic!("bad char behind boxes"),
                }
            }
            other => {
                println!("{other}");
                panic!("bad char");
            }
        }
    }
    grid.cells()
        .filter_map(|((r, c), cell)| (*cell == 'O').then_some(r * 100 + c))
        .sum()
}

pub fn part2(input: String) -> usize {
    let (grid, moves): (&str, &str) = input.split_into("\n\n");
    let grid = grid
        .to_string()
        .replace("#", "##")
        .replace("O", "[]")
        .replace(".", "..")
        .replace("@", "@.");
    let mut grid = parse_grid(grid);
    let mut pos = grid.find(&'@').unwrap();

    'main: for m in moves.chars() {
        let delta = match m {
            '>' => DeltaLoc::new(0, 1),
            'v' => DeltaLoc::new(1, 0),
            '^' => DeltaLoc::new(-1, 0),
            '<' => DeltaLoc::new(0, -1),
            _ => continue,
        };

        assert_eq!(grid.get(pos.r, pos.c), '@');

        let check = pos + delta;
        let see = grid.get(check.r, check.c);
        match see {
            '#' => continue,
            '.' => {
                *grid.get_mut(check.r, check.c) = '@';
                *grid.get_mut(pos.r, pos.c) = '.';
                pos = check;
            }
            '[' | ']' => {
                if delta.dr == 0 {
                    // easy case
                    let mut check_box = check;
                    while ['[', ']'].contains(&grid.get(check_box.r, check_box.c)) {
                        check_box += delta;
                    }
                    match grid.get(check_box.r, check_box.c) {
                        '#' => continue,
                        '.' => {
                            while check_box != pos {
                                let prev = check_box - delta;
                                *grid.get_mut(check_box.r, check_box.c) = grid.get(prev.r, prev.c);
                                check_box = prev;
                            }
                            *grid.get_mut(pos.r, pos.c) = '.';
                            pos = check;
                        }
                        _ => panic!("bad char behind boxes"),
                    }
                } else {
                    // hard case

                    let mut cells_to_move = BTreeSet::new();
                    let mut cells_to_check = BTreeSet::new();
                    cells_to_check.insert(pos);

                    while cells_to_check.len() > 0 {
                        let cell = cells_to_check.pop_first().unwrap();
                        let check_cell = cell + delta;

                        match grid.get(check_cell.r, check_cell.c) {
                            '.' => {
                                cells_to_move.insert(cell);
                            }
                            '#' => {
                                continue 'main;
                            }
                            half @ ('[' | ']') => {
                                for c in [
                                    check_cell,
                                    Loc::new(
                                        check_cell.r,
                                        if half == '[' {
                                            check_cell.c + 1
                                        } else {
                                            check_cell.c - 1
                                        },
                                    ),
                                ] {
                                    if !cells_to_move.contains(&c) {
                                        cells_to_check.insert(c);
                                    }
                                }
                                cells_to_move.insert(cell);
                            }
                            _ => panic!("bad char behind box"),
                        }
                    }
                    // not blocked, move everything
                    let mut cells = HashMap::new();
                    for l in &cells_to_move {
                        cells.insert(*l + delta, grid.get(l.r, l.c));
                        grid.set(l.r, l.c, '.');
                    }
                    for (l, c) in cells {
                        grid.set(l.r, l.c, c);
                    }
                    pos = check;
                }
            }
            other => {
                println!("{other}");
                panic!("bad char");
            }
        }
    }

    grid.cells()
        .filter_map(|((r, c), cell)| (*cell == '[').then_some(r * 100 + c))
        .sum()
}

aoc_test!(
    "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
",
    10092,
    9021,
);

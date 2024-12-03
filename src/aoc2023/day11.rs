use crate::util::aoc_test;
fn dist(input: String, expand_size: usize) -> u64 {
    let universe: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut empty_cols = vec![];
    let mut c = 0;
    while c < universe[0].len() {
        if universe.iter().map(|row| row[c]).all(|chr| chr == '.') {
            empty_cols.push(c);
        }
        c += 1;
    }

    let mut empty_rows = vec![];
    let mut r = 0;
    while r < universe.len() {
        if universe[r].iter().all(|c| *c == '.') {
            empty_rows.push(r)
        }
        r += 1;
    }

    let galaxies = universe
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(c, chr)| match chr {
                    '#' => Some((r, c)),
                    _ => None,
                })
        })
        .flatten()
        .collect::<Vec<_>>();

    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let (r1, c1) = galaxies[i];
            let (r2, c2) = galaxies[j];
            fn count_expanded(empty_list: &Vec<usize>, r1: usize, r2: usize) -> usize {
                empty_list
                    .iter()
                    .filter(|r| {
                        if r1 < r2 {
                            **r > r1 && **r < r2
                        } else {
                            **r > r2 && **r < r1
                        }
                    })
                    .count()
            }

            sum += r1.abs_diff(r2) + count_expanded(&empty_rows, r1, r2) * expand_size;
            sum += c1.abs_diff(c2) + count_expanded(&empty_cols, c1, c2) * expand_size;
        }
    }
    println!("{sum}");
    sum as u64
}

pub fn part1(_input: String) -> u64 {
    dist(_input, 1)
}
pub fn part2(_input: String) -> u64 {
    dist(_input, 999999)
}

aoc_test!(
    "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
    374,
    82000210,
);

use crate::util::aoc_test;

pub fn part1(input: String) -> u64 {
    let mut locks = vec![];
    let mut keys = vec![];

    for schematic in input.split("\n\n") {
        let mut s = [0u64; 5];

        for l in schematic.lines().skip(1).take(5) {
            for i in 0..5usize {
                if l[i..i + 1] == *"#" {
                    s[i] += 1;
                }
            }
        }
        if schematic.starts_with("#") {
            locks.push(s);
        } else {
            keys.push(s);
        }
    }

    let mut n_fits = 0;
    for lock in &locks {
        for key in &keys {
            if lock
                .iter()
                .zip(key.iter())
                .map(|(l, k)| l + k)
                .max()
                .unwrap()
                <= 5
            {
                n_fits += 1;
            }
        }
    }

    n_fits
}

pub fn part2(_input: String) -> u64 {
    0
}

aoc_test!(
    "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
",
    3,
    0,
);

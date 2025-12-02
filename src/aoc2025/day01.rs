use num::Integer;

use crate::util::aoc_test;

pub fn part1(input: String) -> u64 {
    let mut dial = 50i32;
    let mut zeros = 0;
    for line in input.lines() {
        let mut mul = 1;
        if line.chars().next().unwrap() == 'L' {
            mul = -1;
        }

        let n = line[1..].parse::<i32>().unwrap();
        dial += n * mul;

        if dial.is_multiple_of(&100) {
            zeros += 1;
        }
    }

    zeros
}

pub fn part2(input: String) -> u64 {
    let mut dial = 50i32;
    let mut zeros = 0;
    for line in input.lines() {
        let mut mul = 1;
        if line.chars().next().unwrap() == 'L' {
            mul = -1;
        }
        let n = mul * line[1..].parse::<i32>().unwrap();

        let start = dial;
        dial += n;

        if start != 0 && start * dial <= 0 {
            zeros += 1;
        }

        while dial < -99 {
            zeros += 1;
            dial += 100;
        }
        while dial > 99 {
            zeros += 1;
            dial -= 100;
        }
    }

    zeros
}

aoc_test!(
    "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
    3,
    6,
);

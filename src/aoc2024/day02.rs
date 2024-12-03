use crate::util::{aoc_test, CollectVec};

pub fn is_safe(levels: &[i64]) -> bool {
    let mut diffs = levels
        .windows(2)
        .map(|lvls| lvls[0] - lvls[1])
        .collect_vec();

    if diffs[0] < 0 {
        diffs.iter_mut().for_each(|d| *d *= -1)
    }
    diffs.into_iter().all(|d| d == 1 || d == 2 || d == 3)
}

pub fn part1(input: String) -> u64 {
    let mut n_safe = 0;
    for line in input.lines() {
        let levels = line
            .split(" ")
            .map(|n| n.parse::<i64>().unwrap())
            .collect_vec();
        if is_safe(&levels) {
            n_safe += 1
        }
    }
    n_safe
}

pub fn part2(input: String) -> u64 {
    let mut n_safe = 0;
    'line: for line in input.lines() {
        let levels = line
            .split(" ")
            .map(|n| n.parse::<i64>().unwrap())
            .collect_vec();

        for i in 0..levels.len() {
            let mut levels = levels.clone();
            levels.remove(i);
            if is_safe(&levels) {
                n_safe += 1;
                continue 'line;
            }
        }
    }
    n_safe
}

aoc_test!(
    "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
    2,
    4,
);

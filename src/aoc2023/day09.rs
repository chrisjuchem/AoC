use crate::util::aoc_test;

fn next_val(list: Vec<i64>) -> i64 {
    if list.iter().all(|d| *d == 0) {
        return 0;
    }
    let diffs = list
        .windows(2)
        .map(|slice| slice[1] - slice[0])
        .collect::<Vec<_>>();

    list.last().unwrap() + next_val(diffs)
}

fn prev_val(list: Vec<i64>) -> i64 {
    if list.iter().all(|d| *d == 0) {
        return 0;
    }
    let diffs = list
        .windows(2)
        .map(|slice| slice[1] - slice[0])
        .collect::<Vec<_>>();

    list.first().unwrap() - prev_val(diffs)
}

pub fn part1(input: String) -> u64 {
    input
        .lines()
        .map(|line| {
            let list = line.split(" ").map(str::parse).map(Result::unwrap);
            next_val(list.collect())
        })
        .sum::<i64>() as u64
}

pub fn part2(input: String) -> u64 {
    input
        .lines()
        .map(|line| {
            let list = line.split(" ").map(str::parse).map(Result::unwrap);
            prev_val(list.collect())
        })
        .sum::<i64>() as u64
}

aoc_test!(
    "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
    114,
    2,
);

use crate::util::{aoc_test, SplitInto};
use std::collections::HashMap;
use std::iter::zip;

pub fn part1(input: String) -> u64 {
    let mut ls = vec![];
    let mut rs = vec![];
    for line in input.lines() {
        let (l, r) = line.split_into("   ");
        ls.push(l.parse::<u64>().unwrap());
        rs.push(r.parse::<u64>().unwrap());
    }
    ls.sort();
    rs.sort();
    zip(ls.into_iter(), rs.into_iter())
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

pub fn part2(input: String) -> u64 {
    let mut ls = vec![];
    let mut rs = vec![];
    for line in input.lines() {
        let (l, r) = line.split_into("   ");
        ls.push(l.parse::<u64>().unwrap());
        rs.push(r.parse::<u64>().unwrap());
    }

    let mut counts_r = HashMap::new();
    for r in rs {
        *counts_r.entry(r).or_insert(0u64) += 1;
    }

    let mut score = 0;
    for l in ls {
        score += l * counts_r.get(&l).unwrap_or(&0)
    }
    score
}

aoc_test!(
    "3   4
4   3
2   5
1   3
3   9
3   3",
    11,
    31,
);

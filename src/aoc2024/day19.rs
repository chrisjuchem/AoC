use crate::util::{aoc_test, CollectVec, SplitInto};
use regex::Regex;
use std::collections::HashMap;

pub fn part1(input: String) -> usize {
    let (towels, patterns): (&str, &str) = input.split_into("\n\n");
    let re = format!("^({})+$", towels.split(", ").collect_vec().join("|"));
    let re = Regex::new(&re).unwrap();
    patterns.lines().filter(|p| re.find(p).is_some()).count()
}

pub fn part2(input: String) -> u64 {
    let mut ways = 0;
    let (towels, patterns): (&str, &str) = input.split_into("\n\n");
    let towels = towels.split(", ").collect_vec();

    let mut cache = HashMap::new();
    for p in patterns.lines() {
        ways += recurse(p, &towels, &mut cache);
    }
    ways
}

pub fn recurse<'a>(pattern: &'a str, towels: &[&str], cache: &mut HashMap<&'a str, u64>) -> u64 {
    if pattern.len() == 0 {
        return 1;
    }
    if let Some(n) = cache.get(pattern) {
        return *n;
    }

    let mut ways = 0;
    for t in towels {
        if pattern.starts_with(t) {
            ways += recurse(&pattern[t.len()..], towels, cache);
        }
    }

    cache.insert(pattern, ways);
    ways
}

aoc_test!(
    "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
",
    6,
    16,
);

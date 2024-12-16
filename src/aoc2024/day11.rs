use crate::util::{aoc_test, CountMap};
use std::collections::HashMap;

fn blink(input: String, n_blinks: usize) -> u64 {
    let mut stones = HashMap::new();
    input
        .trim()
        .split(" ")
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .for_each(|n| stones.insert_one(n));

    for _ in 0..n_blinks {
        let mut next = HashMap::new();

        for (n, c) in stones {
            let s = n.to_string();
            if &s == "0" {
                next.insert_n(1, c)
            } else if s.len() % 2 == 0 {
                next.insert_n(s[0..s.len() / 2].parse::<u64>().unwrap(), c);
                next.insert_n(s[s.len() / 2..s.len()].parse::<u64>().unwrap(), c);
            } else {
                next.insert_n(n * 2024, c)
            }
        }

        stones = next;
    }

    stones.into_iter().map(|(_, c)| c).sum()
}

pub fn part1(input: String) -> u64 {
    blink(input, 25)
}
pub fn part2(input: String) -> u64 {
    blink(input, 75)
}

aoc_test!("125 17", 55312, 65601038650482);

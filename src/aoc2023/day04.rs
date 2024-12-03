use crate::util::aoc_test;
use regex::Regex;
use std::collections::HashMap;
use std::ops::Add;

pub fn part1(_input: String) -> u64 {
    let mut total = 0;
    for line in _input.trim().split("\n") {
        let re = Regex::new(r"^Card ([\d ]+): ([\d ]+)\|([\d ]+)$").unwrap();
        let [_, winners, ours] = re.captures(line).unwrap().extract::<3>().1;
        let winners: Vec<_> = winners.split(" ").filter(|n| *n != "").collect();
        total += match ours.split(" ").filter(|n| winners.contains(n)).count() {
            0 => 0,
            n => 2u64.pow(n as u32 - 1),
        };
    }
    total
}

pub fn part2(_input: String) -> u64 {
    let mut copies = HashMap::new();
    for (card, line) in _input.trim().split("\n").enumerate() {
        let n = *copies.entry(card).or_insert(1u64);
        let re = Regex::new(r"^Card ([\d ]+): ([\d ]+)\|([\d ]+)$").unwrap();
        let [_, winners, ours] = re.captures(line).unwrap().extract::<3>().1;
        let winners: Vec<_> = winners.split(" ").filter(|n| *n != "").collect();
        let count = ours.split(" ").filter(|n| winners.contains(n)).count();
        for i in card + 1..card + 1 + count {
            *copies.entry(i).or_insert(1) += n;
        }
    }
    copies.values().fold(0u64, u64::add)
}

aoc_test!(
    "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    13,
    30,
);

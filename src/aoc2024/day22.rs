use crate::util::{CollectVec, CountMap, aoc_test};
use std::collections::{HashMap, HashSet};

struct Secrets {
    val: u64,
}
impl Secrets {
    fn new(n: u64) -> Self {
        Self { val: n }
    }
}
impl Iterator for Secrets {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let n = Some(self.val);
        let m = 16777216;
        self.val ^= self.val << 6;
        self.val %= m;
        self.val ^= self.val >> 5;
        self.val %= m;
        self.val ^= self.val << 11;
        self.val %= m;
        n
    }
}

pub fn part1(input: String) -> u64 {
    input
        .lines()
        .map(|line| {
            Secrets::new(line.parse::<u64>().unwrap())
                .nth(2000)
                .unwrap()
        })
        .sum()
}

pub fn part2(input: String) -> u64 {
    let monkeys = input.lines().map(|line| {
        Secrets::new(line.parse::<u64>().unwrap())
            .take(2001)
            .collect_vec()
    });

    let mut seqs = HashMap::new();

    for m in monkeys {
        let mut seen = HashSet::new();

        for seq in m.windows(5) {
            let a = (seq[0] % 10) as i64;
            let b = (seq[1] % 10) as i64;
            let c = (seq[2] % 10) as i64;
            let d = (seq[3] % 10) as i64;
            let e = (seq[4] % 10) as i64;

            let s = (b - a, c - b, d - c, e - d);
            if !seen.contains(&s) {
                seen.insert(s);
                seqs.insert_n(s, e as u64);
            }
        }
    }

    *seqs.values().max().unwrap()
}

aoc_test!(
    "1
10
100
2024
",
    37327623,
    "1
2
3
2024
",
    23,
);

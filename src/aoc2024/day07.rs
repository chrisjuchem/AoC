use crate::util::{aoc_test, CollectVec, SplitInto};
use std::collections::HashSet;

enum Ops {
    Plus,
    Times,
    Concat,
}

impl Ops {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Ops::Plus => a + b,
            Ops::Times => a * b,
            Ops::Concat => a * 10i32.pow(b.ilog10() + 1) as u64 + b,
        }
    }
}

fn check_equations(input: String, ops: &[Ops]) -> u64 {
    let mut total = 0;
    for line in input.lines() {
        let (result, nums): (&str, &str) = line.split_into(": ");
        let result = result.parse::<u64>().unwrap();
        let mut nums = nums
            .split(" ")
            .map(str::parse::<u64>)
            .map(Result::unwrap)
            .collect_vec();

        let mut results = HashSet::from([nums.remove(0)]);
        let mut prev_results = HashSet::new();
        for n in nums {
            std::mem::swap(&mut results, &mut prev_results);
            results.clear();
            for op in ops {
                for prev in prev_results.iter() {
                    results.insert(op.apply(*prev, n));
                }
            }
        }

        if results.contains(&result) {
            total += result;
        }
    }
    total
}

pub fn part1(input: String) -> u64 {
    check_equations(input, &[Ops::Plus, Ops::Times])
}
pub fn part2(input: String) -> u64 {
    check_equations(input, &[Ops::Plus, Ops::Times, Ops::Concat])
}

#[cfg(test)]
mod test {
    use super::Ops;

    #[test]
    fn test_concat() {
        assert_eq!(Ops::Concat.apply(15, 6), 156);
        assert_eq!(Ops::Concat.apply(6, 15), 615);
        assert_eq!(Ops::Concat.apply(6, 10), 610);
        assert_eq!(Ops::Concat.apply(6, 9), 69);
    }
}

aoc_test!(
    "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
    3749,
    11387,
);

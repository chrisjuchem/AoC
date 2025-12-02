use num::Integer;

use crate::util::{SplitInto, aoc_test};

pub fn part1(input: String) -> u64 {
    let mut sum = 0;

    for range_str in input.split(',') {
        let (min, max) = range_str.trim().split_into("-");
        let min: u64 = min.parse().unwrap();
        let max = max.parse().unwrap();

        for n in min..=max {
            let s = format!("{n}");
            if s.len().is_odd() {
                continue;
            };
            let len = s.len() / 2;
            if s[..len] == s[len..] {
                sum += n;
            }
        }
    }

    sum
}

pub fn part2(input: String) -> u64 {
    let mut sum = 0;

    for range_str in input.split(',') {
        let (min, max) = range_str.trim().split_into("-");
        let min: u64 = min.parse().unwrap();
        let max = max.parse().unwrap();

        'n: for n in min..=max {
            let s = format!("{n}");
            let len = s.len();

            'pat: for pat_len in 1..=len / 2 {
                if !len.is_multiple_of(pat_len) {
                    continue;
                }

                for i in 1..len / pat_len {
                    if s[0..pat_len] != s[pat_len * i..pat_len * (i + 1)] {
                        continue 'pat;
                    }
                }

                sum += n;
                continue 'n;
            }
        }
    }

    sum
}

aoc_test!(
    "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124",
    1227775554,
    4174379265,
);

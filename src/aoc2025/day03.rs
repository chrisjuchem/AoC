use crate::util::aoc_test;

fn num<const N: usize>(digits: [u64; N]) -> u64 {
    let mut tot = 0;
    for d in digits {
        tot = tot * 10 + d;
    }
    tot
}

fn insert_digit<const N: usize>(orig: &mut [u64; N], d: u64) {
    let mut best = orig.clone();

    let mut digits = [d; N];
    digits[0..N - 1].copy_from_slice(&best[1..N]);

    for n in 0..N {
        if num(digits) > num(best) {
            best = digits;
        }

        if n == N {
            break;
        }
        digits[n] = orig[n];
    }

    *orig = best;
}

fn max_joltage<const N: usize>(line: &str) -> u64 {
    let mut digits = line.trim().chars().map(|c| c.to_digit(10).unwrap() as u64);
    let mut cur: [u64; N] = digits.next_chunk().unwrap();

    for d in digits {
        insert_digit(&mut cur, d);
    }

    return num(cur);
}

pub fn part1(input: String) -> u64 {
    let mut total = 0;
    for line in input.lines() {
        let amt = max_joltage::<2>(line);
        total += amt;
    }
    total as u64
}

pub fn part2(input: String) -> u64 {
    let mut total = 0;
    for line in input.lines() {
        let amt = max_joltage::<12>(line);
        total += amt;
    }
    total as u64
}

aoc_test!(
    "987654321111111
811111111111119
234234234234278
818181911112111",
    357,
    3121910778619,
);

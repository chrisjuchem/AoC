use crate::util::aoc_test;
use std::ops::{Add, Mul};

pub fn part1(input: String) -> u64 {
    let mut lines = input.lines().rev().map(|l| l.split_ascii_whitespace());
    let mut ops = lines.next().unwrap();
    let mut nums = lines.collect::<Vec<_>>();

    let mut total = 0;
    while let Some(op) = ops.next() {
        let ns = nums
            .iter_mut()
            .map(|n| n.next().unwrap().parse::<u64>().unwrap());

        total += match op {
            "*" => ns.product::<u64>(),
            "+" => ns.sum(),
            _ => panic!(""),
        };
    }

    total
}

pub fn part2(input: String) -> u64 {
    let mut lines = input.lines().rev();
    let mut ops = lines.next().unwrap().split_ascii_whitespace().rev();
    let mut cols = lines
        .rev()
        .map(|l| l.chars().rev().chain(std::iter::once(' ')))
        .collect::<Vec<_>>();

    let mut total = 0;
    while let Some(op) = ops.next() {
        let mut nums = Vec::new();
        loop {
            let n = cols
                .iter_mut()
                .filter_map(|cs| {
                    let c = cs.next().unwrap();
                    if c.is_ascii_whitespace() {
                        None
                    } else {
                        Some(c.to_digit(10).unwrap() as u64)
                    }
                })
                .fold(0, |n, digit| n * 10 + digit);
            if n == 0 {
                break;
            }
            nums.push(n)
        }

        let (init, f): (u64, fn(u64, u64) -> u64) = match op {
            "*" => (1, u64::mul),
            "+" => (0, u64::add),
            _ => panic!(""),
        };
        let ans = nums.into_iter().fold(init, f);
        total += ans;
    }
    total
}

aoc_test!(
    "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ",
    4277556,
    3263827,
);

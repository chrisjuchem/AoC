use crate::aoc2019::intcode::Program;
use crate::util::aoc_test;

pub fn part1(input: String) -> i64 {
    let mut p = input.parse::<Program>().unwrap().with_input(vec![1]);

    let out = p.run().unwrap();
    out[out.len() - 1]
}

pub fn part2(_input: String) -> u64 {
    0
}

aoc_test!("3,0,4,0,99\n", 1, 0,);

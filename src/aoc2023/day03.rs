use crate::util::aoc_test;
use std::ops::Add;

#[derive(Debug)]
struct Posn {
    x: usize,
    y: usize,
}
impl Posn {
    fn is_near(&self, other: &Posn) -> bool {
        self.x.abs_diff(other.x) < 2 && self.y.abs_diff(other.y) < 2
    }
}

#[derive(Debug)]
struct Number {
    n: usize,
    posns: Vec<Posn>,
}

impl Number {
    fn is_near(&self, part: &Part) -> bool {
        self.posns.iter().any(|p| p.is_near(&part.pos))
    }
}

#[derive(Debug)]
struct Part {
    sym: char,
    pos: Posn,
}

fn nums_and_parts(input: String) -> (Vec<Number>, Vec<Part>) {
    let mut nums = vec![];
    let mut parts = vec![];
    for (y, line) in input.split("\n").enumerate() {
        let mut x = 0;
        while x < line.len() {
            match line.chars().nth(x).unwrap() {
                '.' => {
                    x += 1;
                }
                n if n.is_ascii_digit() => {
                    let mut ndigits = 1usize;
                    while line
                        .chars()
                        .nth(x + ndigits)
                        .unwrap_or('_')
                        .is_ascii_digit()
                    {
                        ndigits += 1;
                    }
                    let num = line.get(x..x + ndigits).unwrap().parse::<usize>().unwrap();
                    nums.push(Number {
                        n: num,
                        posns: (0..ndigits).map(|i| Posn { x: x + i, y }).collect(),
                    });
                    x += ndigits;
                }
                p => {
                    parts.push(Part {
                        sym: p,
                        pos: Posn { x, y },
                    });
                    x += 1;
                }
            }
        }
    }
    (nums, parts)
}

pub fn part1(input: String) -> u64 {
    let (nums, parts) = nums_and_parts(input);
    nums.iter()
        .filter_map(|n| parts.iter().any(|p| n.is_near(p)).then_some(n.n))
        .fold(0usize, <usize as Add>::add) as u64
}

pub fn part2(input: String) -> u64 {
    let (nums, parts) = nums_and_parts(input);
    parts
        .iter()
        .filter(|p| p.sym == '*')
        .map(|p| {
            let nums = nums.iter().filter(|n| n.is_near(p)).collect::<Vec<_>>();
            if nums.len() == 2 {
                nums[0].n * nums[1].n
            } else {
                0
            }
        })
        .fold(0usize, <usize as Add>::add) as u64
}

aoc_test!(
    "\
467....114
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
    4361,
    467835,
);

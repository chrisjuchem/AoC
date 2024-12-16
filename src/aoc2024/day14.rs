use crate::util::{aoc_test, ToCountMap};
use regex::Regex;
use std::cmp::{Ord, Ordering};

pub fn part1(input: String) -> u64 {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let quadrant = input
        .trim()
        .lines()
        .map(|line| {
            let c = re.captures(line).unwrap();
            let (_, [c0, r0, dc, dr]) = c.extract();

            let c0 = c0.parse::<i64>().unwrap();
            let r0 = r0.parse::<i64>().unwrap();
            let dc = dc.parse::<i64>().unwrap();
            let dr = dr.parse::<i64>().unwrap();

            #[cfg(test)]
            let (w, h) = (11, 7);
            #[cfg(not(test))]
            let (w, h) = (101, 103);

            let c = (c0 + (100 * dc)).rem_euclid(w);
            let r = (r0 + (100 * dr)).rem_euclid(h);
            match (r.cmp(&(h / 2)), c.cmp(&(w / 2))) {
                (Ordering::Equal, _) | (_, Ordering::Equal) => 0,
                (Ordering::Less, Ordering::Less) => 1,
                (Ordering::Greater, Ordering::Less) => 2,
                (Ordering::Less, Ordering::Greater) => 3,
                (Ordering::Greater, Ordering::Greater) => 4,
            }
        })
        .to_count_map();
    quadrant[&1] * quadrant[&2] * quadrant[&3] * quadrant[&4]
}

pub fn part2(_input: String) -> u64 {
    0
}

aoc_test!(
    "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
",
    12,
    0,
);

use crate::util::{aoc_test, SplitInto};
use std::ops::Mul;

fn ways(t: f64, d: f64) -> u64 {
    let thresh = (t - ((t * t) - (4. * d)).sqrt()) / 2.;
    (t - (2. * thresh.floor())) as u64 - 1
}

pub fn part1(_input: String) -> u64 {
    let (t, d) = _input.split_into("\n");
    let times = t
        .split(" ")
        .skip(1)
        .filter(|s| *s != "")
        .map(str::parse::<f64>)
        .map(Result::unwrap);
    let dists = d
        .split(" ")
        .skip(1)
        .filter(|s| *s != "")
        .map(str::parse::<f64>)
        .map(Result::unwrap);

    // n = 7
    // d = 9
    // x * (n - x) > d
    // -x^2 + nx - d = 0
    // x^2 - nx + d = 0
    // (-b +- sqrt(b*b - 4ac)) / 2a
    // (n +- sqrt(n*n - 4d)) / 2 *****
    // (7 +- sqrt(49-36))/2
    // (7 +- sqrt(13))/2
    // (7 +- 3.6)/2
    //  5.3, 1.7 -> 2,3,4,5-> 4

    times.zip(dists).map(|(t, d)| ways(t, d)).fold(1, u64::mul)
}

pub fn part2(_input: String) -> u64 {
    let (t, d) = _input.split_into("\n");
    let time = t
        .split(" ")
        .skip(1)
        .filter(|s| *s != "")
        .collect::<String>()
        .parse()
        .unwrap();
    let dist = d
        .split(" ")
        .skip(1)
        .filter(|s| *s != "")
        .collect::<String>()
        .parse()
        .unwrap();
    ways(time, dist)
}

aoc_test!(
    "\
Time:      7  15   30
Distance:  9  40  200
",
    288,
    71503,
);

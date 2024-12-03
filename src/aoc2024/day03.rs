use crate::util::aoc_test;

pub fn part1(input: String) -> u64 {
    let re = regex::Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();

    re.captures_iter(&input)
        .map(|caps| {
            let (_, [a, b]) = caps.extract();
            a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap()
        })
        .sum()
}

pub fn part2(input: String) -> u64 {
    let re = regex::Regex::new(r"do\(\)()()|don't\(\)()()|mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();

    let mut enabled = true;
    re.captures_iter(&input)
        .filter_map(|caps| {
            let (s, [a, b]) = caps.extract();
            if s == "do()" {
                enabled = true;
                return None;
            } else if s == "don't()" {
                enabled = false;
                return None;
            }
            enabled.then_some(a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap())
        })
        .sum()
}

aoc_test!(
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
    161,
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
    48,
);

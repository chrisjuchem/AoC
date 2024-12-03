use crate::util::aoc_test;

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
pub fn part1(input: String) -> u64 {
    let mut sum = 0;
    for line in input.trim().split("\n") {
        let found = format!(
            "{}{}",
            line.find(DIGITS).and_then(|i| line.get(i..i + 1)).unwrap(),
            line.rfind(DIGITS).and_then(|i| line.get(i..i + 1)).unwrap()
        );
        sum += found.parse::<u64>().unwrap();
    }
    sum
}

pub fn part2(input: String) -> u64 {
    let mut sum = 0;
    for mut line in input.trim().split("\n") {
        let (mut first, mut last) = (None::<u8>, None::<u8>);
        while line != "" {
            let n = match &line.chars().collect::<Vec<_>>()[..] {
                ['0', ..] | ['z', 'e', 'r', 'o', ..] => Some(0),
                ['1', ..] | ['o', 'n', 'e', ..] => Some(1),
                ['2', ..] | ['t', 'w', 'o', ..] => Some(2),
                ['3', ..] | ['t', 'h', 'r', 'e', 'e', ..] => Some(3),
                ['4', ..] | ['f', 'o', 'u', 'r', ..] => Some(4),
                ['5', ..] | ['f', 'i', 'v', 'e', ..] => Some(5),
                ['6', ..] | ['s', 'i', 'x', ..] => Some(6),
                ['7', ..] | ['s', 'e', 'v', 'e', 'n', ..] => Some(7),
                ['8', ..] | ['e', 'i', 'g', 'h', 't', ..] => Some(8),
                ['9', ..] | ['n', 'i', 'n', 'e', ..] => Some(9),
                _ => None,
            };
            first = first.or(n);
            last = n.or(last);
            line = &line[1..];
        }
        let found = format!("{}{}", first.unwrap(), last.unwrap());
        sum += found.parse::<u64>().unwrap();
    }
    sum
}

aoc_test!(
    "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet",
    142,
    "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen",
    281,
);

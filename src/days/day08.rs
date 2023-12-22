use crate::util::aoc_test;
use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;

fn parse_maps<'a>(lines: impl Iterator<Item = &'a str>) -> HashMap<&'a str, (&'a str, &'a str)> {
    let re = Regex::new(r"^(\S+) = \((\S+), (\S+)\)$").unwrap();
    lines
        .map(move |line| {
            let [node, left, right] = re.captures(line).unwrap().extract::<3>().1;
            (node, (left, right))
        })
        .collect()
}

pub fn part1(input: String) -> u64 {
    let mut lines = input.lines();
    let turns = lines.next().unwrap();
    let maps = parse_maps(lines.skip(1));
    let mut cur = "AAA";
    let mut steps = 0;
    for turn in turns.chars().cycle() {
        let map = maps.get(cur).unwrap();
        cur = match turn {
            'L' => map.0,
            'R' => map.1,
            _ => panic!("bad turn"),
        };
        steps += 1;
        if cur == "ZZZ" {
            return steps;
        }
    }
    panic!("end of steps")
}

pub fn part2(input: String) -> u64 {
    let mut lines = input.lines();
    let turns = lines.next().unwrap();
    let maps = parse_maps(lines.skip(1));
    maps.keys()
        .cloned()
        .filter(|spot| spot.ends_with("A"))
        .map(|mut path| {
            let mut steps = 0u64;

            for turn in turns.chars().cycle() {
                let map = maps.get(path).unwrap();
                path = match turn {
                    'L' => map.0,
                    'R' => map.1,
                    _ => panic!("bad turn"),
                };
                steps += 1;
                if path.ends_with("Z") {
                    return steps;
                }
            }
            panic!("end of steps")
        })
        .fold(1, lcm)
}

aoc_test!(
    "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
",
    6,
    "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
    6,
);

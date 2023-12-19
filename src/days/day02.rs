use crate::util::aoc_test;
use std::cmp::max;
use std::collections::HashMap;

pub fn part1(_input: String) -> u64 {
    let mut sum = 0;
    'outer: for line in _input.trim().split("\n") {
        let [game, rounds] = line.split(": ").collect::<Vec<_>>().try_into().unwrap();
        for round in rounds.split("; ") {
            for pull in round.split(", ") {
                let [n, color] = pull.split(" ").collect::<Vec<_>>().try_into().unwrap();

                match (n.parse::<u64>().unwrap(), color) {
                    (n, "red") => {
                        if n > 12 {
                            continue 'outer;
                        }
                    }
                    (n, "green") => {
                        if n > 13 {
                            continue 'outer;
                        }
                    }
                    (n, "blue") => {
                        if n > 14 {
                            continue 'outer;
                        }
                    }
                    _ => panic!("bad color"),
                }
            }
        }

        sum += game[5..].parse::<u64>().unwrap();
    }
    sum
}

pub fn part2(_input: String) -> u64 {
    let mut sum = 0;
    for line in _input.trim().split("\n") {
        let [_, rounds] = line.split(": ").collect::<Vec<_>>().try_into().unwrap();
        let mut counts = HashMap::new();
        for round in rounds.split("; ") {
            for pull in round.split(", ") {
                let [n, color] = pull.split(" ").collect::<Vec<_>>().try_into().unwrap();

                let color_entry = counts.entry(color).or_insert(0);
                *color_entry = max(*color_entry, n.parse::<u64>().unwrap())
            }
        }

        let power =
            counts.get("red").unwrap() * counts.get("blue").unwrap() * counts.get("green").unwrap();
        sum += power;
    }
    sum
}

aoc_test!(
    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
",
    8,
    2286,
);

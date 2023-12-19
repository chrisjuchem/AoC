use anyhow::{bail, Context};
use clap::Parser;
use std::fs;

mod util;
mod days {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
    pub mod day06;
    pub mod day07;
    pub mod day08;
    pub mod day09;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
    pub mod day14;
    pub mod day15;
    pub mod day16;
    pub mod day17;
    pub mod day18;
    pub mod day19;
    pub mod day20;
    pub mod day21;
    pub mod day22;
    pub mod day23;
    pub mod day24;
    pub mod day25;
}
pub use days::*;

#[derive(Parser)]
struct Cli {
    day: u8,
    part: u8,
}

type AocFn = &'static dyn Fn(String) -> u64;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let (part1, part2) = match cli.day {
        1 => (&day01::part1 as AocFn, &day01::part2 as AocFn),
        2 => (&day02::part1 as AocFn, &day02::part2 as AocFn),
        3 => (&day03::part1 as AocFn, &day03::part2 as AocFn),
        4 => (&day04::part1 as AocFn, &day04::part2 as AocFn),
        5 => (&day05::part1 as AocFn, &day05::part2 as AocFn),
        6 => (&day06::part1 as AocFn, &day06::part2 as AocFn),
        7 => (&day07::part1 as AocFn, &day07::part2 as AocFn),
        8 => (&day08::part1 as AocFn, &day08::part2 as AocFn),
        9 => (&day09::part1 as AocFn, &day09::part2 as AocFn),
        10 => (&day10::part1 as AocFn, &day10::part2 as AocFn),
        11 => (&day11::part1 as AocFn, &day11::part2 as AocFn),
        12 => (&day12::part1 as AocFn, &day12::part2 as AocFn),
        13 => (&day13::part1 as AocFn, &day13::part2 as AocFn),
        14 => (&day14::part1 as AocFn, &day14::part2 as AocFn),
        15 => (&day15::part1 as AocFn, &day15::part2 as AocFn),
        16 => (&day16::part1 as AocFn, &day16::part2 as AocFn),
        17 => (&day17::part1 as AocFn, &day17::part2 as AocFn),
        18 => (&day18::part1 as AocFn, &day18::part2 as AocFn),
        19 => (&day19::part1 as AocFn, &day19::part2 as AocFn),
        20 => (&day20::part1 as AocFn, &day20::part2 as AocFn),
        21 => (&day21::part1 as AocFn, &day21::part2 as AocFn),
        22 => (&day22::part1 as AocFn, &day22::part2 as AocFn),
        23 => (&day23::part1 as AocFn, &day23::part2 as AocFn),
        24 => (&day24::part1 as AocFn, &day24::part2 as AocFn),
        25 => (&day25::part1 as AocFn, &day25::part2 as AocFn),
        _ => bail!("invalid day"),
    };
    let func = match cli.part {
        1 => part1,
        2 => part2,
        _ => bail!("invalid part"),
    };

    let cookie = fs::read_to_string("cookie.txt").context("reading cookie")?;
    let input = reqwest::blocking::Client::new()
        .get(format!(
            "https://adventofcode.com/2023/day/{}/input",
            cli.day
        ))
        .header("Cookie", cookie.trim())
        .send()
        .context("requesting input")?
        .text()
        .context("reading input")?;
    println!("{}", func(input));
    Ok(())
}

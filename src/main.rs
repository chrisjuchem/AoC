#![feature(iter_array_chunks)]

use anyhow::{bail, Context};
use clap::Parser;
use std::fs;

mod infra;
mod util;

mod aoc2023;
mod aoc2024;

type AocFn = &'static dyn Fn(String) -> u64;
#[derive(Copy, Clone)]
pub struct AocDay {
    pub part1: AocFn,
    pub part2: AocFn,
}
pub type AocYear = [AocDay; 25];

#[derive(Parser)]
struct Cli {
    #[clap(long, default_value = "2024")]
    year: u16,
    day: u8,
    part: u8,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let days = match cli.year {
        2023 | 23 => aoc2023::DAYS,
        2024 | 24 => aoc2024::DAYS,
        _ => bail!("invalid year"),
    };
    let day = days[cli.day as usize - 1];
    let func = match cli.part {
        1 => day.part1,
        2 => day.part2,
        _ => bail!("invalid part"),
    };

    let cookie = fs::read_to_string("cookie.txt").context("reading cookie")?;
    let input = reqwest::blocking::Client::new()
        .get(format!(
            "https://adventofcode.com/{}/day/{}/input",
            cli.year, cli.day,
        ))
        .header("Cookie", cookie.trim())
        .send()
        .context("requesting input")?
        .text()
        .context("reading input")?;
    println!("{}", func(input));
    Ok(())
}

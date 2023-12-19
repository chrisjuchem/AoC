use anyhow::{bail, Context};
use clap::Parser;
use std::fs;

mod util;
mod days {
    pub mod day01;
}
pub use days::*;

#[derive(Parser)]
struct Cli {
    day: u8,
    part: u8,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let (part1, part2) = match cli.day {
        1 => (day01::part1, day01::part2),
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

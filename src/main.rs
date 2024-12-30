#![feature(iter_array_chunks)]
#![feature(gen_blocks)]
#![feature(macro_metavar_expr)]

extern crate core;
extern crate priority_queue;

use anyhow::{Context, bail};
use clap::Parser;
use std::fs;

mod grid;
mod infra;
mod util;

mod aoc2023;
mod aoc2024;
mod aoc2025;

pub trait AocFn {
    fn call(&self, input: String) -> String;
}
impl<F, R> AocFn for F
where
    F: Fn(String) -> R,
    R: ToString,
{
    fn call(&self, input: String) -> String {
        self(input).to_string()
    }
}

#[derive(Copy, Clone)]
pub struct AocDay {
    pub part1: &'static dyn AocFn,
    pub part2: &'static dyn AocFn,
}
pub type AocYear = [AocDay; 25];

#[derive(Parser)]
struct Cli {
    #[clap(long, short, default_value = "2025")]
    year: u16,
    day: u8,
    part: u8,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let days = match cli.year {
        2023 => aoc2023::DAYS,
        2024 => aoc2024::DAYS,
        2025 => aoc2025::DAYS,
        _ => bail!("invalid year"),
    };
    let day = days[cli.day as usize - 1];
    let func = match cli.part {
        1 => day.part1,
        2 => day.part2,
        _ => bail!("invalid part"),
    };

    let cookie = fs::read_to_string("cookie.txt").context("reading cookie")?;
    let resp = reqwest::blocking::Client::new()
        .get(format!(
            "https://adventofcode.com/{}/day/{}/input",
            cli.year, cli.day,
        ))
        .header("Cookie", cookie.trim())
        .send()
        .context("requesting input")?;
    if !resp.status().is_success() {
        if let Ok(content) = resp.text() {
            println!("{content}");
        }
        panic!("Failed to fetch input");
    }
    let input = resp.text().context("reading input")?;
    println!("{}", func.call(input));
    Ok(())
}

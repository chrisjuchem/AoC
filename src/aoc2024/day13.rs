use crate::grid::{DeltaLoc, Loc};
use crate::util::{aoc_test, CollectVec};
use regex::Regex;

const N: f64 = 10000000000000.;

struct Game {
    a: DeltaLoc,
    b: DeltaLoc,
    target: Loc,
}
impl Game {
    fn solve(&self) -> Option<(i64, i64)> {
        let mut na = 0;
        let mut nb = 100;
        let start = Loc { r: 0, c: 0 };

        while na < 100 {
            let claw = start + self.a * na + self.b * nb;

            if claw.r == self.target.r && claw.c == self.target.c {
                return Some((na, nb));
            }

            if claw.r >= self.target.r || claw.c >= self.target.c {
                nb -= 1;
            } else {
                na += 1;
            }
        }
        None
    }

    fn solve_large(&self) -> Option<(i64, i64)> {
        let slope_a = self.a.dc as f64 / self.a.dr as f64;
        let slope_b = self.b.dc as f64 / self.b.dr as f64;

        // y= mx+b
        // A: 0,0                              ->  y = slope_a * x + 0
        // B: N,N  ->  N = slope_b * N + icpt  ->  y = slope_b * x + (N - slope_b * N)
        //                             0 = (slope_a - slope_b) * x - (N - slope_b * N)
        let x = (N - slope_b * N) / (slope_a - slope_b);
        // let y = slope_a * x;

        let nai = x as i64 / self.a.dr;
        let nbi = (N - x) as i64 / self.b.dr;

        let start = Loc {
            r: (self.a.dr * nai + self.b.dr * nbi) - N as i64,
            c: (self.a.dc * nai + self.b.dc * nbi) - N as i64,
        };

        let mut na = -1000;
        let mut nb = 1000;

        while na < 1000 && nb > -1000 {
            let claw = start + self.a * na + self.b * nb;

            if claw.r == self.target.r && claw.c == self.target.c {
                return Some((na + nai, nb + nbi));
            }

            if claw.r >= self.target.r || claw.c >= self.target.c {
                nb -= 1;
            } else {
                na += 1;
            }
        }
        None
    }
}

fn tokens(input: String, solve: impl FnMut(&Game) -> Option<(i64, i64)>) -> u64 {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    let games = input
        .split("\n\n")
        .map(|s| {
            let (_, [dax, day, dbx, dby, tx, ty]) = re.captures(s).unwrap().extract::<6>();
            let dax = dax.parse::<u64>().unwrap();
            let day = day.parse::<u64>().unwrap();
            let dbx = dbx.parse::<u64>().unwrap();
            let dby = dby.parse::<u64>().unwrap();
            let tx = tx.parse::<u64>().unwrap();
            let ty = ty.parse::<u64>().unwrap();

            Game {
                a: DeltaLoc::new(dax, day),
                b: DeltaLoc::new(dbx, dby),
                target: Loc::new(tx, ty),
            }
        })
        .collect_vec();

    games
        .iter()
        .filter_map(solve)
        .map(|(a, b)| (a * 3 + b) as u64)
        .sum()
}

pub fn part1(input: String) -> u64 {
    tokens(input, Game::solve)
}

pub fn part2(input: String) -> u64 {
    tokens(input, Game::solve_large)
}

aoc_test!(
    "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
    480,
    875318608908,
);

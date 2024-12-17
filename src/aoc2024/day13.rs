use crate::grid::{DeltaLoc, Loc};
use crate::util::{aoc_test, CollectVec};
use regex::Regex;

const START: Loc = Loc { r: 0, c: 0 };

struct Game {
    a: DeltaLoc,
    b: DeltaLoc,
    target: Loc,
}
impl Game {
    fn solve(&self) -> Option<(i64, i64)> {
        let mut na = 0;
        let mut nb = 100;

        while na < 100 {
            let claw = START + self.a * na + self.b * nb;

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
}

pub fn part1(input: String) -> u64 {
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
        .into_iter()
        .filter_map(|g| g.solve())
        .map(|(a, b)| (a * 3 + b) as u64)
        .sum()
}

pub fn part2(_input: String) -> u64 {
    0
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
    0,
);

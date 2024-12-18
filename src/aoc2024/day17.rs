use crate::util::{aoc_test, CollectVec};
use regex::Regex;

#[derive(Debug)]
struct State {
    a: usize,
    b: usize,
    c: usize,
    pc: usize,
    printed: bool,
}
impl State {
    fn step(&mut self, code: &[usize]) -> bool {
        if self.pc >= code.len() {
            return false;
        }

        let op = code[self.pc];
        let arg = code[self.pc + 1];
        self.pc += 2;

        match op {
            //div
            0 => self.a = self.a >> self.get(arg),
            6 => self.b = self.a >> self.get(arg),
            7 => self.c = self.a >> self.get(arg),
            // xors
            1 => self.b ^= arg,
            4 => self.b ^= self.c,
            // bst
            2 => self.b = self.get(arg) % 8,
            // jnz
            3 => {
                if self.a > 0 {
                    self.pc = arg;
                }
            }
            //out
            5 => {
                print!(
                    "{}{}",
                    if self.printed { "," } else { "" },
                    self.get(arg) % 8
                );
                self.printed = true;
            }

            _ => panic!("bad op"),
        }

        true
    }

    fn get(&self, arg: usize) -> usize {
        match arg {
            x if x <= 3 => x,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => {
                panic!("invalid arg")
            }
        }
    }
}

pub fn part1(input: String) -> u64 {
    let re = Regex::new(
        r"Register A: (\d+)
Register B: (\d+)
Register C: (\d+)

Program: ([\d,]+)",
    )
    .unwrap();

    let (_, [a, b, c, code]) = re.captures(&input).unwrap().extract();

    let mut state = State {
        a: a.parse().unwrap(),
        b: b.parse().unwrap(),
        c: c.parse().unwrap(),
        pc: 0,
        printed: false,
    };

    let code = code
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect_vec();

    loop {
        if !state.step(&code) {
            println!();
            return 0;
        }
    }
}

pub fn part2(_input: String) -> u64 {
    0
}

aoc_test!(
    "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
    0,
    0,
);

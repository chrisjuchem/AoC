use crate::util::{aoc_test, CollectVec};
use regex::Regex;
use std::time::Duration;

#[derive(Debug, Clone)]
struct State {
    a: usize,
    b: usize,
    c: usize,
    pc: usize,
    printed: bool,
}
impl State {
    fn with_a(mut self, a: usize) -> Self {
        self.a = a;
        self
    }

    fn run(&mut self, code: &[usize]) -> Vec<usize> {
        let mut out = vec![];
        while self.pc < code.len() {
            if let Some(n) = self.step(code) {
                out.push(n)
            }
        }
        out
    }

    fn step(&mut self, code: &[usize]) -> Option<usize> {
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
                return Some(self.get(arg) % 8);
            }

            _ => panic!("bad op"),
        }

        return None;
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

    println!(
        "{}",
        state
            .run(&code)
            .iter()
            .map(usize::to_string)
            .collect_vec()
            .join(",")
    );
    return 0;
}

pub fn part2(input: String) -> u64 {
    // 2,4,1,6,7,5,4,4,1,7,0,3,5,5,3,0
    //  0. 2,4  a mod 8 -> b
    //  2. 1,6  b xor 110 -> b          3. 6,7
    //  4. 7,5  a >> b -> c             5. 5,4
    //  6. 4,4  b xor c -> b            7. 4,1
    //  8. 1,7  b xor 111 -> b          9. 7,0
    // 10. 0,3  a >> 011 -> a          11. 3,5
    // 12. 5,5  print bmod 8           13. 5,3
    // 14. 3,0  janz 0

    // k 0-7
    // n 0+

    // 14  A=0
    // 12  A=0     B=8n+0
    // 10  A=k     B=8n+0
    //  8  A=k     B=8n+7
    //  6  A=k     B=8n+7
    //  4  A=k     B=7       C=0
    //  2  A=1     B=7
    //  0  A=1     B=1
    //k=0 B=110 C=0 B=110 x
    //k=1   111   0   111
    //k=2   100   0   x
    //k=3   101   0   x
    //k=4   010   0   x
    //k=5   011   0   x
    //k=6   000   6   x
    //k=7   001   3   x

    // 14  A=1     B=?
    // 12  A=1k
    // 10
    //  8
    //  6
    //  4
    //  2
    //  0

    let re = Regex::new(
        r"Register A: (\d+)
Register B: (\d+)
Register C: (\d+)

Program: ([\d,]+)",
    )
    .unwrap();

    let (_, [a, b, c, codestr]) = re.captures(&input).unwrap().extract();

    let mut initstate = State {
        a: a.parse().unwrap(),
        b: b.parse().unwrap(),
        c: c.parse().unwrap(),
        pc: 0,
        printed: false,
    };

    let code = codestr
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect_vec();

    recurse(&initstate, &code, 0, code.len() - 1).unwrap() as u64
}

fn recurse(initstate: &State, code: &Vec<usize>, a: usize, i: usize) -> Option<usize> {
    let n = code.len();

    for o in 0..8 {
        let testval = a + (o << 3 * i);

        let mut results = initstate.clone().with_a(testval).run(&code);
        while results.len() < n {
            results.insert(0, 9);
        }

        let mut matches = true;
        for x in i..n {
            if results[x] != code[x] {
                matches = false;
            }
        }

        if matches {
            if i == 0 {
                return Some(testval);
            }
            if let Some(ans) = recurse(initstate, code, testval, i - 1) {
                return Some(ans);
            }
        }
    }

    None
}

aoc_test!(
    "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
    0,
    "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0",
    117440,
);

use crate::aoc2019::intcode::Program;
use crate::util::aoc_test;

pub fn part1(input: String) -> i64 {
    let mut p: Program = input.parse().unwrap();

    #[cfg(not(test))]
    {
        p.set(1, 12).unwrap();
        p.set(2, 2).unwrap();
    }

    p.run_0().unwrap()
}

pub fn part2(input: String) -> i64 {
    let p: Program = input.parse().unwrap();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut p2 = p.clone();
            p2.set(1, noun).unwrap();
            p2.set(2, verb).unwrap();

            if p2.run_0() == Ok(19690720) {
                return noun * 100 + verb;
            }
        }
    }

    return 987654321;
}

aoc_test!("1,9,10,3,2,3,11,0,99,30,40,50", 3500, 987654321,);

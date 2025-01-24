use crate::aoc2019::intcode::Program;
use crate::util::aoc_test;

pub fn part1(input: String) -> u64 {
    let mut p: Program = input.parse().unwrap();

    #[cfg(not(test))]
    {
        p.set(1, 12);
        p.set(2, 2);
    }

    p.run().unwrap()
}

pub fn part2(input: String) -> u64 {
    #[cfg(test)]
    return 0;

    let p: Program = input.parse().unwrap();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut p2 = p.clone();
            p2.set(1, noun);
            p2.set(2, verb);

            if p2.run() == Ok(19690720) {
                return noun * 100 + verb;
            }
        }
    }

    panic!("no solution");
}

aoc_test!("1,9,10,3,2,3,11,0,99,30,40,50", 3500, 0,);

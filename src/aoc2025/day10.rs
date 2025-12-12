use good_lp::{Expression, Solution, SolverModel, variable, variables};
use std::{collections::HashSet, str::FromStr};

use crate::util::{CollectVec, aoc_test};

#[derive(PartialEq, Eq, Hash, Clone)]
struct State {
    ons: Vec<bool>,
}
impl State {
    fn after_press(&self, btn: &Button) -> Self {
        let mut ret = self.clone();
        for i in btn.idxs.iter().copied() {
            ret.ons[i] = !ret.ons[i];
        }
        ret
    }
}
impl FromStr for State {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret = State { ons: vec![] };
        for c in s.chars() {
            match c {
                '[' => {}
                '.' => ret.ons.push(false),
                '#' => ret.ons.push(true),
                ']' => return Ok(ret),
                _ => break,
            }
        }
        Err(())
    }
}

struct Button {
    idxs: Vec<usize>,
}
impl FromStr for Button {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret = Button { idxs: vec![] };

        if s.chars().next().unwrap() != '(' {
            return Err(());
        }

        for n in s[1..s.len() - 1].split(",") {
            ret.idxs.push(n.parse().unwrap())
        }

        Ok(ret)
    }
}
#[derive(PartialEq, Eq, Hash, Clone)]
struct Joltage {
    idxs: Vec<usize>,
}
impl FromStr for Joltage {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret = Joltage { idxs: vec![] };

        if s.chars().next().unwrap() != '{' {
            return Err(());
        }

        for n in s[1..s.len() - 1].split(",") {
            ret.idxs.push(n.parse().unwrap())
        }

        Ok(ret)
    }
}

fn parse_line(line: &str) -> (State, Vec<Button>, Joltage) {
    let mut sections = line.split(" ");
    let goal = sections.next().unwrap().parse().unwrap();
    let mut buttons = vec![];
    let joltage = loop {
        let s = sections.next().unwrap();
        if s.starts_with("{") {
            break s.parse().unwrap();
        }

        buttons.push(s.parse().unwrap());
    };

    (goal, buttons, joltage)
}

pub fn part1(input: String) -> u64 {
    let mut total = 0;
    'line: for line in input.lines() {
        let (goal, buttons, _) = parse_line(line);

        let mut possible_states = Vec::from([State {
            ons: vec![false; goal.ons.len()],
        }]);
        let mut all_seen_states: HashSet<State> =
            HashSet::from_iter(possible_states.iter().cloned());

        for i in 1..=10 {
            let mut new_possibilities = Vec::new();

            for start in possible_states {
                for btn in buttons.iter() {
                    let next = start.after_press(btn);

                    if next == goal {
                        total += i;
                        continue 'line;
                    }

                    if !all_seen_states.contains(&next) {
                        all_seen_states.insert(next.clone());
                        new_possibilities.push(next)
                    }
                }
            }

            possible_states = new_possibilities;
        }
        panic!("didnt find")
    }

    total
}

pub fn part2(input: String) -> u64 {
    let mut total = 0;
    for line in input.lines() {
        let (_, buttons, goal) = parse_line(line);

        // (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        //     presses   *    buttons   =   joltages
        //     1 x 6           6 x 4         1 x 4
        // [A B C D E F]  x  [0 0 0 1]  =  [3 5 4 7]
        //                   [0 1 0 1]
        //                   [0 0 1 0]
        //                   [0 0 1 1]
        //                   [1 0 1 0]
        //                   [1 1 0 0]
        //
        // P = J * B^-1

        // let b = DMatrix::from_fn(buttons.len(), goal.idxs.len(), |r, c| {
        //     if buttons[r].idxs.contains(&c) { 1. } else { 0. }
        // });
        // let j = DMatrix::from_iterator(1, goal.idxs.len(), goal.idxs.iter().map(|i| *i as f64));

        // let b_inv = b.clone().pseudo_inverse(f64::EPSILON).unwrap();
        // println!("{j} * {b_inv}");

        // println!(
        //     "linv {}  rinv{}",
        //     b_inv.clone() * b.clone(),
        //     b * b_inv.clone()
        // );
        // let p = j * b_inv;
        // println!("= {p}");

        // total += p.iter().sum::<f64>() as u64;

        let mut problem_vars = variables!();
        let vars = buttons
            .iter()
            .map(|_| problem_vars.add(variable().integer().min(0)))
            .collect_vec();

        let objective = vars.iter().fold(Expression::from(0.), |e, v| e + v);

        let mut problem = problem_vars
            .minimise(objective.clone())
            .using(good_lp::solvers::microlp::microlp);
        for (n, j) in goal.idxs.iter().copied().enumerate() {
            let mut expr = Expression::from(0.);

            for (i, v) in vars.iter().enumerate() {
                if buttons[i].idxs.contains(&n) {
                    expr += v;
                }
            }

            problem = problem.with(expr.eq(j as f64))
        }

        let solution = problem.solve().unwrap();
        let presses = solution.eval(objective);
        total += presses as u64;
    }

    total
}

aoc_test!(
    "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
    7,
    33,
);

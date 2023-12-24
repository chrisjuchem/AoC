use crate::util::{aoc_test, SplitInto};
use regex::Regex;
use std::collections::HashMap;

#[derive(Copy, Clone)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}
impl Part {
    fn get(&self, field: &str) -> u64 {
        match field {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => panic!("no such field"),
        }
    }
}

struct Rule<'a> {
    cond: Cond<'a>,
    next_flow: &'a str,
}
impl<'a> From<&'a str> for Rule<'a> {
    fn from(value: &'a str) -> Self {
        if value.contains(":") {
            let (cond, next_flow) = value.split_into(":");
            Rule {
                cond: cond.into(),
                next_flow,
            }
        } else {
            Rule {
                cond: Cond::Always,
                next_flow: value,
            }
        }
    }
}

enum Cond<'a> {
    Always,
    LessThan(&'a str, u64),
    GreaterThan(&'a str, u64),
}
impl<'a> From<&'a str> for Cond<'a> {
    fn from(value: &'a str) -> Self {
        if value.contains(">") {
            let (field, num) = value.split_into(">");
            Cond::GreaterThan(field, num.parse().unwrap())
        } else if value.contains("<") {
            let (field, num) = value.split_into("<");
            Cond::LessThan(field, num.parse().unwrap())
        } else {
            panic!("bad cond")
        }
    }
}
impl<'a> Cond<'a> {
    fn matches(&self, part: Part) -> bool {
        match self {
            Cond::Always => true,
            Cond::LessThan(f, n) => part.get(f) < *n,
            Cond::GreaterThan(f, n) => part.get(f) > *n,
        }
    }
}

fn process<'a>(part: Part, rules: &[Rule<'a>]) -> &'a str {
    for r in rules {
        if r.cond.matches(part) {
            return r.next_flow;
        }
    }
    panic!("no rule matched")
}

pub fn part1(_input: String) -> u64 {
    let (rules, parts) = _input.split_into("\n\n");

    let re = Regex::new(r"x=(\d+),m=(\d+),a=(\d+),s=(\d+)").unwrap();
    let parts = parts.lines().map(move |line| {
        let [x, m, a, s] = re.captures(line).unwrap().extract::<4>().1;
        Part {
            x: x.parse().unwrap(),
            m: m.parse().unwrap(),
            a: a.parse().unwrap(),
            s: s.parse().unwrap(),
        }
    });

    let flows: HashMap<&str, Vec<Rule>> = rules
        .lines()
        .map(|line| {
            let (name, rules) = line.split("}").next().unwrap().split_into("{");
            (name, rules.split(",").map(Rule::from).collect())
        })
        .collect();

    parts
        .filter(|p| {
            let mut flow = "in";
            loop {
                let f = flows.get(flow).unwrap();
                match process(*p, f) {
                    "A" => return true,
                    "R" => return false,
                    other => flow = other,
                }
            }
        })
        .map(|p| p.x + p.m + p.a + p.s)
        .sum()
}

pub fn part2(_input: String) -> u64 {
    0
}

aoc_test!(
    "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
",
    19114,
    0,
    // 167409079868000,
);

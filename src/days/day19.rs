use crate::util::{aoc_test, SplitInto};
use regex::Regex;
use std::collections::HashMap;
use std::ops::RangeInclusive;

#[derive(Copy, Clone)]
struct Part<T> {
    x: T,
    m: T,
    a: T,
    s: T,
}
impl<T> Part<T> {
    fn get(&self, field: &str) -> &T {
        match field {
            "x" => &self.x,
            "m" => &self.m,
            "a" => &self.a,
            "s" => &self.s,
            _ => panic!("no such field"),
        }
    }

    fn set(&mut self, field: &str, val: T) {
        match field {
            "x" => self.x = val,
            "m" => self.m = val,
            "a" => self.a = val,
            "s" => self.s = val,
            _ => panic!("no such field"),
        }
    }
}

impl Part<RangeInclusive<u64>> {
    fn count(&self) -> u64 {
        (self.x.end() - self.x.start() + 1)
            * (self.m.end() - self.m.start() + 1)
            * (self.a.end() - self.a.start() + 1)
            * (self.s.end() - self.s.start() + 1)
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
    fn matches(&self, part: Part<u64>) -> bool {
        match self {
            Cond::Always => true,
            Cond::LessThan(f, n) => part.get(f) < n,
            Cond::GreaterThan(f, n) => part.get(f) > n,
        }
    }

    fn take_matching(
        &self,
        parts_opt: &mut Option<Part<RangeInclusive<u64>>>,
    ) -> Option<Part<RangeInclusive<u64>>> {
        let Some(parts) = parts_opt else { return None };

        match self {
            Cond::Always => parts_opt.take(),
            Cond::LessThan(f, n) => {
                let range = parts.get(f);
                let (start, end) = (range.start(), range.end());

                if start >= n {
                    None
                } else if end < n {
                    parts_opt.take()
                } else {
                    let mut matching = parts.clone();
                    matching.set(f, *start..=*n - 1);
                    parts.set(f, *n..=*end);
                    Some(matching)
                }
            }
            Cond::GreaterThan(f, n) => {
                let range = parts.get(f);
                let (start, end) = (range.start(), range.end());

                if end <= n {
                    None
                } else if start > n {
                    parts_opt.take()
                } else {
                    let mut matching = parts.clone();
                    matching.set(f, *n + 1..=*end);
                    parts.set(f, *start..=*n);
                    Some(matching)
                }
            }
        }
    }
}

fn process<'a>(part: Part<u64>, rules: &[Rule<'a>]) -> &'a str {
    for r in rules {
        if r.cond.matches(part) {
            return r.next_flow;
        }
    }
    panic!("no rule matched")
}
fn process_range<'a>(
    parts: Part<RangeInclusive<u64>>,
    rules: &'a [Rule<'a>],
) -> impl Iterator<Item = (Part<RangeInclusive<u64>>, &'a str)> + 'a {
    let mut parts = Some(parts);
    rules.iter().filter_map(move |rule| {
        rule.cond
            .take_matching(&mut parts)
            .map(|ps| (ps, rule.next_flow))
    })
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
    let flows: HashMap<&str, Vec<Rule>> = _input
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let (name, rules) = line.split("}").next().unwrap().split_into("{");
            (name, rules.split(",").map(Rule::from).collect())
        })
        .collect();

    let mut parts_list = vec![(
        Part {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        },
        "in",
    )];
    let mut accepted = 0;
    while !parts_list.is_empty() {
        let accepted_ref = &mut accepted;
        parts_list = parts_list
            .into_iter()
            .flat_map(|(part_range, flow)| process_range(part_range, &flows.get(flow).unwrap()))
            .filter(move |(part_range, new_flow)| match *new_flow {
                "A" => {
                    *accepted_ref += part_range.count();
                    false
                }
                "R" => false,
                _ => true,
            })
            .collect();
    }
    accepted
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
    167409079868000,
);

use crate::util::{aoc_test, CollectVec, SplitInto};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub fn part1(input: String) -> u64 {
    let (rules, lists) = input.split_into("\n\n");
    let mut before_constraints: HashMap<u64, HashSet<u64>> = HashMap::new(); // values must come after keys
    for rule in rules.lines() {
        let (before, after) = rule.split_into("|");
        let before = before.parse().unwrap();
        let after = after.parse().unwrap();

        before_constraints.entry(before).or_default().insert(after);
    }

    let mut total = 0;
    'list: for list in lists.lines() {
        let mut seen = HashSet::new();
        let pages = list.split(",").map(|p| p.parse().unwrap()).collect_vec();
        for page in &pages {
            let relevant_constraints = before_constraints.get(&page).cloned().unwrap_or_default();
            let broken_constraints = relevant_constraints.intersection(&seen).collect_vec();
            if !broken_constraints.is_empty() {
                continue 'list;
            }
            seen.insert(*page);
        }
        total += pages[pages.len() / 2];
    }

    total as u64
}

pub fn part2(input: String) -> u64 {
    let (rules, lists) = input.split_into("\n\n");
    let mut before_constraints: HashMap<u64, HashSet<u64>> = HashMap::new(); // values must come after keys
    for rule in rules.lines() {
        let (before, after) = rule.split_into("|");
        let before = before.parse().unwrap();
        let after = after.parse().unwrap();
        // let c = Constraint {
        //     before, after,
        // };
        before_constraints.entry(before).or_default().insert(after);
    }

    let mut total = 0;
    for list in lists.lines() {
        let mut pages: Vec<_> = list.split(",").map(|p| p.parse().unwrap()).collect_vec();

        if pages.is_sorted_by(|a, b| {
            before_constraints
                .get(a)
                .map(|constraints| constraints.get(b).is_some())
                .unwrap_or(false)
        }) {
            continue;
        }
        pages.sort_by(|a, b| {
            if before_constraints
                .get(a)
                .map(|constraints| constraints.get(b).is_some())
                .unwrap_or(false)
            {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        total += pages[pages.len() / 2];
    }

    total as u64
}

aoc_test!(
    "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
    143,
    123,
);

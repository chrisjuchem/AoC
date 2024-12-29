use crate::util::{SplitInto, aoc_test};
use std::collections::{BTreeSet, HashMap};

pub fn part1(input: String) -> usize {
    let mut connections: HashMap<_, BTreeSet<_>> = HashMap::new();

    for line in input.lines() {
        let (mut a, mut b) = line.split_into("-");
        if a > b {
            (a, b) = (b, a);
        }

        connections.entry(a).or_default().insert(b);
    }

    let mut trios = vec![];

    for (a, a_conns) in &connections {
        for b in a_conns {
            let Some(b_conns) = connections.get(b) else {
                continue;
            };

            for c in b_conns {
                if connections[a].contains(c) {
                    trios.push((a, b, c));
                }
            }
        }
    }

    trios
        .iter()
        .filter(|(a, b, c)| a.starts_with("t") || b.starts_with("t") || c.starts_with("t"))
        .count()
}

fn can_join(conns: &HashMap<&str, BTreeSet<&str>>, party: &[&str], candidate: &str) -> bool {
    for c in party {
        let Some(c_conns) = conns.get(c) else {
            return false;
        };

        if !c_conns.contains(candidate) {
            return false;
        }
    }
    true
}

pub fn recurse<'a>(party: Vec<&'a str>, conns: &HashMap<&str, BTreeSet<&'a str>>) -> Vec<&'a str> {
    let Some(candidates) = conns.get(party[party.len() - 1]) else {
        return party;
    };

    let mut best = party.clone();
    for c in candidates {
        if can_join(conns, &party, c) {
            let mut new_party = party.clone();
            new_party.push(c);
            let r = recurse(new_party, conns);

            if r.len() > best.len() {
                best = r;
            }
        }
    }

    best
}

pub fn part2(input: String) -> String {
    let mut connections: HashMap<&str, BTreeSet<_>> = HashMap::new();

    for line in input.lines() {
        let (mut a, mut b) = line.split_into("-");
        if a > b {
            (a, b) = (b, a);
        }

        connections.entry(a).or_default().insert(b);
    }

    connections
        .keys()
        .map(|comp| recurse(vec![*comp], &connections))
        .max_by_key(|party| party.len())
        .unwrap()
        .join(",")
}

aoc_test!(
    "kh-tc
qp-kh
de-cg
ka-co
yn-at
qp-ub
cg-tb
vc-at
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
at-cg
wq-ub
ub-vc
de-ta
wq-at
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
",
    7,
    "co,de,ka,ta",
);

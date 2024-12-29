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

pub fn part2(_input: String) -> u64 {
    0
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
    0,
);

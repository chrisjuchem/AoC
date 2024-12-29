use crate::grid::Loc;
use crate::util::CountMap;
use crate::util::aoc_test;
use std::collections::HashMap;

fn num_map() -> HashMap<char, Loc> {
    [
        ('A', Loc::new(0, 0)),
        ('0', Loc::new(0, -1)),
        ('1', Loc::new(1, -2)),
        ('2', Loc::new(1, -1)),
        ('3', Loc::new(1, 0)),
        ('4', Loc::new(2, -2)),
        ('5', Loc::new(2, -1)),
        ('6', Loc::new(2, 0)),
        ('7', Loc::new(3, -2)),
        ('8', Loc::new(3, -1)),
        ('9', Loc::new(3, 0)),
    ]
    .into()
}
fn dir_map() -> HashMap<char, Loc> {
    [
        ('A', Loc::new(0, 0)),
        ('^', Loc::new(0, -1)),
        ('<', Loc::new(-1, -2)),
        ('v', Loc::new(-1, -1)),
        ('>', Loc::new(-1, 0)),
    ]
    .into()
}

fn get_seq(map: HashMap<char, Loc>, targets: HashMap<String, u64>) -> HashMap<String, u64> {
    let mut seqs = HashMap::new();

    let illegal = Loc::new(0, -2);

    for (target, n) in targets {
        let mut cur_pos = Loc::new(0, 0);
        assert!(target.ends_with("A"));

        for c in target.chars() {
            let delta = map[&c] - cur_pos;

            let mut seq = String::new();
            let col_first = if cur_pos.r == illegal.r && cur_pos.c + delta.dc == illegal.c {
                false
            } else if cur_pos.r + delta.dr == illegal.r && cur_pos.c == illegal.c {
                true
            } else {
                delta.dc < 0
            };

            if col_first {
                for _ in 0..(delta.dc.abs()) {
                    seq.push(if delta.dc < 0 { '<' } else { '>' });
                }
            }
            for _ in 0..(delta.dr.abs()) {
                seq.push(if delta.dr < 0 { 'v' } else { '^' });
            }
            if !col_first {
                for _ in 0..(delta.dc.abs()) {
                    seq.push(if delta.dc < 0 { '<' } else { '>' });
                }
            }
            seq.push('A');

            seqs.insert_n(seq, n);

            cur_pos = cur_pos + delta;
        }
    }
    seqs
}

pub fn solve(input: String, d: usize) -> usize {
    input
        .lines()
        .map(|line| {
            let mut s = get_seq(num_map(), HashMap::from([(line.to_string(), 1)]));

            for _ in 0..d {
                s = get_seq(dir_map(), s);
            }

            let len = s
                .into_iter()
                .map(|(k, v)| k.len() * (v as usize))
                .sum::<usize>();

            line[0..line.len() - 1].parse::<usize>().unwrap() * len
        })
        .sum()
}

pub fn part1(input: String) -> usize {
    solve(input, 2)
}

pub fn part2(input: String) -> usize {
    solve(input, 25)
}

aoc_test!(
    "029A
980A
179A
456A
379A",
    126384,
    154115708116294,
);

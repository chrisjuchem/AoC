use crate::grid::Loc;
use crate::util::aoc_test;
use std::collections::{HashMap, HashSet};

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

fn get_seqs(map: HashMap<char, Loc>, target: String) -> HashSet<String> {
    let mut candidates = HashSet::from([String::new()]);

    let illegal = Loc::new(0, -2);

    let mut cur_pos = Loc::new(0, 0);
    for c in target.chars() {
        let delta = map[&c] - cur_pos;

        let mut paths = vec![];

        if cur_pos.r != illegal.r || cur_pos.c + delta.dc != illegal.c {
            let mut path = String::new();
            for _ in 0..(delta.dc.abs()) {
                path.push(if delta.dc < 0 { '<' } else { '>' });
            }
            for _ in 0..(delta.dr.abs()) {
                path.push(if delta.dr < 0 { 'v' } else { '^' });
            }
            path.push('A');
            paths.push(path);
        }
        if cur_pos.r + delta.dr != illegal.r || cur_pos.c != illegal.c {
            let mut path = String::new();
            for _ in 0..(delta.dr.abs()) {
                path.push(if delta.dr < 0 { 'v' } else { '^' });
            }
            for _ in 0..(delta.dc.abs()) {
                path.push(if delta.dc < 0 { '<' } else { '>' });
            }
            path.push('A');
            paths.push(path);
        }

        let mut new_candidates = HashSet::new();
        for cand in candidates {
            for path in &paths {
                new_candidates.insert(cand.clone() + path.as_str());
            }
        }
        candidates = new_candidates;
        // println!("{paths:?}");
        // println!("{candidates:?}");

        cur_pos = cur_pos + delta;
    }
    candidates
}

pub fn part1(input: String) -> usize {
    input
        .lines()
        .map(|line| {
            // println!("{line}");
            let s1 = get_seqs(num_map(), line.to_string());
            // println!("{}", s1.len());
            let s2 = s1
                .into_iter()
                .flat_map(|s| get_seqs(dir_map(), s))
                .collect::<HashSet<String>>();
            // println!("{}", s2.len());
            let s3 = s2
                .into_iter()
                .flat_map(|s| get_seqs(dir_map(), s))
                .collect::<HashSet<String>>();
            // println!("{}", s3.len());

            let best = s3.into_iter().min_by_key(|s| s.len()).unwrap();

            line[0..line.len() - 1].parse::<usize>().unwrap() * best.len()
        })
        .sum()
}
/*
379A
^A^^<<A>>AvvvA
<A>A<AAv<AA^>>AvAA^Av<AAA^>A
v<<A^>>AvA^Av<<A^>>AAv<A<A^>>AA<Av>AA^Av<A^>AA<A>Av<A<A^>>AAA<Av>A^A
           |                      .   '
<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
   <   A > A  v <<   AA >  ^ AA > A  v  AA ^ A   < v  AAA >  ^ A
<A>Av<<AA>^AA>AvAA^A<vAAA>^A
 ^ A   <<  ^^ A >> A  vvv  A


^^        <<         A
<AA       v<AA       ^>>A
v<<A^>>AA v<A<A^>>AA <Av>AA^A
                        .   '
<vA<AA>>^AA vA<^A>AA vA^A
v<<AA       >^AA     >A
<<          ^^       A
*/
pub fn part2(_input: String) -> u64 {
    0
}

aoc_test!(
    "029A
980A
179A
456A
379A",
    126384,
    0,
);

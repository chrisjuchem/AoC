use crate::grid::Loc;
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

fn get_seq(map: HashMap<char, Loc>, target: String) -> String {
    let mut seq = String::new();
    let mut cur_pos = Loc::new(0, 0);
    for c in target.chars() {
        let delta = map[&c] - cur_pos;

        // moving horizontal first is more efficient because it guarantees all needed < presses are
        // bunched together, and getting to < is most difficult.
        // need to check that we can't move off keypad though
        if cur_pos.r != 0 {
            for _ in 0..(delta.dc.abs()) {
                seq.push(if delta.dc < 0 { '<' } else { '>' });
            }
            for _ in 0..(delta.dr.abs()) {
                seq.push(if delta.dr < 0 { 'v' } else { '^' });
            }
        } else {
            for _ in 0..(delta.dr.abs()) {
                seq.push(if delta.dr < 0 { 'v' } else { '^' });
            }
            for _ in 0..(delta.dc.abs()) {
                seq.push(if delta.dc < 0 { '<' } else { '>' });
            }
        }
        seq.push('A');

        cur_pos = cur_pos + delta;
    }
    seq
}

pub fn part1(input: String) -> usize {
    input
        .lines()
        .map(|line| {
            println!("{line}");
            let s1 = get_seq(num_map(), line.to_string());
            println!("{s1}");
            let s2 = get_seq(dir_map(), s1);
            println!("{s2}");
            let s3 = get_seq(dir_map(), s2);
            println!("{s3}");

            line[0..line.len() - 1].parse::<usize>().unwrap() * s3.len()
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

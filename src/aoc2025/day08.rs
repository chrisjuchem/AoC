use std::{
    collections::{HashMap, HashSet},
    usize,
};

use crate::util::{SplitInto, aoc_test};

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Pt {
    x: u64,
    y: u64,
    z: u64,
}
impl Pt {
    fn dist(&self, other: &Pt) -> f64 {
        let x1 = self.x as f64;
        let y1 = self.y as f64;
        let z1 = self.z as f64;
        let x2 = other.x as f64;
        let y2 = other.y as f64;
        let z2 = other.z as f64;

        let dx = x1 - x2;
        let dy = y1 - y2;
        let dz = z1 - z2;

        f64::sqrt(dx * dx + dy * dy + dz * dz)
    }
}
#[derive(PartialEq, Clone, Debug)]
struct Edge {
    p1: Pt,
    p2: Pt,
    dist: f64,
}

fn logic(input: String, limit: Option<usize>) -> (Edge, HashMap<Pt, HashSet<Pt>>) {
    let mut pts = Vec::new();
    let mut edges = Vec::new();

    for l in input.lines() {
        let (x, y, z) = l.split_into(",");
        pts.push(Pt {
            x: x.parse::<u64>().unwrap(),
            y: y.parse::<u64>().unwrap(),
            z: z.parse::<u64>().unwrap(),
        });
    }
    let mut reps = HashMap::new();
    let mut groups = HashMap::new();
    for p in pts.iter() {
        reps.insert(*p, *p);
        groups.insert(*p, HashSet::from([*p]));
    }

    for i1 in 0..pts.len() {
        for i2 in i1 + 1..pts.len() {
            let p1 = pts[i1];
            let p2 = pts[i2];
            let dist = p1.dist(&p2);
            edges.push(Edge { p1, p2, dist });
        }
    }
    edges.sort_unstable_by(|e1, e2| e1.dist.partial_cmp(&e2.dist).unwrap());

    let mut e_iter = edges.into_iter().take(limit.unwrap_or(usize::MAX));

    let mut e = e_iter.next().unwrap();
    loop {
        let r1 = reps[&e.p1];
        let r2 = reps[&e.p2];

        if r1 != r2 {
            let group_to_move = groups.remove(&r2).unwrap();
            let main_group = groups.get_mut(&r1).unwrap();
            for p in group_to_move {
                *reps.get_mut(&p).unwrap() = r1;
                main_group.insert(p);
            }
        }

        if groups.len() == 1 {
            break;
        }
        match e_iter.next() {
            None => break,
            Some(e_) => e = e_,
        }
    }

    return (e, groups);
}

#[cfg(test)]
const N: usize = 10;
#[cfg(not(test))]
const N: usize = 1000;

pub fn part1(input: String) -> u64 {
    let (_, groups) = logic(input, Some(N));

    let mut sizes = groups.values().map(|g| g.len() as u64).collect::<Vec<_>>();
    sizes.sort();
    sizes.into_iter().rev().take(3).product()
}

pub fn part2(input: String) -> u64 {
    let (e, _) = logic(input, None);
    e.p1.x * e.p2.x
}

aoc_test!(
    "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
",
    40,
    25272,
);

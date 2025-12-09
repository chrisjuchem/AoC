use crate::{
    grid::Loc,
    util::{CollectVec, SplitInto, aoc_test},
};

pub fn part1(input: String) -> u64 {
    let pts = input
        .lines()
        .map(|l| {
            let (x, y) = l.split_into(",");
            Loc {
                r: y.parse().unwrap(),
                c: x.parse().unwrap(),
            }
        })
        .collect_vec();

    let mut best = 0;
    for i in 0..pts.len() {
        for j in i + 1..pts.len() {
            let dr = pts[i].r.abs_diff(pts[j].r) + 1;
            let dc = pts[i].c.abs_diff(pts[j].c) + 1;
            let area = dr * dc;
            best = best.max(area);
        }
    }
    best
}

struct Edge {
    along: i64,
    min: i64,
    max: i64,
    vertical: bool,
}

pub fn part2(input: String) -> u64 {
    let pts = input
        .lines()
        .map(|l| {
            let (x, y) = l.split_into(",");
            Loc {
                r: y.parse().unwrap(),
                c: x.parse().unwrap(),
            }
        })
        .collect_vec();
    let edges = pts
        .iter()
        .enumerate()
        .map(|(i, p1)| {
            let p2 = pts[(i + 1) % pts.len()];
            if p2.c == p1.c {
                Edge {
                    along: p1.c,
                    min: p1.r.min(p2.r),
                    max: p1.r.max(p2.r),
                    vertical: true,
                }
            } else if p2.r == p1.r {
                Edge {
                    along: p1.r,
                    min: p1.c.min(p2.c),
                    max: p1.c.max(p2.c),
                    vertical: false,
                }
            } else {
                panic!("not aligned");
            }
        })
        .collect_vec();

    let mut best = 0;
    for i in 0..pts.len() {
        'pair: for j in i + 1..pts.len() {
            let p1 = pts[i];
            let p2 = pts[j];

            let rmin = p1.r.min(p2.r);
            let rmax = p1.r.max(p2.r);
            let cmin = p1.c.min(p2.c);
            let cmax = p1.c.max(p2.c);

            for e in edges.iter() {
                if e.vertical {
                    if cmin < e.along && e.along < cmax && !(e.max <= rmin || e.min >= rmax) {
                        continue 'pair;
                    }
                } else {
                    if rmin < e.along && e.along < rmax && !(e.max <= cmin || e.min >= cmax) {
                        continue 'pair;
                    }
                }
            }

            let dr = pts[i].r.abs_diff(pts[j].r) + 1;
            let dc = pts[i].c.abs_diff(pts[j].c) + 1;
            let area = dr * dc;
            best = best.max(area);
        }
    }
    best
}

aoc_test!(
    "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
    50,
    24,
);

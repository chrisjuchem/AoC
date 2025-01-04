use crate::util::{CollectVec, SplitInto, aoc_test, test_val};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq)]
struct Hailstone {
    x: f64,
    y: f64,
    z: f64,
    dx: f64,
    dy: f64,
    dz: f64,
}
impl Display for Hailstone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self {
            x,
            y,
            z,
            dx,
            dy,
            dz,
        } = *self;
        write!(f, "{x}, {y}, {z} @ {dx}, {dy}, {dz}")
    }
}

fn hailstones(input: String) -> Vec<Hailstone> {
    input
        .lines()
        .map(|line| {
            let (pos, delta) = line.split_into(" @ ");
            let (x, y, z) = pos.split_into(",");
            let (dx, dy, dz) = delta.split_into(",");
            let x: f64 = x.trim().parse().unwrap();
            let y: f64 = y.trim().parse().unwrap();
            let z: f64 = z.trim().parse().unwrap();
            let dx: f64 = dx.trim().parse().unwrap();
            let dy: f64 = dy.trim().parse().unwrap();
            let dz: f64 = dz.trim().parse().unwrap();

            Hailstone {
                x,
                y,
                z,
                dx,
                dy,
                dz,
            }
        })
        .collect_vec()
}

pub fn part1(input: String) -> u64 {
    let stones = hailstones(input);

    let mut crosses = 0;

    test_val!(test_area, 7.0..=27.0, 200000000000000.0..400000000000000.0);

    for i in 0..stones.len() {
        for j in (i + 1)..stones.len() {
            let a = stones[i];
            let b = stones[j];

            //         a.x + a.dx * ta  = b.x + b.dx * tb      * b.dy
            //         a.y + a.dy * ta  = b.y + b.dy * tb      * b.dx
            //
            // (b.dy * a.x) + b.dy * a.dx * ta)  = b.dy * b.x +  b.dx * b.dy * tb
            // (b.dx * a.y) + b.dx * a.dy * ta)  = b.dx * b.y +  b.dx * b.dy * tb
            //
            // (b.dy * a.x - b.dx * a.y) + (b.dy * a.dx - b.dx * a.dy) * ta = b.dy * b.x - b.dx * b.y

            let ta = ((b.dy * b.x - b.dx * b.y) - (b.dy * a.x - b.dx * a.y))
                / (b.dy * a.dx - b.dx * a.dy);
            let x = a.x + ta * a.dx;
            let y = a.y + ta * a.dy;
            let tb = (x - b.x) / b.dx;

            if test_area.contains(&x) && test_area.contains(&y) && ta > 0.0 && tb > 0.0 {
                crosses += 1;
            }
        }
    }

    crosses
}

pub fn part2(_input: String) -> u64 {
    0
}

aoc_test!(
    "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3",
    2,
    0,
);

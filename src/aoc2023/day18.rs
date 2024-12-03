use crate::util::{aoc_test, SplitInto};
use bevy_math::Vec2;
use num::Num;
use regex::Regex;
use std::f32::consts::PI;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum Dir {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}
impl From<&str> for Dir {
    fn from(value: &str) -> Self {
        match value {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => panic!("bad dir"),
        }
    }
}
impl Dir {
    fn is_convex_after(&self, other: Self) -> bool {
        match (other, self) {
            (Dir::Right, Dir::Down) => true,
            (Dir::Down, Dir::Left) => true,
            (Dir::Left, Dir::Up) => true,
            (Dir::Up, Dir::Right) => true,
            (Dir::Up, Dir::Left) => false,
            (Dir::Left, Dir::Down) => false,
            (Dir::Down, Dir::Right) => false,
            (Dir::Right, Dir::Up) => false,
            _ => panic!("not 90deg turn"),
        }
    }
}

// https://www.geometrictools.com/Documentation/TriangulationByEarClipping.pdf
// https://en.wikipedia.org/wiki/Polygon_triangulation

fn get_area(mut digs: impl Iterator<Item = (Dir, f32)>) -> u64 {
    let mut verts = vec![Vec2::new(0., 0.)];
    let (mut last_dir, mut last_dist) = digs.next().unwrap();
    let first_dir = last_dir;
    let mut last_convex = true;
    for (d, n) in digs {
        let (vert, convex) = new_coord(*verts.last().unwrap(), last_dir, d, last_dist, last_convex);
        verts.push(vert);
        last_dist = n;
        last_dir = d;
        last_convex = convex;
    }
    assert!(first_dir.is_convex_after(last_dir));

    println!("{verts:?}");
    clip_ears(verts)
}

fn new_coord(
    old_vert: Vec2,
    old_dir: Dir,
    new_dir: Dir,
    mut dist: f32,
    last_convex: bool,
) -> (Vec2, bool) {
    let convex = new_dir.is_convex_after(old_dir);

    if convex {
        dist = dist + 1.;
    }
    if !last_convex {
        dist = dist - 1.;
    }

    (
        Vec2::from(match old_dir {
            Dir::Right => (old_vert.x + dist, old_vert.y),
            Dir::Down => (old_vert.x, old_vert.y + dist),
            Dir::Left => (old_vert.x - dist, old_vert.y),
            Dir::Up => (old_vert.x, old_vert.y - dist),
        }),
        convex,
    )
}

fn clip_ears(mut verts: Vec<Vec2>) -> u64 {
    let mut area = 0;
    let mut i = 0;
    while verts.len() > 2 {
        if i >= verts.len() {
            i -= verts.len();
            continue;
        }

        let tip = i;
        let adj1 = if i == 0 { verts.len() - 1 } else { i - 1 };
        let adj2 = if i == verts.len() - 1 { 0 } else { i + 1 };
        let (tipv, adj1v, adj2v) = (verts[tip], verts[adj1], verts[adj2]);

        println!("checking triangle {}{}{}", tipv, adj1v, adj2v);

        if (tipv - adj1v).angle_between(adj2v - tipv) < 0. {
            // concave
            println!(
                "skipping concave triangle - {} triangles remain",
                verts.len()
            );
            i += 1;
            continue;
        }

        // 83, -55 B
        // 85, -53 A
        // 82, -52 C
        // 83, -52 D

        //     80     87
        //     v      v
        //     +--B---+ -55
        //     |      |
        //     |    A |
        //     | CD   |
        //     |      |
        //     +------+ -50

        if (0..verts.len())
            .filter(|i| *i != tip && *i != adj1 && *i != adj2)
            .all(|i| {
                let v = verts[i];
                let ang1 = tipv - v;
                let ang2 = adj1v - v;
                let ang3 = adj2v - v;
                // a point inside the triangle will equal a full 2pi
                let total_angle = (ang1.angle_between(ang2)
                    + ang2.angle_between(ang3)
                    + ang3.angle_between(ang1))
                .abs();
                let outside = total_angle < (PI * 2. - f32::EPSILON * 10.);
                if outside {
                    println!("{v} is outside - {total_angle}");
                } else {
                    println!("{v} is inside - {total_angle}");
                }
                outside
            })
        {
            let new_area = triangle_double_area(tipv, adj1v, adj2v);
            println!("adding area {}", new_area / 2);
            area += new_area;
            verts.remove(i);
        } else {
            i += 1;
            continue;
        }
    }
    area / 2
}

fn triangle_double_area(p0: Vec2, p1: Vec2, p2: Vec2) -> u64 {
    let p0x = p0.x as i64;
    let p0y = p0.y as i64;
    let p1x = p1.x as i64;
    let p1y = p1.y as i64;
    let p2x = p2.x as i64;
    let p2y = p2.y as i64;
    (p0x * (p1y - p2y) + p1x * (p2y - p0y) + p2x * (p0y - p1y)).abs() as u64
}

pub fn part1(_input: String) -> u64 {
    let digs = _input.lines().map(|row| {
        let (d, n, _) = row.split_into(" ");
        (Dir::from(d), n.parse().unwrap())
    });

    get_area(digs)
}

pub fn part2(_input: String) -> u64 {
    let re = Regex::new(r"\(#([0-9a-f]{5})([0-3])\)").unwrap();
    let digs = _input.lines().map(move |line| {
        let [dist, dir] = re.captures(line).unwrap().extract::<2>().1;
        // uses repr(u8) and in range b/c regex captured it.
        let dir: Dir = unsafe { std::mem::transmute(dir.parse::<u8>().unwrap()) };
        (dir, f32::from_str_radix(dist, 16).unwrap())
    });
    get_area(digs)
}

aoc_test!(
    "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
",
    62,
    952408144115,
);

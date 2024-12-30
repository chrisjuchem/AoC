use crate::grid::{Grid, Loc, parse_grid_with};
use crate::util::aoc_test;
use std::collections::{HashSet, VecDeque};
use std::iter::FromIterator;

#[derive(Copy, Clone)]
struct Cell {
    kind: char,
    seen: bool,
}
impl Cell {
    fn new(kind: char) -> Self {
        Self { kind, seen: false }
    }
}

fn divide(input: String) -> Vec<(HashSet<Loc>, Vec<Loc>)> {
    let mut regions = vec![];

    let mut grid: Grid<Cell> = parse_grid_with(input, Cell::new);

    for r in 0..grid.h() {
        for c in 0..grid.w() {
            let cell = grid.get(r, c);
            if cell.seen {
                continue;
            }

            let mut q = VecDeque::new();
            q.push_back(Loc::new(r, c));

            let mut area = HashSet::new();
            let mut perimeter = vec![];

            while let Some(loc) = q.pop_front() {
                let Loc { r, c } = loc;
                match grid.try_get_mut(r, c) {
                    Some(crop) if crop.kind == cell.kind => {
                        if !crop.seen {
                            crop.seen = true;
                            area.insert(loc);
                            for next in loc.adj() {
                                q.push_back(next)
                            }
                        }
                    }
                    _ => {
                        perimeter.push(loc);
                    }
                }
            }

            regions.push((area, perimeter))
        }
    }

    regions
}

pub fn part1(input: String) -> u64 {
    // println!("{:?}", divide(input));

    divide(input)
        .into_iter()
        .map(|(a, p)| a.len() * p.len())
        .sum::<usize>() as u64
}

struct Edge {
    vertical: bool,
    left_or_above: i64,
    start: i64, // < end
    end: i64,
}
impl Edge {
    fn between(a: Loc, b: Loc) -> Option<Self> {
        // A
        // _
        // B
        if a.c == b.c && (a.r - b.r).abs() == 1 {
            return Some(Edge {
                vertical: false,
                left_or_above: a.r.max(b.r),
                start: a.c,
                end: a.c + 1,
            });
        }

        // A|B
        if a.r == b.r && (a.c - b.c).abs() == 1 {
            return Some(Edge {
                vertical: true,
                left_or_above: a.c.max(b.c),
                start: a.r,
                end: a.r + 1,
            });
        }

        None
    }

    fn mergeable(&self, other: &Edge) -> bool {
        self.vertical == other.vertical
            && self.left_or_above == other.left_or_above
            && (self.end == other.start || other.end == self.start)
    }

    fn merge(&mut self, other: Edge) {
        if !self.mergeable(&other) {
            panic!("not mergeable")
        }

        if self.end == other.start {
            self.end = other.end;
        } else if self.start == other.end {
            self.start = other.start;
        } else {
            panic!("bad case")
        }
    }

    fn crosses(&self, other: &Edge) -> bool {
        self.vertical != other.vertical
            && self.left_or_above > other.start
            && self.left_or_above < other.end
            && other.left_or_above > self.start
            && other.left_or_above < self.end
    }
}

pub fn part2(input: String) -> usize {
    divide(input)
        .into_iter()
        .map(|(a, p)| {
            let p = HashSet::<Loc>::from_iter(p.into_iter());

            let mut edges: Vec<Edge> = vec![];

            // this doesnt count mobius fences
            for outer in &p {
                for inner in &a {
                    if let Some(mut edge) = Edge::between(*outer, *inner) {
                        let mut i = 0;
                        while i < edges.len() {
                            if edges[i].mergeable(&edge) {
                                edge.merge(edges.remove(i));
                            } else {
                                i += 1;
                            }
                        }
                        edges.push(edge);
                    }
                }
            }

            let mut p_cost = edges.len();

            for e1 in &edges {
                for e2 in &edges {
                    if e1.crosses(e2) {
                        p_cost += 1; // will be counted agin in the symmetrical case
                    }
                }
            }

            a.len() * p_cost
        })
        .sum::<usize>()
}

aoc_test!(
    "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
",
    1930,
    "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA",
    368,
);

use crate::grid::parse_grid_with;
use crate::util::aoc_test;
use priority_queue::priority_queue::PriorityQueue;
use std::cmp::max;
use std::ops::Not;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Dir {
    Horizontal,
    Vertical,
}
impl Not for Dir {
    type Output = Dir;
    fn not(self) -> Self::Output {
        match self {
            Dir::Horizontal => Dir::Vertical,
            Dir::Vertical => Dir::Horizontal,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Node {
    r: usize,
    c: usize,
    dir: Dir,
}

fn dijkstra(input: String, min_dist: isize, max_dist: isize) -> u64 {
    let grid = parse_grid_with(input, |chr| chr.to_digit(10).unwrap() as u64);

    let mut unvisited = PriorityQueue::new();
    for ((r, c), _) in grid.cells() {
        unvisited.push(
            Node {
                r,
                c,
                dir: Dir::Horizontal,
            },
            0,
        );
        unvisited.push(
            Node {
                r,
                c,
                dir: Dir::Vertical,
            },
            0,
        );
    }

    let start_heat = u64::MAX;
    unvisited.change_priority(
        &Node {
            r: 0,
            c: 0,
            dir: Dir::Horizontal,
        },
        start_heat,
    );
    unvisited.change_priority(
        &Node {
            r: 0,
            c: 0,
            dir: Dir::Vertical,
        },
        start_heat,
    );

    loop {
        // println!("{} nodes to process", unvisited.len());
        let (node, heat): (Node, u64) = unvisited.pop().unwrap();
        // println!("processing {:?}, dist={}", node, start_heat - heat);
        if node.r == grid.h() - 1 && node.c == grid.w() - 1 {
            return start_heat - heat;
        }

        let deltas = match node.dir {
            Dir::Horizontal => [(0, -1), (0, 1)],
            Dir::Vertical => [(-1, 0), (1, 0)],
        };

        for (dr, dc) in deltas {
            let mut dist = 0;
            for i in 1..=max_dist {
                let r = node.r.wrapping_add_signed(dr * i);
                let c = node.c.wrapping_add_signed(dc * i);
                let Some(cooling) = grid.try_get_ref(r, c) else {
                    break;
                };
                dist += cooling;

                if i < min_dist {
                    continue;
                }

                let dest_node = Node {
                    r,
                    c,
                    dir: !node.dir,
                };
                if let Some(best) = unvisited.get_priority(&dest_node) {
                    unvisited.change_priority(&dest_node, max(*best, heat - dist));
                }
            }
        }
    }
}

pub fn part1(_input: String) -> u64 {
    dijkstra(_input, 1, 3)
}
pub fn part2(_input: String) -> u64 {
    dijkstra(_input, 4, 10)
}

aoc_test!(
    "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
",
    102,
    94,
);

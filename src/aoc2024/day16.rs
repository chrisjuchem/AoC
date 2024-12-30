use crate::grid::{DeltaLoc, Loc, parse_grid};
use crate::util::aoc_test;
use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Dir {
    N,
    S,
    W,
    E,
}
impl Dir {
    fn delta(&self) -> DeltaLoc {
        match self {
            Dir::N => DeltaLoc::new(-1, 0),
            Dir::S => DeltaLoc::new(1, 0),
            Dir::W => DeltaLoc::new(0, -1),
            Dir::E => DeltaLoc::new(0, 1),
        }
    }

    fn turns(&self) -> [Self; 2] {
        match self {
            Dir::N | Dir::S => [Dir::W, Dir::E],
            Dir::W | Dir::E => [Dir::N, Dir::S],
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    loc: Loc,
    facing: Dir,
    history: HashSet<Loc>,
}
impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.loc.hash(state);
        self.facing.hash(state);
    }
}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.loc == other.loc && self.facing == other.facing
    }
}
impl Eq for State {}

fn route(input: String) -> (State, u64) {
    let grid = parse_grid(input);

    let start = grid.find(&'S').unwrap();
    let end = grid.find(&'E').unwrap();

    let mut pq = PriorityQueue::new();
    pq.push(
        State {
            loc: start,
            facing: Dir::E,
            history: HashSet::from([start]),
        },
        0i64,
    );

    let mut seen = HashMap::<State, _>::new();

    while let Some((state, score)) = pq.pop() {
        if let Some(seen_score) = seen.get(&state) {
            if score > *seen_score {
                panic!("priority q failed me");
            }
            if score == *seen_score {
                //combine histories
                let mut s = state.clone();
                println!("{:?}", s.history);
                let hist = seen
                    .iter()
                    .find_map(|(k, _v)| (k == &s).then_some(k.history.iter()))
                    .unwrap();
                println!("{:?}", hist);
                s.history.extend(hist);
                seen.remove(&s);
                seen.insert(s, score);
            }
            continue;
        }

        for d in state.facing.turns() {
            let new_state = State {
                loc: state.loc,
                facing: d,
                history: state.history.clone(),
            };
            if pq.get_priority(&new_state) == Some(&(score - 1000)) {
                pq.get_mut(&new_state)
                    .unwrap()
                    .0
                    .history
                    .extend(&new_state.history);
            }
            pq.push_increase(new_state, score - 1000);
        }

        let step = state.loc + state.facing.delta();
        if grid.get(step.r, step.c) != '#' {
            let mut h = HashSet::from([step]);
            h.extend(state.history.iter());
            let new_state = State {
                loc: step,
                facing: state.facing,
                history: h,
            };

            if pq.get_priority(&new_state) == Some(&(score - 1)) {
                pq.get_mut(&new_state)
                    .unwrap()
                    .0
                    .history
                    .extend(&new_state.history);
            }

            pq.push_increase(new_state, score - 1);
        }

        seen.insert(state, score);
    }

    seen.into_iter()
        .filter(|(state, _score)| state.loc == end)
        .map(|(state, score)| (state, ((-score) as u64)))
        .min_by_key(|(_state, score)| *score)
        .unwrap()
}

pub fn part1(input: String) -> u64 {
    route(input).1
}

pub fn part2(input: String) -> usize {
    route(input).0.history.len()
}

aoc_test!(
    "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
    11048,
    64,
);

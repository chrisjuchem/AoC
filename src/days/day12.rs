use crate::util::{aoc_test, SplitInto};

#[derive(Copy, Clone, Eq, PartialEq)]
enum State {
    Working,
    Broken,
    Unknown,
}
impl From<char> for State {
    fn from(value: char) -> Self {
        match value {
            '#' => State::Broken,
            '.' => State::Working,
            '?' => State::Unknown,
            _ => panic!("bad char"),
        }
    }
}

fn matches_groups(states: &Vec<State>, groups: &Vec<usize>) -> bool {
    states
        .group_by(|a, b| a == b)
        .filter(|slice| slice[0] == State::Broken)
        .map(|slice| slice.len())
        .eq(groups.iter().cloned())
}

fn concretize_(mut states: Vec<State>, target_broken: usize, n_unknown: usize) -> Vec<Vec<State>> {
    if target_broken == 0 {
        return vec![states
            .iter()
            .map(|s| match s {
                State::Unknown => State::Working,
                known => *known,
            })
            .collect()];
    }
    if target_broken == n_unknown {
        return vec![states
            .iter()
            .map(|s| match s {
                State::Unknown => State::Broken,
                known => *known,
            })
            .collect()];
    }

    // println!("{target_broken}, {n_unknown}");
    let idx = states.iter().position(|s| *s == State::Unknown).unwrap();
    let mut states_clone = states.clone();
    states[idx] = State::Working;
    states_clone[idx] = State::Broken;
    let mut w_working = concretize_(states, target_broken, n_unknown - 1);
    let mut w_broken = concretize_(states_clone, target_broken - 1, n_unknown - 1);
    w_broken.append(&mut w_working);
    w_broken
}

fn concretize(states: Vec<State>, groups: &Vec<usize>) -> Vec<Vec<State>> {
    let unknown_count = states.iter().filter(|s| s == &&State::Unknown).count();
    let broken_count = states.iter().filter(|s| s == &&State::Broken).count();
    concretize_(
        states,
        groups.iter().sum::<usize>() - broken_count,
        unknown_count,
    )
}

pub fn part1(input: String) -> u64 {
    input
        .lines()
        .map(|line| {
            let (states, groups) = line.split_into(" ");
            let states = states.chars().map(State::from).collect::<Vec<_>>();
            let groups = groups
                .split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect::<Vec<usize>>();

            // println!("=====================");
            concretize(states, &groups)
                .into_iter()
                .filter(|states| matches_groups(states, &groups))
                .count()
        })
        .sum::<usize>() as u64
}

pub fn part2(input: String) -> u64 {
    input
        .lines()
        .map(|line| {
            let (states, groups) = line.split_into(" ");
            let states = states.repeat(5);
            let groups = format!("{},", groups).repeat(5);

            let states = states.chars().map(State::from).collect::<Vec<_>>();
            let groups = groups
                .split(',')
                .filter(|s| !s.is_empty())
                .map(str::parse)
                .map(Result::unwrap)
                .collect::<Vec<usize>>();

            concretize(states, &groups)
                .into_iter()
                .filter(|states| matches_groups(states, &groups))
                .count()
        })
        .sum::<usize>() as u64
}

aoc_test!(
    "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
",
    21,
    "",
    0,
);

use crate::util::{SplitInto, aoc_test};

pub fn part1(input: String) -> u64 {
    let (ranges, items) = input.split_into("\n\n");
    let ranges = ranges
        .lines()
        .map(|l| {
            let (from, to) = l.split_into("-");
            from.parse::<u64>().unwrap()..=to.parse().unwrap()
        })
        .collect::<Vec<_>>();
    let mut count = 0;
    'item: for item in items.lines() {
        let i = item.parse::<u64>().unwrap();
        for r in &ranges {
            if r.contains(&i) {
                count += 1;
                continue 'item;
            }
        }
    }
    count
}

pub fn part2(input: String) -> u64 {
    let (ranges, _items) = input.split_into("\n\n");
    let mut ranges = ranges
        .lines()
        .map(|l| {
            let (from, to) = l.split_into("-");
            from.parse::<u64>().unwrap()..=to.parse().unwrap()
        })
        .collect::<Vec<_>>();

    ranges.sort_by_key(|r| *r.start());

    let mut end = 0;
    let mut count = 0;

    for r in ranges {
        if *r.start() > end {
            count += *r.end() - *r.start() + 1;
            end = *r.end();
        } else if *r.end() > end {
            count += *r.end() - end;
            end = *r.end()
        }
    }
    count
}

aoc_test!(
    "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
    3,
    14,
);

use crate::util::{aoc_test, SplitInto};
use std::cmp::{max, min};
use std::convert::Infallible;

use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct SeedRange {
    start: u64,
    len: u64,
}

struct Range {
    to: u64,
    from: u64,
    len: u64,
}
impl FromStr for Range {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (to, from, len) = s.split_into(" ");
        let (to, from, len) = (
            to.parse().unwrap(),
            from.parse().unwrap(),
            len.parse().unwrap(),
        );
        Ok(Range { to, from, len })
    }
}
impl Range {
    /// How far into this range is n, if it's in the range.
    fn offset(&self, n: u64) -> Option<u64> {
        if n >= self.from {
            let diff = n - self.from;
            if diff < self.len {
                return Some(diff);
            }
        }
        None
    }

    fn overlap(&self, r: SeedRange) -> Option<SeedRange> {
        let start = max(r.start, self.from);
        let end = min(r.start + r.len, self.from + self.len);
        (start < end).then(|| SeedRange {
            start: self.to + self.offset(start).unwrap(),
            len: end - start,
        })
    }
}

struct RangeMap {
    ranges: Vec<Range>,
}
impl RangeMap {
    fn lookup(&self, n: u64) -> u64 {
        for range in &self.ranges {
            if let Some(diff) = range.offset(n) {
                return range.to + diff;
            }
        }
        panic!("gap in range map");
    }

    fn map_range(&self, r: SeedRange) -> impl Iterator<Item = SeedRange> + '_ {
        self.ranges.iter().filter_map(move |range| range.overlap(r))
    }

    fn sorted(mut self) -> Self {
        self.ranges
            .sort_by(|range, other| range.from.cmp(&other.from));
        self
    }

    fn contiguous(mut self) -> Self {
        let mut i = 0;
        let mut n = 0;
        while n < u64::MAX {
            match self.ranges.get(i) {
                None => self.ranges.push(Range {
                    to: n,
                    from: n,
                    len: u64::MAX - n,
                }),
                Some(r) => {
                    if r.from == n {
                        i += 1;
                        n += r.len;
                        continue;
                    }
                    self.ranges.insert(
                        i,
                        Range {
                            to: n,
                            from: n,
                            len: r.from - n,
                        },
                    )
                }
            }
        }
        self
    }
}

/// https://youtu.be/CWiz_RtA1Hw?t=814
trait Captures<U> {}
impl<T: ?Sized, U> Captures<U> for T {}

fn parse_maps<'a>(
    input: impl Iterator<Item = &'a str>,
) -> impl Iterator<Item = RangeMap> + Captures<&'a str> {
    input.map(|section| {
        RangeMap {
            ranges: section
                .split("\n")
                .skip(1)
                .filter(|line| *line != "")
                .map(|line| line.parse().unwrap())
                .collect(),
        }
        .sorted()
        .contiguous()
    })
}

pub fn part1(input: String) -> u64 {
    let mut sections = input.split("\n\n");
    let mut seeds = sections
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    for map in parse_maps(sections) {
        for seed in &mut seeds {
            *seed = map.lookup(*seed);
        }
    }
    seeds.into_iter().min().unwrap()
}

pub fn part2(input: String) -> u64 {
    let mut sections = input.split("\n\n");
    let mut seeds = sections
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .array_chunks::<2>()
        .map(|[start, len]| SeedRange { start, len })
        .collect::<Vec<_>>();
    for map in parse_maps(sections) {
        seeds = seeds.into_iter().flat_map(|r| map.map_range(r)).collect()
    }
    seeds.into_iter().map(|r| r.start).min().unwrap()
}

#[cfg(test)]
mod more_tests {
    use crate::day05::{Range, RangeMap, SeedRange};

    #[test]
    fn test_overlap() {
        // 0 1  2 3 4  5 6 7 8 9 ->
        // 0 1 [5 6 7] 5 6 7 8 9
        let r = Range {
            to: 5,
            from: 2,
            len: 3,
        };
        assert_eq!(r.overlap(SeedRange { start: 0, len: 2 }), None);
        assert_eq!(r.overlap(SeedRange { start: 5, len: 2 }), None);
        assert_eq!(
            r.overlap(SeedRange { start: 0, len: 9 }),
            Some(SeedRange { start: 5, len: 3 })
        );
        assert_eq!(
            r.overlap(SeedRange { start: 0, len: 4 }),
            Some(SeedRange { start: 5, len: 2 })
        );
        assert_eq!(
            r.overlap(SeedRange { start: 3, len: 4 }),
            Some(SeedRange { start: 6, len: 2 })
        );

        let map = RangeMap { ranges: vec![r] }.contiguous();
        assert_eq!(
            map.map_range(SeedRange { start: 0, len: 10 })
                .collect::<Vec<_>>(),
            Vec::from([
                SeedRange { start: 0, len: 2 },
                SeedRange { start: 5, len: 3 },
                SeedRange { start: 5, len: 5 }
            ])
        )
    }
}

aoc_test!(
    "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
",
    35,
    46,
);

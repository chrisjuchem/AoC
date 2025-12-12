use std::collections::HashMap;

use crate::util::{CollectVec, SplitInto, aoc_test};

fn lookup(node: &str, map: &HashMap<&str, Vec<&str>>) -> u64 {
    let mut paths = 0;
    if node == "out" {
        return 1;
    }

    for next in map[node].iter() {
        paths += lookup(next, map);
    }

    paths
}

fn lookup2(
    node: &str,
    map: &HashMap<&str, Vec<&str>>,
    mut found_dac: bool,
    mut found_fft: bool,
    cache: &mut HashMap<(String, bool, bool), u64>,
) -> u64 {
    let mut paths = 0;
    if node == "out" {
        return (found_dac && found_fft).then_some(1).unwrap_or(0);
    }
    if node == "fft" {
        found_fft = true;
    }
    if node == "dac" {
        found_dac = true;
    }

    for next in map[node].iter() {
        let cache_key = (next.to_string(), found_dac, found_fft);
        let n = match cache.get(&cache_key) {
            None => {
                let n = lookup2(next, map, found_dac, found_fft, cache);
                cache.insert(cache_key, n);
                n
            }
            Some(n) => *n,
        };
        paths += n;
    }

    paths
}

pub fn part1(input: String) -> u64 {
    let mut map = HashMap::new();
    for line in input.lines() {
        let (node, conns) = line.split_into(": ");
        map.insert(node, conns.split(" ").collect_vec());
    }

    lookup("you", &map)
}

pub fn part2(input: String) -> u64 {
    let mut map = HashMap::new();
    for line in input.lines() {
        let (node, conns) = line.split_into(": ");
        map.insert(node, conns.split(" ").collect_vec());
    }

    lookup2("svr", &map, false, false, &mut HashMap::new())
}

aoc_test!(
    "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
",
    5,
    "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out",
    2,
);

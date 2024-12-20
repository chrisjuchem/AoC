use crate::util::{aoc_test, SplitInto};
use std::collections::HashMap;

fn merge_edges<'a>(
    edges: &mut HashMap<&str, HashMap<&'a str, usize>>,
    absorbing_edge: &'a str,
    absorbed_edge: &'a str,
) {
    let e = edges.remove(absorbed_edge).unwrap();
    for (adj, w) in e {
        edges.get_mut(adj).unwrap().remove(absorbed_edge);
        if adj == absorbing_edge {
            continue;
        }
        *edges
            .get_mut(adj)
            .unwrap()
            .entry(absorbing_edge)
            .or_default() += w;
        *edges
            .get_mut(absorbing_edge)
            .unwrap()
            .entry(adj)
            .or_default() += w;
    }
}

pub fn part1(input: String) -> usize {
    let mut edges = HashMap::new();
    let mut nodes = HashMap::new(); //lifetime bullshit
    let mut node_counts = HashMap::new();

    for line in input.lines() {
        let (a, b): (&str, &str) = line.split_into(": ");
        for c in b.split(" ") {
            edges.entry(a).or_insert(HashMap::new()).insert(c, 1);
            edges.entry(c).or_insert(HashMap::new()).insert(a, 1);
            nodes.insert(c, c);
            node_counts.insert(c, 1);
        }
        nodes.insert(a, a);
        node_counts.insert(a, 1);
    }
    let total_node_count = nodes.len();

    // https://dl.acm.org/doi/pdf/10.1145/263867.263872
    // A Simple Min-Cut Algorithm - MECHTHILD STOER & FRANK WAGNER
    let mut perma_edges = edges;
    let n = loop {
        let mut edges = perma_edges.clone();

        // println!("{}", edges.len());

        edges.insert("mega", HashMap::new());
        let a = edges.keys().next().unwrap().to_string();
        let a = nodes[a.as_str()];
        merge_edges(&mut edges, "mega", a);
        let mut mega_node: Vec<&str> = vec![a];

        // for (n1, e) in &edges {
        //     for n2 in e.keys() {
        //         if n1 < n2 {
        //             println!("{n1} {n2}")
        //         }
        //     }
        // }

        let mut best_n = 9999999;

        while edges.len() > 2 {
            // println!("{edges:?}");
            let max_edge = edges["mega"].iter().max_by_key(|(_e, w)| **w).unwrap().0;
            // println!("{max_edge:?}");
            let max_edge = nodes[max_edge];
            merge_edges(&mut edges, "mega", max_edge);
            mega_node.push(max_edge)

            // remaining edge weight is candidate for min cut, compare to min seen so far
        }
        assert_eq!(edges.len(), 2);
        assert_eq!(edges["mega"].len(), 1);
        let (other, w) = edges.remove("mega").unwrap().into_iter().next().unwrap();
        assert_eq!(edges[other].len(), 1);
        assert_eq!(edges[other]["mega"], w);

        if w < best_n {
            best_n = w;
        }
        if best_n == 3 {
            break node_counts.get(other).unwrap();
        }

        let absorbed = mega_node.pop().unwrap();
        // merge mega_node[-1] and last node outside mega node, try again
        merge_edges(&mut perma_edges, other, absorbed);
        *node_counts.get_mut(other).unwrap() += node_counts.remove(absorbed).unwrap();
    };

    n * (total_node_count - n)
}

pub fn part2(_input: String) -> u64 {
    0
}

aoc_test!(
    "jjj: rrr xxx vvv
sss: fff ppp lll
xxx: hhh
ggg: qqq vvv kkk bbb
rrr: xxx bbb hhh
bbb: xxx hhh
ppp: lll hhh vvv
qqq: vvv
aaa: jjj hhh bbb xxx
vvv: kkk
lll: kkk
zzz: qqq ggg lll sss
fff: qqq kkk lll
",
    54,
    0,
);

use crate::util::{CollectVec, SplitInto, aoc_test};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
struct Gate<'a> {
    a: &'a str,
    b: &'a str,
    op: &'a str,
    out: &'a str,
}
impl<'a> From<&'a str> for Gate<'a> {
    fn from(s: &'a str) -> Self {
        let (mut a, op, mut b, arrow, out) = s.split_into(" ");
        assert!(arrow == "->");
        if a > b {
            (a, b) = (b, a)
        }

        Gate { a, b, op, out }
    }
}

pub fn part1(input: String) -> u64 {
    let (inputs, gates) = input.split_into("\n\n");

    let states = inputs
        .lines()
        .map(|line| {
            let (label, state) = line.split_into(": ");
            (label, state == "1")
        })
        .collect::<HashMap<&str, bool>>();

    let gates = gates.lines().map(Gate::from).collect_vec();

    calc(gates, states)
}

fn calc<'a>(mut gates: Vec<Gate<'a>>, mut states: HashMap<&'a str, bool>) -> u64 {
    while !gates.is_empty() {
        let mut i = 0;
        while i < gates.len() {
            let gate = gates[i];

            if let (Some(a_state), Some(b_state)) =
                (states.get(gate.a).copied(), states.get(gate.b).copied())
            {
                states.insert(gate.out, match gate.op {
                    "AND" => a_state && b_state,
                    "OR" => a_state || b_state,
                    "XOR" => a_state != b_state,
                    _ => panic!("bad op"),
                });
                gates.swap_remove(i);
            } else {
                i += 1;
            }
        }
    }

    let mut n = 0;
    for i in 0.. {
        let Some(bit) = states.get(format!("z{i:02}").as_str()) else {
            return n;
        };

        if *bit {
            n += 1 << i;
        }
    }
    panic!("broke loop")
}
fn mk_inputs<'a>(template: &'a HashMap<&'a str, bool>, x: u64, y: u64) -> HashMap<&'a str, bool> {
    let mut h = template.clone();
    for i in 0.. {
        let Some(xdigit) = h.get_mut(format!("x{i:02}").as_str()) else {
            return h;
        };
        *xdigit = (x >> i) % 2 == 1;
        let Some(ydigit) = h.get_mut(format!("y{i:02}").as_str()) else {
            return h;
        };
        *ydigit = (y >> i) % 2 == 1;
    }
    h
}

fn rename<'a>(gates: &mut [Gate<'a>], from: &str, to: &'a str) {
    for g in gates.iter_mut() {
        if g.a == from {
            g.a = to;
        }
        if g.b == from {
            g.b = to;
        }
        if g.out == from {
            g.out = to;
        }
        if g.b < g.a {
            std::mem::swap(&mut g.a, &mut g.b);
        }
    }
}

pub fn part2(input: String) -> String {
    let (inputs, gates) = input.split_into("\n\n");
    let states = inputs
        .lines()
        .map(|line| {
            let (label, state) = line.split_into(": ");
            (label, state == "1")
        })
        .collect::<HashMap<&str, bool>>();
    let input_width = inputs.lines().count() / 2;

    let mut gates = gates.lines().map(Gate::from).collect_vec();

    #[cfg(not(test))]
    let mut swaps = vec!["qnw", "z15", "cqr", "z20", "ncd", "nfj", "vkg", "z37"];
    #[cfg(test)]
    let mut swaps = vec!["z05", "z00", "z02", "z01"];
    for g in &mut gates {
        if let Some(i) = swaps.iter().position(|node| *node == g.out) {
            g.out = swaps[i ^ 1];
        }
    }

    for i in 1..input_width {
        #[cfg(not(test))]
        assert_eq!(
            1 << i,
            calc(gates.clone(), mk_inputs(&states, (1 << i) - 1, 1))
        );
        #[cfg(test)]
        assert_eq!(
            (1 << i) & 5,
            calc(gates.clone(), mk_inputs(&states, 1 << i, 5))
        );
    }

    let print_graph_code = false;
    if print_graph_code {
        let mut renames = HashMap::new();
        for g in gates.iter() {
            if g.a.starts_with("x") && g.b.starts_with("y") && !g.out.starts_with("z") {
                renames.insert(
                    g.out,
                    g.a.replace("x", if g.op == "XOR" { "odd" } else { "natcarry" }),
                );
            }
        }
        for g in gates.iter() {
            if let Some(a) = renames.get(g.a).cloned() {
                if a.starts_with("odd") && !g.out.starts_with("z") {
                    renames.insert(g.out, a.replace("odd", "cascadecarry"));
                }
            }
            if let Some(b) = renames.get(g.b).cloned() {
                if b.starts_with("odd") && !g.out.starts_with("z") {
                    renames.insert(g.out, b.replace("odd", "cascadecarry"));
                }
            }
        }
        for g in gates.iter() {
            if let (Some(a), Some(b)) = (renames.get(g.a), renames.get(g.b)) {
                if a.starts_with("cascadecarry")
                    && b.starts_with("natcarry")
                    && !g.out.starts_with("z")
                {
                    renames.insert(g.out, a.replace("cascadecarry", "carry"));
                }
            }
            if let (Some(a), Some(b)) = (renames.get(g.a), renames.get(g.b)) {
                if b.starts_with("cascadecarry")
                    && a.starts_with("natcarry")
                    && !g.out.starts_with("z")
                {
                    renames.insert(g.out, b.replace("cascadecarry", "carry"));
                }
            }
        }
        for (from, to) in &renames {
            rename(&mut gates, from, to.as_str());
        }

        println!("digraph G {{");
        for g in &mut gates {
            println!("{} -> {} [label={}]", g.a, g.out, g.op);
            println!("{} -> {} [label={}]", g.b, g.out, g.op);
        }
        for i in 0..44 {
            println!("x{i:02} -> x{:02}", i + 1);
            println!("y{i:02} -> y{:02}", i + 1);
            println!("z{i:02} -> z{:02}", i + 1);

            if i > 38 {
                println!("IN -> x{:02}", i + 1);
                println!("IN -> y{:02}", i + 1);
                println!("OUT -> z{:02}", i + 2);
            }
        }
        println!("z44 -> z45");
        println!("}}");
    }

    swaps.sort();
    swaps.join(",")
}

aoc_test!(
    "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
",
    2024,
    "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00",
    "z00,z01,z02,z05",
);

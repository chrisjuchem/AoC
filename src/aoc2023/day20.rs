use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::hash::{DefaultHasher, Hash, Hasher};

struct Module<'a> {
    kind: ModuleKind<'a>,
    outputs: Vec<&'a str>,
}
#[derive(Eq, PartialEq, Debug)]
enum ModuleKind<'a> {
    Broadcast,
    FlipFlop { on: bool },
    Conjunction { inputs: HashMap<&'a str, bool> },
    Rx { got_low: bool },
}
impl<'a> From<&str> for ModuleKind<'a> {
    fn from(value: &str) -> Self {
        match value {
            "b" => ModuleKind::Broadcast,
            "%" => ModuleKind::FlipFlop { on: false },
            "&" => ModuleKind::Conjunction {
                inputs: HashMap::new(),
            },
            _ => panic!("unknown kind"),
        }
    }
}
impl<'a> Hash for ModuleKind<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            ModuleKind::Broadcast => 436278194.hash(state),
            ModuleKind::FlipFlop { on } => {
                32419873.hash(state);
                on.hash(state)
            }
            ModuleKind::Conjunction { inputs } => {
                23910434732855u64.hash(state);
                for remembered in inputs.values() {
                    remembered.hash(state)
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
struct Pulse<'a> {
    high: bool,
    from: &'a str,
    to: &'a str,
}

fn press(modules: &mut HashMap<&str, Module>) -> (u64, u64) {
    let mut to_process = VecDeque::from([Pulse {
        high: false,
        from: "button",
        to: "roadcaster",
    }]);

    let (mut highs, mut lows) = (0, 0);
    while !to_process.is_empty() {
        let pulse = to_process.pop_front().unwrap();
        if pulse.high {
            highs += 1;
        } else {
            lows += 1;
        }
        let Some(recvr) = modules.get_mut(pulse.to) else {
            continue;
        };
        let Some(signal) = (match &mut recvr.kind {
            ModuleKind::Broadcast => Some(pulse.high),
            ModuleKind::FlipFlop { on } => {
                if pulse.high {
                    None
                } else {
                    *on = !*on;
                    Some(*on)
                }
            }
            ModuleKind::Conjunction { inputs } => {
                *inputs.get_mut(pulse.from).unwrap() = pulse.high;
                Some(!inputs.values().all(|remembered| *remembered))
            }
            ModuleKind::Rx { got_low } => {
                if !pulse.high {
                    *got_low = true
                }
                None
            }
        }) else {
            continue;
        };
        for out in &recvr.outputs {
            to_process.push_back(Pulse {
                from: pulse.to,
                high: signal,
                to: *out,
            })
        }
    }
    (highs, lows)
}

fn parse_modules(input: &str) -> HashMap<&str, Module> {
    let re = Regex::new(r"^([b%&])(\S+) -> (.*)$").unwrap();
    let mut modules: HashMap<_, _> = input
        .lines()
        .map(move |line| {
            let [kind, name, outputs] = re.captures(line).unwrap().extract::<3>().1;
            (
                name,
                Module {
                    kind: kind.into(),
                    outputs: outputs.split(",").map(str::trim).collect(),
                },
            )
        })
        .collect();

    let conns: Vec<_> = modules
        .iter()
        .flat_map(|(name, module)| module.outputs.iter().map(|out| (*name, *out)))
        .collect();
    for (input, output) in conns {
        match modules.get_mut(output).map(|module| &mut module.kind) {
            Some(&mut ModuleKind::Conjunction { ref mut inputs }) => {
                inputs.insert(input, false);
            }
            _ => {}
        }
    }
    modules
}

#[derive(Debug)]
struct History {
    hash: u64,
    highs: u64,
    lows: u64,
}

fn hash(map: &HashMap<&str, Module>) -> u64 {
    let mut hasher = DefaultHasher::new();
    map.values()
        .for_each(|module| module.kind.hash(&mut hasher));
    hasher.finish()
}

pub fn part1(_input: String) -> u64 {
    let mut modules = parse_modules(&_input);

    let mut history = vec![History {
        highs: 0,
        lows: 0,
        hash: hash(&modules),
    }];

    let mut remaining_presses = 5000;
    while remaining_presses > 0 {
        let (highs, lows) = press(&mut modules);
        remaining_presses -= 1;

        let hash = hash(&modules);
        if let Some(idx) = history.iter().position(|h| h.hash == hash) {
            let cycle_len = history.len() - idx;
            println!("found cycle, length = {}", cycle_len);
            remaining_presses %= cycle_len;
        }
        history.push(History { highs, lows, hash });
    }

    let (tot_highs, tot_lows) = history
        .into_iter()
        .fold((0, 0), |(h, l), hist| (h + hist.highs, l + hist.lows));
    tot_highs * tot_lows
}

pub fn part2(_input: String) -> u64 {
    let mut modules = parse_modules(&_input);
    modules.insert(
        "rx",
        Module {
            kind: ModuleKind::Rx { got_low: false },
            outputs: Vec::new(),
        },
    );

    for i in 1.. {
        press(&mut modules);
        let pre = modules.get("xn").unwrap();
        if let ModuleKind::Conjunction { inputs } = &pre.kind {
            if inputs.values().any(|a| *a) {
                println!("i:{:?}", inputs);
            }
        }
        if let ModuleKind::Rx { got_low: true } = modules.get("rx").unwrap().kind {
            return i;
        }
    }
    panic!("broke loop")
}

// aoc_test!(
//     "broadcaster -> a
// %a -> inv, con
// &inv -> b
// %b -> con
// &con -> rx
// ",
//     11687500,
//     1,
// );

//jv rn
// jv jn
// fb hb
// fb vk
// fb fz
// fb kl
// fb cg
// rr vm
// rr gp
// gp vm
// gp cb
// gp bd
// gp qm
// gp xf
// gp pk
// hm ql
// cf dx
// cf fb
// cg kl
// hv kg
// hv fb
// hs jv
// bd dt
// xv mv
// xv gp
// js zb
// js jl
// rn bk
// rn jn
// lp hm
// dx fb
// dx jm
// ss lp
// hn xn
// bh jl
// bh ms
// km jl
// km lm
// mv gp
// mv qm
// jl km
// jl lm
// jl ms
// jl mp
// jl lr
// jl zb
// jl bg
// pt jt
// pt jl
// cb bd
// xt jn
// xt jf
// kg fb
// dg jn
// rt fb
// rt hb
// broadcaster km
// broadcaster xt
// broadcaster pk
// broadcaster vk
// lr pt
// vm bf
// hx qd
// hx jl
// mp xn
// hb pd
// vk cg
// vk fb
// kl rs
// pk gp
// pk cb
// jt hx
// jt jl
// jn hs
// jn lp
// jn hm
// jn hn
// jn ql
// jn xt
// jn ss
// bg js
// kz ss
// kz jn
// bf fx
// bf gp
// bk  dg
// bk jn
// qm  rr
// fx gp
// fx dp
// dp  gp
// jf  jn
// jf kz
// jm hv
// jm fb
// ql  hs
// ms  bg
// zb  lr
// rs  fb
// rs rt
// dt  xv
// dt gp
// lm  bh
// xf  xn
// pd  cf
// pd fb
// qd  jl
// xn  rx
// fz  xn

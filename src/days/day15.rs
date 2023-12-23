use crate::util::{aoc_test, SplitInto};

fn hash(s: &str) -> usize {
    let mut cur: u8 = 0;
    for c in s.chars() {
        cur = cur.wrapping_add(c as u8).wrapping_mul(17)
    }
    cur as usize
}

#[derive(Clone, Debug)]
struct Lens<'a> {
    label: &'a str,
    focal_length: usize,
}

pub fn part1(_input: String) -> u64 {
    _input.trim().split(",").map(hash).sum::<usize>() as u64
}

pub fn part2(_input: String) -> u64 {
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
    for instr in _input.trim().split(",") {
        if instr.contains('=') {
            let (label, focal_length) = instr.split_into("=");
            let focal_length = focal_length.parse().unwrap();
            let bx = boxes.get_mut(hash(label)).unwrap();
            match bx.iter_mut().find(|lens| lens.label == label) {
                None => bx.push(Lens {
                    label,
                    focal_length,
                }),
                Some(lens) => lens.focal_length = focal_length,
            }
        } else {
            let label = instr.split('-').next().unwrap();
            let bx = boxes.get_mut(hash(label)).unwrap();
            if let Some(lens_loc) = bx.iter().position(|lens| lens.label == label) {
                bx.remove(lens_loc);
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(bx_num, bx)| {
            bx.iter()
                .enumerate()
                .map(move |(slot_num, lens)| (bx_num + 1) * (slot_num + 1) * lens.focal_length)
        })
        .flatten()
        .map(|n| n)
        .sum::<usize>() as u64
}

aoc_test!(
    "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
",
    1320,
    145,
);

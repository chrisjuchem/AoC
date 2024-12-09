use crate::util::{aoc_test, CollectVec};

fn parse_file_blocks(input: String) -> Vec<Option<u64>> {
    input
        .trim()
        .char_indices()
        .flat_map(|(i, c)| {
            let n = c.to_digit(10).unwrap() as usize;

            let is_file = i % 2 == 0;
            let idx = i / 2;

            std::iter::repeat_n(is_file.then_some(idx as u64), n)
        })
        .collect_vec()
}

pub fn part1(input: String) -> u64 {
    let mut blocks = parse_file_blocks(input);

    let mut p1 = 0;
    let mut p2 = blocks.len() - 1;

    while p1 < p2 {
        let n1 = blocks[p1];
        let n2 = blocks[p2];

        if n1.is_none() && n2.is_some() {
            // println!("swap {p1} {p2}");
            blocks.swap(p1, p2);
            p1 += 1;
            p2 -= 1;
            continue;
        }

        if n1.is_some() {
            p1 += 1;
            // println!("inc p1 to {p1}");
        }
        if n2.is_none() {
            p2 -= 1;
            // println!("dec p2 to {p2}");
        }
    }

    // println!("{:?}", blocks);
    blocks
        .into_iter()
        .enumerate()
        .filter_map(|(i, b)| b.map(|n| n * i as u64))
        .sum()
}

#[derive(Debug)]
struct File {
    id: u64,
    size: u64,
    trailing_space: u64,
}
impl File {
    fn checksum(&self, i: u64) -> u64 {
        (i..(i + self.size)).sum::<u64>() * self.id
    }
    // fn print(&self) {
    //     print!(
    //         "{}{}",
    //         (self.id % 10).to_string().repeat(self.size as usize),
    //         ".".repeat(self.trailing_space as usize),
    //     )
    // }
}

pub fn part2(input: String) -> u64 {
    let mut files: Vec<File> = input
        .trim()
        .chars()
        .chain(std::iter::once('0'))
        .array_chunks::<2>()
        .enumerate()
        .map(|(i, [f, s])| File {
            id: i as u64,
            size: f.to_digit(10).unwrap() as u64,
            trailing_space: s.to_digit(10).unwrap() as u64,
        })
        .collect_vec();

    let total_len: u64 = files.iter().map(|f| f.size + f.trailing_space).sum();

    let mut i = files.len() - 1;
    'outer: while i > 0 {
        let fsize = files[i].size;

        for j in 0..i {
            if files[j].trailing_space >= fsize {
                //remove
                let mut f = files.remove(i);
                files[i - 1].trailing_space += f.size + f.trailing_space;
                //insert
                f.trailing_space = files[j].trailing_space - f.size;
                files.insert(j + 1, f);
                files[j].trailing_space = 0;
                continue 'outer;
            }
        }
        i -= 1;
    }

    assert_eq!(
        total_len,
        files.iter().map(|f| f.size + f.trailing_space).sum()
    );

    let mut checksum = 0;
    let mut i = 0;
    for f in files {
        checksum += f.checksum(i);
        i += f.size + f.trailing_space;
    }
    println!();
    checksum as u64
}

aoc_test!("2333133121414131402", 1928, 2858,);

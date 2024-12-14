use crate::util::aoc_test;

// enum Either<A, B> {
//     A(A),
//     B(B),
// }
// impl<T, A, B> Iterator for Either<A, B>
// where
//     A: Iterator<Item = T>,
//     B: Iterator<Item = T>,
// {
//     type Item = T;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         match self {
//             Either::A(a) => a.next(),
//             Either::B(b) => b.next(),
//         }
//     }
// }

enum Node {
    Num(String),
    Nodes(Box<(Node, Node)>),
}
impl Node {
    fn new(s: &(impl ToString + ?Sized)) -> Self {
        Node::Num(s.to_string())
    }
    fn step(&mut self) {
        match self {
            Node::Num(s) => {
                if *s == "0" {
                    *self = Node::new("1");
                } else if s.len() % 2 == 0 {
                    *self = Node::Nodes(Box::new((
                        Node::new(&s[0..s.len() / 2].parse::<u64>().unwrap()),
                        Node::new(&s[s.len() / 2..s.len()].parse::<u64>().unwrap()),
                    )))
                } else {
                    *self = Node::new(&(s.parse::<u64>().unwrap() * 2024))
                }
            }
            Node::Nodes(bx) => {
                let (ref mut a, ref mut b) = **bx;
                a.step();
                b.step();
            }
        }
    }

    fn count(&self) -> u64 {
        match self {
            Node::Num(_) => 1,
            Node::Nodes(bx) => {
                let (ref a, ref b) = **bx;
                a.count() + b.count()
            }
        }
    }
    // fn into_iter(self) -> impl Iterator<Item = String> {
    //     match self {
    //         Node::Num(s) => Either::A(std::iter::once(s)),
    //         Node::Nodes(bx) => {
    //             let (a, b) = *bx;
    //             Either::B(a.into_iter().chain(b.into_iter()))
    //         }
    //     }
    // }
}

fn blink(input: String, n: usize) -> u64 {
    let mut stones = input
        .trim()
        .split(" ")
        .map(Node::new)
        .reduce(|a, b| Node::Nodes(Box::new((a, b))))
        .unwrap();

    for i in 0..n {
        println!("{i}");
        stones.step()
    }

    stones.count()
}

pub fn part1(input: String) -> u64 {
    blink(input, 25)
}
pub fn part2(input: String) -> u64 {
    0
    // todo speedup
    // blink(input, 75)
}

aoc_test!("125 17", 55312, 0,);

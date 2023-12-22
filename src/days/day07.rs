use crate::util::{aoc_test, SplitInto};
use std::collections::HashMap;

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
enum Card {
    JOKER,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
    ACE,
}
impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'j' => Card::JOKER,
            '2' => Card::TWO,
            '3' => Card::THREE,
            '4' => Card::FOUR,
            '5' => Card::FIVE,
            '6' => Card::SIX,
            '7' => Card::SEVEN,
            '8' => Card::EIGHT,
            '9' => Card::NINE,
            'T' => Card::TEN,
            'J' => Card::JACK,
            'Q' => Card::QUEEN,
            'K' => Card::KING,
            'A' => Card::ACE,
            _ => panic!("bad card"),
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn of(cards: &[Card]) -> Self {
        let mut map = HashMap::new();
        for card in cards {
            *map.entry(card).or_insert(0) += 1
        }
        let jokers = map.remove(&Card::JOKER).unwrap_or(0);
        let mut counts = map.into_values().collect::<Vec<_>>();
        counts.sort_by(|a, b| b.cmp(a)); //rev
        match counts.get_mut(0) {
            None => counts.push(jokers),
            Some(n) => *n += jokers,
        };
        match &counts[..] {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => panic!("bad hand"),
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Hand {
    kind: HandType,
    cards: [Card; 5],
    bid: u64,
}
impl Hand {
    fn new(cardstr: &str, bid: &str, jokers: bool) -> Self {
        let cards: [_; 5] = cardstr
            .chars()
            .map(|c| if jokers && c == 'J' { 'j' } else { c })
            .map(Card::from)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Hand {
            kind: HandType::of(&cards),
            cards,
            bid: bid.parse().unwrap(),
        }
    }
}
fn calc(input: String, jokers: bool) -> u64 {
    let mut hands = input
        .split("\n")
        .filter(|ln| *ln != "")
        .map(|line| {
            let (cards, bid) = line.split_into(" ");
            Hand::new(cards, bid, jokers)
        })
        .collect::<Vec<_>>();
    hands.sort();
    return hands
        .iter()
        .enumerate()
        .fold(0, |sum, (i, hand)| sum + ((i as u64 + 1) * hand.bid));
}

pub fn part1(input: String) -> u64 {
    calc(input, false)
}
pub fn part2(input: String) -> u64 {
    calc(input, true)
}

aoc_test!(
    "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
",
    6440,
    5905,
);

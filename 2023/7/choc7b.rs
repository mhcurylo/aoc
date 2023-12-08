extern crate combine;
use std::fs;

use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

type HandValue = [u8; 5];

#[derive(Debug, Eq)]
struct Hand {
    hand_type: HandType,
    hand_value: HandValue,
    bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            return self.hand_value.cmp(&other.hand_value);
        };
        self.hand_type.cmp(&other.hand_type)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
            && self.hand_value == other.hand_value
            && self.bid == other.bid
    }
}

fn encode_card(c: char) -> u8 {
    match c {
        'J' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("this ain't a card"),
    }
}

const JOKER: u8 = 0;

fn get_hand_type(hv: HandValue) -> HandType {
    let mut counts: Vec<(usize, u8)> = hv
        .iter()
        .map(|c| (hv.iter().filter(|ic| *ic == c && *c != JOKER).count(), *c))
        .collect();

    counts.sort_by(|a, b| b.cmp(a));
    counts.dedup();

    let just_counts: Vec<usize> = counts.iter().map(|(c, _)| *c).collect();

    let joker_count: usize = hv.iter().filter(|c| **c == JOKER).count();
    let largest_count: usize = *just_counts.first().expect("no way there is no cards");
    let second_count: usize = *just_counts.get(1).unwrap_or(&0);

    println!("{}, {}, {}", largest_count, second_count, joker_count);

    match (largest_count, second_count, joker_count) {
        (5, 0, 0) => HandType::FiveOfAKind,
        (4, 0, 1) => HandType::FiveOfAKind,
        (4, 1, 0) => HandType::FourOfAKind,
        (3, 2, 0) => HandType::FullHouse,
        (3, 1, 1) => HandType::FourOfAKind,
        (3, 0, 2) => HandType::FiveOfAKind,
        (3, 1, 0) => HandType::ThreeOfAKind,
        (2, 0, 3) => HandType::FiveOfAKind,
        (2, 1, 2) => HandType::FourOfAKind,
        (2, 2, 1) => HandType::FullHouse,
        (2, 1, 1) => HandType::ThreeOfAKind,
        (2, 2, 0) => HandType::TwoPair,
        (2, 1, 0) => HandType::OnePair,
        (1, 0, 4) => HandType::FiveOfAKind,
        (1, 1, 3) => HandType::FourOfAKind,
        (1, 1, 2) => HandType::ThreeOfAKind,
        (1, 1, 1) => HandType::OnePair,
        (1, 1, 0) => HandType::HighCard,
        (0, 0, 5) => HandType::FiveOfAKind,
        _otherwise => panic!("oh no how come"),
    }
}

fn parse_hand_value(str: &str) -> HandValue {
    let s: Vec<u8> = str.chars().map(encode_card).collect();

    [
        *s.first().expect("0"),
        *s.get(1).expect("1"),
        *s.get(2).expect("2"),
        *s.get(3).expect("3"),
        *s.get(4).expect("4"),
    ]
}

fn parse_bid(s: &str) -> usize {
    s.trim().parse::<usize>().expect("bid should be")
}

fn parse_hand(s: &str) -> Hand {
    let (hand_string, bid_string) = s.split_once(' ').expect("This should be splittable");
    let hand_value = parse_hand_value(hand_string);
    let hand_type = get_hand_type(hand_value);
    let bid = parse_bid(bid_string);

    Hand {
        hand_value,
        hand_type,
        bid,
    }
}

fn solve(file_name: &str) -> usize {
    let file_contents = fs::read_to_string(file_name).expect("Was not able to read the file");
    let mut hands: Vec<Hand> = file_contents.lines().map(parse_hand).collect::<Vec<Hand>>();

    hands.sort();

    hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum()
}

fn main() {
    let test = solve("./2023/7/test.txt");
    let input = solve("./2023/7/input.txt");

    println!("Test: {}", test);
    println!("Input: {}", input);
}

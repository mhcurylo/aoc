use std::fs;

#[derive(Debug)]
struct Card {
    wins: Vec<i64>,
    has: Vec<i64>,
}

fn parse_num_sequence(line: String) -> Vec<i64> {
    line.split_whitespace()
        .map(|w| w.parse::<i64>().expect(&line))
        .collect()
}

fn parse_line(line: &str) -> Card {
    let (_, nums) = line.split_once(':').expect(line);
    let (wins_ascii, has_ascii) = nums.split_once('|').expect("NO CHANCE!");

    let wins = parse_num_sequence(wins_ascii.to_string());
    let has = parse_num_sequence(has_ascii.to_string());
    println!("has {:?} win {:?}", has, wins);

    Card { wins, has }
}

fn card_points(card: Card) -> i64 {
    let wins = card
        .has
        .iter()
        .filter(|x| card.wins.contains(x))
        .collect::<Vec<&i64>>();

    println!("matching {:?}", wins);

    if wins.len() == 1 {
        1
    } else if wins.is_empty() {
        0
    } else {
        2_i64.pow(wins.len() as u32 - 1)
    }
}

fn solve(file_name: &str) -> i64 {
    let text = fs::read_to_string(file_name).expect("Was not able to read the file");

    text.lines().map(parse_line).map(card_points).sum()
}

fn main() {
    let test = solve("./2023/4/test.txt");
    let input = solve("./2023/4/input.txt");

    println!("Test: {:?}", test);
    println!("Input: {:?}", input);
}

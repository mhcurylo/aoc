use std::fs;

#[derive(Debug)]
struct Card {
    wins: Vec<usize>,
    has: Vec<usize>,
}

fn parse_num_sequence(line: String) -> Vec<usize> {
    line.split_whitespace()
        .map(|w| w.parse::<usize>().expect(&line))
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

fn card_points(card: Card) -> usize {
    card.has.iter().filter(|x| card.wins.contains(x)).count()
}

fn solve(file_name: &str) -> usize {
    let text = fs::read_to_string(file_name).expect("Was not able to read the file");

    let vec: Vec<usize> = text.lines().map(parse_line).map(card_points).collect();

    let mut copies = vec![1; vec.len()];

    for (i, p) in vec.iter().enumerate() {
        let c = copies[i];
        for ic in (i + 1)..(i + 1 + p) {
            copies[ic] += c;
        }
    }

    copies.iter().sum()
}

fn main() {
    let test = solve("./2023/4/test.txt");
    let input = solve("./2023/4/input.txt");

    println!("Test: {:?}", test);
    println!("Input: {:?}", input);
}

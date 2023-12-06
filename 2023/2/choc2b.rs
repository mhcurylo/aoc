use std::cmp::max;
use std::fs;

#[derive(Debug)]
struct Bag {
    blue: i32,
    red: i32,
    green: i32,
}

#[derive(Debug)]
struct Game {
    plays: Vec<Bag>,
}

fn min_bag(bag_one: Bag, bag_two: &Bag) -> Bag {
    Bag {
        blue: max(bag_one.blue, bag_two.blue),
        red: max(bag_one.red, bag_two.red),
        green: max(bag_one.green, bag_two.green),
    }
}

fn power_of_min_bag(game: Game) -> i32 {
    let bag = game.plays.iter().fold(
        Bag {
            red: 0,
            blue: 0,
            green: 0,
        },
        min_bag,
    );

    bag.red * bag.green * bag.blue
}

fn parse_first_num(s: &str) -> i32 {
    s.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .expect("BOOM FIRST NUM")
}

fn parse_bag(line: &str) -> Bag {
    let tokens = line.split(",");
    let red: Vec<i32> = tokens
        .clone()
        .filter(|x| x.contains("red"))
        .map(parse_first_num)
        .collect();
    let green: Vec<i32> = tokens
        .clone()
        .filter(|x| x.contains("green"))
        .map(parse_first_num)
        .collect();
    let blue: Vec<i32> = tokens
        .clone()
        .filter(|x| x.contains("blue"))
        .map(parse_first_num)
        .collect();

    Bag {
        blue: *blue.first().unwrap_or(&0),
        red: *red.first().unwrap_or(&0),
        green: *green.first().unwrap_or(&0),
    }
}

fn parse_line(line: &str) -> Game {
    let (_, sets) = line.split_once(':').expect(line);
    let bags: Vec<Bag> = sets.split(";").map(parse_bag).collect();

    Game { plays: bags }
}

fn solve(file_name: &str) -> i32 {
    let text = fs::read_to_string(file_name).expect("Was not able to read the file");

    text.lines().map(parse_line).map(power_of_min_bag).sum()
}

fn main() {
    let test = solve("./2023/2/test.txt");
    let input = solve("./2023/2/input.txt");

    println!("Test: {:?}", test);
    println!("Input: {:?}", input);
}

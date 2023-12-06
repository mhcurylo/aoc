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
    id: i32
}

const puzzle_bag: Bag = Bag { red:12, green:13, blue: 14};

fn is_playable_with_puzzle_bag (game: &Game) -> bool {

    game.plays.iter().all(|p| p.red <= puzzle_bag.red && p.blue <= puzzle_bag.blue && p.green <= puzzle_bag.green)
}

fn parse_first_num(s: &str) -> i32 {
    s.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().expect("BOOM FIRST NUM")
}

fn parse_bag(line: &str) -> Bag {
    let tokens = line.split(",");
    let red: Vec<i32> = tokens.clone().filter(|x| x.contains("red")).map(parse_first_num).collect();
    let green: Vec<i32> = tokens.clone().filter(|x| x.contains("green")).map(parse_first_num).collect();
    let blue: Vec<i32> = tokens.clone().filter(|x| x.contains("blue")).map(parse_first_num).collect();


    Bag { blue: *blue.first().unwrap_or(&0), red: *red.first().unwrap_or(&0), green: *green.first().unwrap_or(&0)}
}

fn parse_line(line: &str) -> Game {
    let (game, sets) = line.split_once(':').expect(line);
    let bags: Vec<Bag> = sets.split(";").map(parse_bag).collect();
    let game_no = parse_first_num(game);

    Game {plays: bags, id: game_no}
}

fn solve(file_name: &str) -> i32 {
    let text = fs::read_to_string(file_name).expect("Was not able to read the file");

    text.lines()
        .map(parse_line)
        .filter(is_playable_with_puzzle_bag)
        .map(|x| x.id)
        .sum()
}

fn main() {
    let test = solve("./2023/2/test.txt");
    let input = solve("./2023/2/input.txt");

    println!("Test: {:?}", test);
    println!("Input: {:?}", input);
}

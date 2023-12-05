use std::fs;

enum Bag {
    Red(i32),
    Blue(i32),
    Green(i32),
}

fn parse_line(line: String) -> Vec<Bag> {}

fn solve(file_name: &str) -> i32 {
    let text = fs::read_to_string(file_name).expect("Was not able to read the file");

    text.lines()
}

fn main() {
    let test = solve("./2023/1/test.txt");
    let input = solve("./2023/1/input.txt");

    println!("Test: {}", test);
    println!("Input: {}", input);
}

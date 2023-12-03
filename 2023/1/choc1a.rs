use std::fs;

fn solve(file_name: &str) -> i32 {
    let text = fs::read_to_string(file_name).expect("Was not able to read the file");

    text.lines()
        .map(|l| l.chars().filter(|c| c.is_ascii_digit()))
        .map(|mut l| {
            let first = l.next().expect("first digit not found");
            let last = l.next_back().unwrap_or(first);

            [first, last]
                .iter()
                .collect::<String>()
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}

fn main() {
    let test = solve("./2023/1/test.txt");
    let input = solve("./2023/1/input.txt");

    println!("Test: {}", test);
    println!("Input: {}", input);
}

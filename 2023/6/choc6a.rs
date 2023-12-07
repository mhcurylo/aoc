extern crate combine;
use std::fs;

fn parse_numbers(s: &str) -> Vec<usize> {
    s.trim()
        .split(' ')
        .flat_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<usize>>()
}

fn count_of_wins(time: usize, min_win_distance: usize) -> usize {
    let start: usize = 0;
    let end = time;
    let is_win = |x: usize| x > min_win_distance;
    let distance = |b: usize| (b * (time - b));

    (start..end).filter(|t| is_win(distance(*t))).count()
}

fn solve(file_name: &str) -> usize {
    let file_contents = fs::read_to_string(file_name).expect("Was not able to read the file");
    let mut text = file_contents.lines();

    let times: Vec<usize> = text
        .next()
        .iter()
        .flat_map(|x| x.split_once(':'))
        .flat_map(|(_, s)| parse_numbers(s))
        .collect();

    let distances: Vec<usize> = text
        .next()
        .iter()
        .flat_map(|x| x.split_once(':'))
        .flat_map(|(_, s)| parse_numbers(s))
        .collect();

    times
        .iter()
        .zip(distances)
        .map(|(time, distance)| count_of_wins(*time, distance))
        .product()
}

fn main() {
    let test = solve("./2023/6/test.txt");
    let input = solve("./2023/6/input.txt");

    println!("Test: {}", test);
    println!("Input: {}", input);
}

extern crate combine;
use std::fs;

use std::collections::HashMap;

type Turns = Vec<bool>;

type LocationMap = HashMap<String, (String, String)>;

fn parse_turns(str: &str) -> Turns {
    str.chars().map(|x| x == 'L').collect()
}

fn parse_location(str: &str) -> (String, (String, String)) {
    let (name, roads) = str.split_once('=').expect(str);
    let road_vec = roads
        .split(',')
        .map(|r| {
            r.chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
        })
        .collect::<Vec<String>>();
    let road_one = road_vec.first().expect("road one not present");
    let road_two = road_vec.get(1).expect("road two not present");

    (
        name.trim().to_string(),
        (road_one.clone(), road_two.clone()),
    )
}

fn travel(
    current_location: &String,
    turn_no: usize,
    location_map: &LocationMap,
    turns: &Turns,
) -> usize {
    let (left, right) = location_map.get(current_location).expect(current_location);

    let is_left = turns.get(turn_no % turns.len()).expect("no f# way");

    let next_location = if *is_left { left } else { right };

    if next_location == "ZZZ" {
        turn_no
    } else {
        travel(next_location, turn_no + 1, location_map, turns)
    }
}

fn solve(file_name: &str) -> usize {
    let file_contents = fs::read_to_string(file_name).expect("Was not able to read the file");

    let turns: Turns = file_contents
        .lines()
        .take(1)
        .flat_map(parse_turns)
        .collect::<Vec<bool>>();

    let graph: LocationMap = file_contents
        .lines()
        .skip(2)
        .map(parse_location)
        .collect::<LocationMap>();

    travel(&"AAA".to_string(), 0, &graph, &turns) + 1
}

fn main() {
    let test = solve("./2023/8/test.txt");
    let input = solve("./2023/8/input.txt");

    println!("Test: {}", test);
    println!("Input: {}", input);
}

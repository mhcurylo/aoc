extern crate microbench;
use std::fs;

use std::collections::HashMap;

use microbench::Options;

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

#[derive(Debug)]
struct Dod {
    left: [u16; 1024],
    right: [u16; 1024],
    is_end: [bool; 1024],
    starting: Vec<u16>,
}

type TurnsDod<'a> = Vec<&'a [u16; 1024]>;

const LARGEST: u16 = u16::MAX;

fn into_dod(loc: &LocationMap) -> Dod {
    let mut left: [u16; 1024] = [LARGEST; 1024];
    let mut right: [u16; 1024] = [LARGEST; 1024];
    let mut is_end: [bool; 1024] = [false; 1024];
    let indexing: HashMap<&String, u16> =
        loc.keys().enumerate().map(|(i, k)| (k, i as u16)).collect();
    let mut starting: Vec<u16> = Vec::new();

    for (loc, (to_left, to_right)) in loc.iter() {
        let index_loc = *indexing.get(loc).expect("Your loc index is wrong");
        let index_to_left = *indexing.get(to_left).expect("Your left index is wrong");
        let index_to_right = *indexing.get(to_right).expect("Your right index is wrong");

        left[index_loc as usize] = index_to_left;
        right[index_loc as usize] = index_to_right;
        is_end[index_loc as usize] = loc.as_bytes().get(2) == Some(&('Z' as u8));
        if loc.as_bytes().get(2) == Some(&('A' as u8)) {
            starting.push(index_loc)
        }

        /*
        println!(
            "{:? } - {:?} - left {:?} - {:?} right {:?} - {:?} e {:?}",
            loc,
            index_loc,
            to_left,
            index_to_left,
            to_right,
            index_to_right,
            is_end[index_loc as usize],
        );
        */
    }

    Dod {
        left,
        right,
        is_end,
        starting,
    }
}

fn travel(location_map: &LocationMap, turns: &Turns) -> usize {
    let mut turn: usize = 0;
    let mut locations: Vec<&String> = location_map
        .keys()
        .filter(|c| c.as_bytes().get(2) == Some(&('A' as u8)))
        .collect();

    while !locations
        .iter()
        .all(|loc| loc.as_bytes().get(2) == Some(&('Z' as u8)))
    {
        locations = locations
            .iter()
            .map(|current_location| {
                let (left, right) = location_map.get(*current_location).expect(current_location);

                let is_left = turns.get(turn % turns.len()).expect("no f# way");

                if *is_left {
                    left
                } else {
                    right
                }
            })
            .collect();

        turn += 1;
    }

    turn
}

fn travel_dod(turns: &TurnsDod, dod: &Dod) -> usize {
    let is_end = dod.is_end;
    let mut current = dod.starting.clone();
    let mut turn: usize = 0;
    let turn_len = turns.len();

    while !current.iter().all(|i| is_end[*i as usize]) {
        let next_source = turns.get(turn % turn_len).expect("OH NO");

        current = current.iter().map(|l| next_source[*l as usize]).collect();
        turn += 1;
    }

    turn
}

fn solve(file_name: &str, bench: bool) -> usize {
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

    let options = Options::default();

    let dod: Dod = into_dod(&graph);
    let opt_turns: Vec<&[u16; 1024]> = turns
        .iter()
        .map(|t| if *t { &dod.left } else { &dod.right })
        .collect();

    if bench {
        microbench::bench(&options, "iterative_32", || travel(&graph, &turns));
        microbench::bench(&options, "iterative_32", || travel_dod(&opt_turns, &dod));
    }
    travel_dod(&opt_turns, &dod)
}

fn main() {
    let test = solve("./2023/8/test_b.txt", true);
    println!("Test: {}", test);

    let input = solve("./2023/8/input.txt", false);
    println!("Input: {}", input);
}

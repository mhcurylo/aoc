use std::fs;

#[derive(Debug)]
struct RangeMap {
    destination: usize,
    source: usize,
    range: usize,
}

fn parse_nums(strs: &str) -> Vec<usize> {
    strs.trim()
        .split(' ')
        .map(|x| x.parse::<usize>().expect("boom, not a number"))
        .collect()
}

fn to_range_map(vec: Vec<usize>) -> RangeMap {
    RangeMap {
        destination: *vec.first().expect("no from"),
        source: *vec.get(1).expect("no to"),
        range: *vec.get(2).expect("no by"),
    }
}

fn to_range_maps(strs: &str) -> Vec<RangeMap> {
    strs.lines()
        .skip(1)
        .map(parse_nums)
        .map(to_range_map)
        .collect::<Vec<RangeMap>>()
}

fn get_from_range(v: usize, rs: &RangeMap) -> usize {
    v + rs.destination - rs.source
}

fn is_in_range(v: usize, rs: &RangeMap) -> bool {
    v >= rs.source && v < (rs.source + rs.range)
}

fn get_from_ranges(v: usize, rs: &[RangeMap]) -> usize {
    rs.iter()
        .find(|r| is_in_range(v, r))
        .map(|r| get_from_range(v, r))
        .unwrap_or(v)
}

fn get_from_subsequent_ranges(v: usize, rss: &[Vec<RangeMap>]) -> usize {
    rss.iter().fold(v, |acc, e| get_from_ranges(acc, e))
}

fn expand_seeds(seeds: &[usize]) -> Vec<usize> {
    seeds
        .chunks(2)
        .flat_map(|ab| std::ops::Range {
            start: ab[0],
            end: ab[0] + ab[1],
        })
        .collect::<Vec<usize>>()
}

fn solve(file_name: &str) -> usize {
    let text = fs::read_to_string(file_name).expect("Was not able to read the file");

    let mut all_them_datas = text.split("\n\n");

    let (_, seeds_str) = all_them_datas
        .next()
        .expect("no seeds")
        .split_once(':')
        .expect("no seeds split");

    let rest_of_datas = all_them_datas.collect::<Vec<&str>>();

    let seeds = parse_nums(seeds_str);

    let maps = rest_of_datas
        .iter()
        .cloned()
        .map(to_range_maps)
        .collect::<Vec<Vec<RangeMap>>>();

    let s = expand_seeds(&seeds);

    println!("{}", s.len());
    s.iter()
        .map(|s| get_from_subsequent_ranges(*s, &maps))
        .min()
        .expect("na value!")
}

fn main() {
    let test = solve("./2023/5/test.txt");
    let input = solve("./2023/5/input.txt");

    println!("Test: {:?}", test);
    println!("Input: {:?}", input);
}

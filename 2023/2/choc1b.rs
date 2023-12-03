use std::fs;

fn numbers(str: String) -> Vec<char> {
    let chars = str.chars();

    chars
        .enumerate()
        .flat_map(|(i, c)| {
            let (_, s) = str.split_at(i);

            println!("{}", s);
            match s {
                _ if c.is_ascii_digit() => Some(c),
                s if s.starts_with("one") => Some('1'),
                s if s.starts_with("two") => Some('2'),
                s if s.starts_with("three") => Some('3'),
                s if s.starts_with("four") => Some('4'),
                s if s.starts_with("five") => Some('5'),
                s if s.starts_with("six") => Some('6'),
                s if s.starts_with("seven") => Some('7'),
                s if s.starts_with("eight") => Some('8'),
                s if s.starts_with("nine") => Some('9'),
                s if s.starts_with("zero") => Some('0'),
                _ => None,
            }
        })
        .collect()
}

fn solve(file_name: &str) -> i32 {
    let text = fs::read_to_string(file_name).expect("Was not able to read the file");

    text.lines()
        .map(|x| x.to_string())
        .map(numbers)
        .map(|x| {
            let mut iter = x.iter();
            let first = iter.next().expect("no first value!");
            let second = iter.next_back().unwrap_or(first);

            [*first, *second]
                .iter()
                .collect::<String>()
                .parse::<i32>()
                .expect("no way it is a mistake")
        })
        .sum()
    /*
        text.lines()
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
    */
}

fn main() {
    let test = solve("./2023/1/test.txt");
    let input = solve("./2023/1/input.txt");

    println!("Test: {}", test);
    println!("Input: {}", input);
}

use std::fs;

fn is_gear(u: u8) -> bool {
    (u as char) == '*'
}

fn parse_number(line: &[u8]) -> i64 {
    let ascii: String = line
        .iter()
        .take_while(|x| x.is_ascii_digit())
        .map(|x| *x as char)
        .collect();

    println!("GEAR! {}", ascii);

    ascii.parse::<i64>().expect("should have been a number")
}

fn get_number(i: usize, line: &[u8]) -> Vec<i64> {
    if !line[i].is_ascii_digit() {
        return Vec::new();
    }
    for x in (0..i).rev() {
        if !line[x].is_ascii_digit() {
            return [parse_number(&line[(x + 1)..])].to_vec();
        } else if x == 0 {
            return [parse_number(line)].to_vec();
        }
    }
    Vec::new()
}

fn get_numbers(i: usize, line: &[u8]) -> Vec<i64> {
    let c = line[i] as char;
    if c.is_ascii_digit() {
        get_number(i, line)
    } else {
        let mut numbers = get_number(i - 1, line);
        numbers.append(&mut get_number(i + 1, line));

        numbers
    }
}

fn solve(file_name: &str) -> i64 {
    let text = fs::read_to_string(file_name).expect("Was not able to read the file");

    let mut init_lines = text.lines().map(|x| x.as_bytes());

    let mut next_line: &[u8] = init_lines.next().expect("No second line");
    let line_length = next_line.len();
    let replacement_string: String = ".".repeat(line_length);
    let replacement_u8: &[u8] = replacement_string.as_bytes();
    let mut previous_line: &[u8];
    let mut current_line: &[u8] = replacement_u8;

    let mut sum: i64 = 0;

    let lines = init_lines.chain([replacement_u8]);

    for s in lines {
        previous_line = current_line;
        current_line = next_line;
        next_line = s;

        for (i, c) in current_line.iter().enumerate() {
            if is_gear(*c) {
                let mut gears = get_numbers(i, previous_line);
                let mut curr_gears = get_numbers(i, current_line);
                let mut next_gears = get_numbers(i, next_line);

                gears.append(&mut curr_gears);
                gears.append(&mut next_gears);

                if gears.len() == 2 {
                    sum += gears.iter().product::<i64>();
                }
            }
        }
    }

    sum
}

fn main() {
    let test = solve("./2023/3/test.txt");

    let input = solve("./2023/3/input.txt");

    println!("Test: {:?}", test);
    println!("Input: {:?}", input);
}

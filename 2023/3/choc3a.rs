use std::fs;

fn is_symbol(u: u8) -> bool {
    let c = u as char;
    c != '.' && !c.is_ascii_digit()
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

        let mut is_engine_part: bool = false;
        let mut digit: String = "".to_string();

        for (i, c) in current_line.iter().enumerate() {
            let has_symbol =
                is_symbol(previous_line[i]) || is_symbol(*c) || is_symbol(next_line[i]);
            let is_last_of_series = !c.is_ascii_digit() || i == line_length - 1;

            if c.is_ascii_digit() {
                digit.push(*c as char);
            }

            if is_last_of_series && (is_engine_part || has_symbol) && !digit.is_empty() {
                sum += digit.parse::<i64>().expect("the unexpected");
                digit = "".to_string();
            }

            if is_last_of_series {
                digit = "".to_string();
                is_engine_part = false
            }

            if has_symbol {
                is_engine_part = true
            };
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

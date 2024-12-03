use std::fs;
use regex::Regex;

const FILE_PATH: &str = "./data";

fn main() {
    let corrupted_data = fs::read_to_string(FILE_PATH)
        .expect(&format!("Could not read contents of file \"{}\"", FILE_PATH));

    let multiply_instructions_regex = Regex::new(r"mul\((?<digit_1>\d{1,3}),(?<digit_2>\d{1,3})\)")
        .expect("Regex could not be parsed");

    let total: i32 = multiply_instructions_regex.captures_iter(&corrupted_data).map(
        |captures| {
            let digit_1_string = captures.name("digit_1").unwrap().as_str();
            let digit_2_string = captures.name("digit_2").unwrap().as_str();

            let digit_1 = digit_1_string.parse::<i32>()
                .expect(&format!("Could not parse \"{}\" to integer", digit_1_string));

            let digit_2 = digit_2_string.parse::<i32>()
                .expect(&format!("Could not parse \"{}\" to integer", digit_2_string));

            return digit_1 * digit_2
        }
    ).sum();

    println!("Total: {}", total);
}

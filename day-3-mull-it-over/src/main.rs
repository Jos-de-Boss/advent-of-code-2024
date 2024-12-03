use std::fs;
use regex::Regex;

const FILE_PATH: &str = "./data";
const DO_INSTRUCTION: &str = "do()";
const DONT_INSTRUCTION: &str = "don't()";

fn main() {
    let corrupted_data = fs::read_to_string(FILE_PATH)
        .expect(&format!("Could not read contents of file \"{}\"", FILE_PATH));

    let find_instructions_regex = Regex::new(r"(don't\(\))|(do\(\))|(mul\(\d{1,3},\d{1,3}\))")
        .expect("Find Instructions regex could not be parsed.");

    let instructions = find_instructions_regex.find_iter(&corrupted_data);

    let mut valid_multiply_instructions: Vec<&str> = Vec::new();
    let mut multiply_instructions_enabled = true;
    for instruction in instructions {
        let instruction_string = instruction.as_str();
        match instruction_string {
            DO_INSTRUCTION => multiply_instructions_enabled = true,
            DONT_INSTRUCTION => multiply_instructions_enabled = false,
            _ => {
                if multiply_instructions_enabled {
                    valid_multiply_instructions.push(instruction_string);
                }
            }
        };
    }

    let parse_multiply_instruction_regex = Regex::new(r"^mul\((?<digit_1>\d{1,3}),(?<digit_2>\d{1,3})\)$")
        .expect("Parse multiply instruction regex could not be parsed.");

    let total: i32 = valid_multiply_instructions.into_iter().map(
        |instruction| {
            let captures = parse_multiply_instruction_regex.captures(instruction)
                .expect(&format!("Parse multiply instruction regex did not match \"{}\"", instruction));

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

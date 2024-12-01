use std::fs::File;
use std::io::{BufRead, BufReader};

const FILE_PATH: &str = "./data";
const FILE_COLUMN_SEPARATOR: &str = "   ";

fn read_data_from_file(file_path: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];

    let file = File::open(file_path)
        .expect(format!("Could not open file \"{}\".", &file_path).as_str());
    let file_buffer = BufReader::new(file);

    for (line_index, line) in file_buffer.lines().enumerate() {
        let line_text = line
            .expect(&format!("Could not read line {} of file \"{}\".", line_index + 1, &file_path));

        let columns: Vec<&str> = line_text.split(FILE_COLUMN_SEPARATOR).collect();
        if columns.len() != 2 {
            panic!(
                "Line {} of file \"{}\" has an unexpected amount of columns. Expected 2.",
                line_index + 1,
                file_path,
            )
        }

        let left_value: i32 = columns[0].parse()
            .expect(&format!(
                "Could not parse the left value of line {} of file \"{}\" to an integer.",
                line_index +1,
                file_path
            ));

        let right_value: i32 = columns[1].parse()
            .expect(&format!(
                "Could not parse the right value of line {} of file \"{}\" to an integer.",
                line_index +1,
                file_path
            ));

        left_list.push(left_value);
        right_list.push(right_value);
    }

    (left_list, right_list)
}

fn compute_difference(left_value: i32, right_value: i32) -> i32 {
    if left_value > right_value {
        return left_value - right_value
    }

    if right_value > left_value {
        return right_value - left_value
    }

    0
}

fn main() {
    let (mut left_list, mut right_list) = read_data_from_file(FILE_PATH);

    left_list.sort();
    right_list.sort();

    let mut total_difference = 0;
    for vector_index in 0..left_list.len() {
        let left_value = left_list[vector_index];
        let right_value = right_list[vector_index];

        total_difference += compute_difference(left_value, right_value)
    }

    println!("Total difference: {}", total_difference)
}

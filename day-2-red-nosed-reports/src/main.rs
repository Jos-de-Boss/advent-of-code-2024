use std::fs::File;
use std::io::{BufRead, BufReader};

const FILE_PATH: &str = "./data";
const FILE_COLUMN_SEPARATOR: &str = " ";

fn read_data_from_file<'a>(file_path: &'a str) -> impl Iterator<Item = Vec<i32>> + 'a {
    let file = File::open(file_path)
        .expect(format!("Could not open file \"{}\".", &file_path).as_str());
    let file_buffer = BufReader::new(file);

    file_buffer.lines()
        .map(move |maybe_line| maybe_line.expect(&format!("Could not read the next line of file \"{}\".", file_path)))
        .map(|line| line
            .split(FILE_COLUMN_SEPARATOR)
            .map(|column| column.parse::<i32>().expect(&format!("Could not parse value \"{}\" to integer.", column)))
            .collect()
        )
}

fn validate_increasing_report(previous_level: i32, remaining_levels: &[i32]) -> Result<(), ()> {
    let min_increase = previous_level + 1;
    let max_increase = previous_level + 3;
    let current_level = remaining_levels[0];

    if current_level < min_increase || current_level > max_increase {
        return Err(())
    }

    if remaining_levels.len() > 1 {
        return validate_increasing_report(current_level, &remaining_levels[1..])
    }

    Ok(())
}

fn validate_decreasing_report(previous_level: i32, remaining_levels: &[i32]) -> Result<(), ()> {
    let min_decrease = previous_level - 1;
    let max_decrease = previous_level - 3;
    let current_level = remaining_levels[0];

    if current_level > min_decrease || current_level < max_decrease {
        return Err(())
    }

    if remaining_levels.len() > 1 {
        return validate_decreasing_report(current_level, &remaining_levels[1..])
    }

    Ok(())
}

fn main() {
    let reports = read_data_from_file(FILE_PATH);

    let mut safe_reports = 0;
    let mut unsafe_reports = 0;
    for report_levels in reports {
        if report_levels[0] < report_levels[1] {
            match validate_increasing_report(report_levels[0], &report_levels[1..]) {
                Ok(_) => safe_reports += 1,
                Err(_) => unsafe_reports += 1,
            }
        } else if report_levels[0] > report_levels[1] {
            match validate_decreasing_report(report_levels[0], &report_levels[1..]) {
                Ok(_) => safe_reports += 1,
                Err(_) => unsafe_reports += 1,
            }
        } else {
            unsafe_reports += 1;
        }
    }

    println!("Safe reports: {}", safe_reports);
    println!("Unsafe reports: {}", unsafe_reports);
}

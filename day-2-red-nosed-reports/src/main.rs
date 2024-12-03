use std::fs::File;
use std::io::{BufRead, BufReader};
use day_2_red_nosed_reports::models::report::Report;

const FILE_PATH: &str = "./data";

fn read_data_from_file<'a>(file_path: &'a str) -> impl Iterator<Item = Report> + 'a {
    let file = File::open(file_path)
        .expect(format!("Could not open file \"{}\".", &file_path).as_str());
    let file_buffer = BufReader::new(file);

    file_buffer.lines()
        .map(move |maybe_line| maybe_line.expect(&format!("Could not read the next line of file \"{}\".", file_path)))
        .map(|line| Report::from_string(&line))
}

fn main() {
    let reports = read_data_from_file(FILE_PATH);

    let mut safe_reports = 0;
    let mut unsafe_reports = 0;
    for report in reports {
        if report.is_safe() {
            safe_reports += 1;
            continue
        }

        unsafe_reports += 1;
    }

    println!("Safe reports: {}", safe_reports);
    println!("Unsafe reports: {}", unsafe_reports);
}

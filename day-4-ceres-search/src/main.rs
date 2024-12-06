use std::fs;
use std::iter::Iterator;
use day_4_ceres_search::models::four_by_four_segment::FourByFourSegment;

const FILE_PATH: &str = "./data";
const SEARCH_STRING: &str = "XMAS";
const SEARCH_GRID_SIZE: usize = 4;

fn read_lines_from_file(file_path: &str) -> Vec<String> {
    let Ok(full_file_contents) = fs::read_to_string(file_path) else {
        panic!("Could not read contents of file \"{}\"", file_path);
    };

    full_file_contents
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    let search_string: String = String::from(SEARCH_STRING);
    let search_string_reverted: String = search_string.chars().rev().collect();

    let text_lines = read_lines_from_file(FILE_PATH);
    let line_count = text_lines.len();
    let first_line_length = text_lines[0].len();

    if line_count % SEARCH_GRID_SIZE != 0 && first_line_length % SEARCH_GRID_SIZE != 0  {
        panic!("The elf that wrote this program overlooked something important when first writing the code. In order to preserve his sanity the program now only supports input-grids with a heights and widths that are a multiple of 4.");
    }

    let mut search_matches = 0;
    for row_index in 0..line_count - SEARCH_GRID_SIZE + 1 {
        for column_index in 0..first_line_length - SEARCH_GRID_SIZE + 1 {
            let search_segment = FourByFourSegment::from_lines(vec![
                &text_lines[row_index][column_index..column_index + SEARCH_GRID_SIZE],
                &text_lines[row_index + 1][column_index..column_index + SEARCH_GRID_SIZE],
                &text_lines[row_index + 2][column_index..column_index + SEARCH_GRID_SIZE],
                &text_lines[row_index + 3][column_index..column_index + SEARCH_GRID_SIZE],
            ]);

            if row_index % SEARCH_GRID_SIZE == 0 {
                for row_index in 0..SEARCH_GRID_SIZE {
                    let row = search_segment.get_row(row_index);
                    if row == search_string || row == search_string_reverted {
                        search_matches += 1;
                    }
                }
            }

            if column_index % SEARCH_GRID_SIZE == 0 {
                for column_index in 0..SEARCH_GRID_SIZE {
                    let column = search_segment.get_column(column_index);
                    if column == search_string || column == search_string_reverted {
                        search_matches += 1;
                    }
                }
            }

            let top_left_diagonal = search_segment.get_diagonal_starting_at_top_left();
            if top_left_diagonal == search_string || top_left_diagonal == search_string_reverted {
                search_matches += 1;
            }

            let bottom_left_diagonal = search_segment.get_diagonal_starting_at_bottom_left();
            if bottom_left_diagonal == search_string || bottom_left_diagonal == search_string_reverted {
                search_matches += 1;
            }
        }
    }

    println!("\"XMAS\" Matches: {}", search_matches)
}

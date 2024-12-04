pub struct FourByFourSegment {
    lines: Vec<Vec<char>>,
}

impl FourByFourSegment {
    pub fn from_lines(lines: Vec<&str>) -> Self {
        assert_eq!(lines.len(), 4);
        assert!(lines.iter().all(|line| line.len() == 4));

        let split_lines = lines.into_iter().map(
            |line| line.chars().collect()
        ).collect();

        FourByFourSegment { lines: split_lines }
    }

    pub fn get_row(self: &Self, row_index: usize) -> String {
        self.lines[row_index].iter().collect()
    }

    pub fn get_column(self: &Self, column_index: usize) -> String {
        vec!(
            self.lines[0][column_index],
            self.lines[1][column_index],
            self.lines[2][column_index],
            self.lines[3][column_index],
        ).iter().collect()
    }

    pub fn get_diagonal_starting_at_top_left(self: &Self) -> String {
        vec!(
            self.lines[0][0],
            self.lines[1][1],
            self.lines[2][2],
            self.lines[3][3],
        ).iter().collect()
    }

    pub fn get_diagonal_starting_at_bottom_left(self: &Self) -> String {
        vec!(
            self.lines[3][0],
            self.lines[2][1],
            self.lines[1][2],
            self.lines[0][3],
        ).iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_four_by_four_segment_fixture() -> FourByFourSegment {
        FourByFourSegment::from_lines(vec![
            "ABCD",
            "EFGH",
            "IJKL",
            "MNOP",
        ])
    }

    #[test]
    fn get_row_should_return_correct_strings() {
        let subject_of_testing = get_four_by_four_segment_fixture();
        let expected_first_row = "ABCD";
        let expected_second_row = "EFGH";
        let expected_third_row = "IJKL";
        let expected_fourth_row = "MNOP";

        let actual_first_row = subject_of_testing.get_row(0);
        let actual_second_row = subject_of_testing.get_row(1);
        let actual_third_row = subject_of_testing.get_row(2);
        let actual_fourth_row = subject_of_testing.get_row(3);

        assert_eq!(actual_first_row, expected_first_row);
        assert_eq!(actual_second_row, expected_second_row);
        assert_eq!(actual_third_row, expected_third_row);
        assert_eq!(actual_fourth_row, expected_fourth_row);
    }

    #[test]
    fn get_column_should_return_correct_strings() {
        let subject_of_testing = get_four_by_four_segment_fixture();
        let expected_first_column = "AEIM";
        let expected_second_column = "BFJN";
        let expected_third_column = "CGKO";
        let expected_fourth_column = "DHLP";

        let actual_first_column = subject_of_testing.get_column(0);
        let actual_second_column = subject_of_testing.get_column(1);
        let actual_third_column = subject_of_testing.get_column(2);
        let actual_fourth_column = subject_of_testing.get_column(3);

        assert_eq!(actual_first_column, expected_first_column);
        assert_eq!(actual_second_column, expected_second_column);
        assert_eq!(actual_third_column, expected_third_column);
        assert_eq!(actual_fourth_column, expected_fourth_column);
    }

    #[test]
    fn get_diagonal_starting_at_top_left_should_return_correct_string() {
        let subject_of_testing = get_four_by_four_segment_fixture();
        let expected_string = "AFKP";

        let actual_string = subject_of_testing.get_diagonal_starting_at_top_left();

        assert_eq!(actual_string, expected_string);
    }

    #[test]
    fn get_diagonal_starting_at_bottom_left_should_return_correct_string() {
        let subject_of_testing = get_four_by_four_segment_fixture();
        let expected_string = "MJGD";

        let actual_string = subject_of_testing.get_diagonal_starting_at_bottom_left();

        assert_eq!(actual_string, expected_string);
    }
}

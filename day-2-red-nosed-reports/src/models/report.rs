use std::cmp::{max, min};

const REPORT_STRING_COLUMN_SEPARATOR: &str = " ";
const MIN_LEVEL_DIFFERENCE: i32 = 1;
const MAX_LEVEL_DIFFERENCE: i32 = 3;

#[derive(PartialEq)]
enum Trajectory {
    Increasing,
    Decreasing,
    Constant,
    Inconsistent,
}

impl Trajectory {
    fn from_values(previous: i32, next: i32) -> Self {
        if next > previous {
            return Trajectory::Increasing
        } else if next < previous {
            return Trajectory::Decreasing
        }

        Trajectory::Constant
    }
}

pub struct Report {
    levels: Vec<i32>,
}

impl Report {
    pub fn from_string(string: &str) -> Self {
        let levels = string
            .split(REPORT_STRING_COLUMN_SEPARATOR)
            .map(|column| column.parse::<i32>().expect(&format!("Could not parse value \"{}\" to integer.", column)))
            .collect();

        Report { levels }
    }

    pub fn is_safe(self: &Self) -> bool {
        let trajectory = self.get_trajectory();
        if trajectory == Trajectory::Inconsistent || trajectory == Trajectory::Constant {
            return false
        }

        for current_level_index in 1..self.levels.len() {
            let previous_level = self.levels[current_level_index - 1];
            let current_level = self.levels[current_level_index];

            let difference = max(previous_level, current_level) - min(previous_level, current_level);

            if difference < MIN_LEVEL_DIFFERENCE || difference > MAX_LEVEL_DIFFERENCE {
                return false
            }
        }

        true
    }

    fn get_trajectory(self: &Self) -> Trajectory {
        let initial_trajectory = Trajectory::from_values(self.levels[0], self.levels[1]);

        for current_level_index in 2..self.levels.len() {
            let previous_level = self.levels[current_level_index - 1];
            let current_level = self.levels[current_level_index];

            let current_trajectory = Trajectory::from_values(previous_level, current_level);
            if current_trajectory != initial_trajectory {
                return Trajectory::Inconsistent
            }
        }

        initial_trajectory
    }
}
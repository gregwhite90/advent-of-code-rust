#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 11 };


/*
    Grid will have serial number and power levels.

    GridSearcher will have a dimensions and method for window size that takes a reference to a grid.
*/

mod utils {
    use std::{collections::HashMap, fmt::Display};

    use itertools::iproduct;

    use crate::utils::io_utils;

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    pub struct Point {
        x: usize,
        y: usize,
    }

    impl Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{},{}", self.x, self.y)
        }
    }

    #[derive(Debug, Default)]
    pub struct Grid {
        serial_number: usize,
        power_levels: HashMap<Point, i64>,
        summed_area_table: HashMap<Point, i64>,
    }

    impl Grid {
        pub fn new(serial_number: usize) -> Self {
            Self {
                serial_number,
                power_levels: HashMap::new(),
                summed_area_table: HashMap::new(),
            }
        }

        pub fn parse_input_file(&mut self, filename: &str) {
            self.serial_number = io_utils::file_to_string(filename).parse().unwrap();
        }

        pub fn power_level(&mut self, point: &Point) -> i64 {
            *self.power_levels.entry(*point).or_insert({
                let rack_id = point.x + 10;
                let mut power_level = (rack_id * point.y + self.serial_number) * rack_id;
                power_level /= 100;
                power_level %= 10;
                let power_level: i64 = power_level.try_into().unwrap();
                power_level - 5
            })
        }

        pub fn build_summed_area_table(&mut self) {
            for x in 1..=300 {
                for y in 1..=300 {
                    let pt = Point { x, y };
                    let mut sat_entry = self.power_level(&pt);
                    if x > 1 {
                        sat_entry += self.summed_area_table.get(&Point { x: x - 1, y }).unwrap()
                    }
                    if y > 1 {
                        sat_entry += self.summed_area_table.get(&Point { x, y: y - 1 }).unwrap()
                    }
                    if x > 1 && y > 1 {
                        sat_entry -= self.summed_area_table.get(&Point { x: x - 1, y: y - 1 }).unwrap();
                    }
                    self.summed_area_table.insert(pt, sat_entry);
                }
            }
        }

        pub fn total_power(&self, point: &Point, window_size: usize) -> i64 {
            // The summed-area table methodology does not work for single points
            assert!(window_size > 1);
            let mut total_power = *self.summed_area_table.get(&Point{ x: point.x + window_size - 1, y: point.y + window_size - 1}).unwrap();
            if point.x > 1 {
                total_power -= *self.summed_area_table.get(&Point{ x: point.x - 1, y: point.y + window_size - 1 }).unwrap();
            }
            if point.y > 1 {
                total_power -= self.summed_area_table.get(&Point{ x: point.x + window_size - 1, y: point.y - 1 }).unwrap();
            }
            if point.x > 1 && point.y > 1 {
                total_power += self.summed_area_table.get(&Point{ x: point.x - 1, y: point.y - 1 }).unwrap();
            }
            total_power
        }
    }

    pub fn search_grid(
        dimensions: usize, 
        window_size: usize,
        grid: &mut Grid,
    ) -> (Point, i64) {
        iproduct!(
            1..dimensions - window_size + 1,
            1..dimensions - window_size + 1
        )
            .map(|(col, row)| {
                let total_power: i64 = iproduct!(col..col + window_size, row..row + window_size)
                    .map(|(x, y)| { 
                        grid.power_level(&Point { x, y })
                    })
                    .sum();
                (Point {x: col, y: row}, total_power)
            })
            .max_by_key(|(_point, total_power)| *total_power)
            .unwrap()
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use super::*;

        #[test_case(3, 5, 8, 4; "example_1")]
        #[test_case(122, 79, 57, -5; "example_2")]
        #[test_case(217, 196, 39, 0; "example_3")]
        #[test_case(101, 153, 71, 4; "example_4")]
        fn power_levels_are_correct(x: usize, y: usize, serial_number: usize, power_level: i64) {
            let mut grid = Grid::new(serial_number);
            assert_eq!(grid.power_level(&Point { x, y }), power_level);
        }

        #[test_case(90, 269, 16, 18, 113; "example_1")]
        #[test_case(232, 251, 12, 42, 119; "example_2")]
        fn total_powers_are_correct(x: usize, y: usize, window_size: usize, serial_number: usize, total_power: i64) {
            let mut grid = Grid::new(serial_number);
            grid.build_summed_area_table();
            assert_eq!(grid.total_power(&Point{ x, y }, window_size), total_power);
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::{self, Grid};

    #[derive(Debug)]
    pub struct Soln {
        grid: Grid,    
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.grid.parse_input_file(filename);
            let (point, _total_power) = utils::search_grid(300, 3, &mut self.grid);
            Answer::String(format!("{}", point))
        }
    }

    impl Default for Soln {
        fn default() -> Self {
            Self {
                grid: Grid::new(0),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::String("33,45".to_string()); "example_1")]
        #[test_case(2, Answer::String("21,61".to_string()); "example_2")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::default(),
                example_key,
                answer,
                &DAY,
            );
        }
    }
}

/**
 * Solution to part two uses a summed-area table, as first found suggested on the Reddit
 * thread of solutions.
 */
pub mod part_two {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::{self, Grid};

    #[derive(Debug)]
    pub struct Soln {
        grid: Grid,    
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.grid.parse_input_file(filename);
            let ((top_left, _total_power), window_size) = (1..=300).map(|window_size| {
                (utils::search_grid(300, window_size, &mut self.grid), window_size)
            })
                .max_by_key(|((_point, total_power), _window_size)| *total_power)
                .unwrap();
            Answer::String(format!("{},{}", top_left, window_size))
        }
    }

    impl Default for Soln {
        fn default() -> Self {
            Self {
                grid: Grid::new(0),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::String("90,269,16".to_string()); "example_1")]
        #[test_case(2, Answer::String("232,251,12".to_string()); "example_2")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::default(),
                example_key,
                answer,
                &DAY,
            );
        }
    }
}
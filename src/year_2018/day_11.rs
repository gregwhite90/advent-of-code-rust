#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 11 };

mod utils {
    use std::{collections::HashMap, fmt::Display};

    use itertools::iproduct;
    use lazy_static::lazy_static;
    use regex::Regex;

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
        dimensions: usize,
        window_size: usize,
        serial_number: usize,
        power_levels: HashMap<Point, i64>,
    }

    lazy_static! {
        static ref POWER_LEVEL_RE: Regex = Regex::new(r"\d*(?<hundreds_digit>\d)\d{2}").unwrap();
    }

    impl Grid {
        pub fn new(serial_number: usize) -> Self {
            Self {
                dimensions: 300,
                window_size: 3,
                serial_number,
                power_levels: HashMap::new(),
            }
        }

        pub fn parse_input_file(&mut self, filename: &str) {
            self.serial_number = io_utils::file_to_string(filename).parse().unwrap();
        }

        fn power_level(&mut self, point: &Point) -> i64 {
            *self.power_levels.entry(*point).or_insert({
                let rack_id = point.x + 10;
                let power_level = (rack_id * point.y + self.serial_number) * rack_id;
                let haystack = format!("{}", power_level);
                let captures = POWER_LEVEL_RE.captures(&haystack).unwrap();
                let power_level: i64 = captures.name("hundreds_digit").unwrap().as_str().parse().unwrap();
                power_level - 5
            })
        }

        pub fn largest_total_power_top_left(&mut self) -> Point {
            let (point, _total_power) = iproduct!(
                1..self.dimensions - self.window_size + 1,
                1..self.dimensions - self.window_size + 1
            )
                .map(|(col, row)| {
                    let total_power: i64 = iproduct!(col..col + self.window_size, row..row + self.window_size)
                        .map(|(x, y)| { 
                            self.power_level(&Point { x, y })
                        })
                        .sum();
                    (Point {x: col, y: row}, total_power)
                })
                .max_by_key(|(_point, total_power)| *total_power)
                .unwrap();
            point
        }
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
            assert_eq!(grid.power_level(&Point { x, y}), power_level);
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::Grid;

    #[derive(Debug)]
    pub struct Soln {
        grid: Grid,    
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.grid.parse_input_file(filename);
            Answer::String(format!("{}", self.grid.largest_total_power_top_left()))
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
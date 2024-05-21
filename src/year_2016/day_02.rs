#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 2 };

mod utils {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Direction {
        U,
        R,
        D,
        L,
    }

    impl Direction {
        pub fn from_char(input: char) -> Self {
            match input {
                'U' => Self::U,
                'R' => Self::R,
                'D' => Self::D,
                'L' => Self::L,
                _ => panic!("Unrecognized Direction character."),
            }
        }
    }
}

pub mod part_one {
    use crate::utils::{io_utils, solution::{Answer, Solution}};
    use super::utils::Direction;

    #[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
    struct Point {
        x: u32,
        y: u32,
    }

    impl Point {
        fn to_digit(&self, dimensions: u32) -> u32 {
            dimensions * self.y + self.x + 1
        }

        fn step(&mut self, dimensions: u32, direction: Direction) {
            match direction {
                Direction::U => if self.y != 0 { self.y -= 1; },
                Direction::L => if self.x != 0 { self.x -= 1; },
                Direction::D => if self.y != dimensions - 1 { self.y += 1; },
                Direction::R => if self.x != dimensions - 1 { self.x += 1; },
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        code: u32,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.code)
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            // TODO: make dynamic?
            let dimensions = 3;
            let midpoint = 1;
            let mut position = Point { x: midpoint, y: midpoint };
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    line.chars().for_each(|ch| {
                        position.step(dimensions, Direction::from_char(ch));
                    });
                    self.code = self.code * 10 + position.to_digit(dimensions);
                });
        }   
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(1985); "example_1")]
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

pub mod part_two {
    use crate::utils::{io_utils, solution::{Answer, Solution}};
    use super::utils::Direction;

    fn row_length(row: i32) -> u32 {
        let row_abs: u32 = row.abs().try_into().unwrap();
        5 - 2 * row_abs   
    }

    #[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        fn to_digit(&self) -> u32 {
            let previous_row_lengths: u32 = (-2..self.y).map(|row| row_length(row)).sum();
            let cur_row_length = row_length(self.y);
            let cur_row_idx: u32 = ((cur_row_length as i32 - 1) / 2 + self.x).try_into().unwrap();
            previous_row_lengths + cur_row_idx + 1
        }

        fn to_char(&self) -> char {
            char::from_digit(self.to_digit(), 16).unwrap().to_ascii_uppercase()
        }

        fn manhattan_distance(&self) -> i32 {
            self.x.abs() + self.y.abs()
        }

        fn step(&self, max_manhattan_distance: i32, direction: Direction) -> Point {
            let next_position = match direction {
                Direction::U => Point { x: self.x, y: self.y - 1 },
                Direction::L => Point { x: self.x - 1, y: self.y },
                Direction::D => Point { x: self.x, y: self.y + 1 },
                Direction::R => Point { x: self.x + 1, y: self.y },
            };
            if next_position.manhattan_distance() > max_manhattan_distance {
                self.clone()
            }
            else {
                next_position
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        code: String,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::String(self.code.clone())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            // TODO: make dynamic?
            let max_manhattan_distance = 2;
            let mut position = Point { x: -2, y: 0 };
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    line.chars().for_each(|ch| {
                        position = position.step(max_manhattan_distance, Direction::from_char(ch));
                    });
                    self.code.push(position.to_char());
                });
        }   
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::String(String::from("5DB3")); "example_1")]
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
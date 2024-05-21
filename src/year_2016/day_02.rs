#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 2 };

pub mod part_one {
    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Direction {
        U,
        R,
        D,
        L,
    }

    impl Direction {
        fn from_char(input: char) -> Self {
            match input {
                'U' => Self::U,
                'R' => Self::R,
                'D' => Self::D,
                'L' => Self::L,
                _ => panic!("Unrecognized Direction character."),
            }
        }
    }

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
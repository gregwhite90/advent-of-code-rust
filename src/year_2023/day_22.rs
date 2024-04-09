#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 22 };

pub mod part_one {

    use std::collections::HashMap;

    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    struct Range {
        min: u32,
        max: u32,
    }

    struct Brick {
        id: usize,
        x: Range,
        y: Range,
        z: Range,
        supporting_ids: Vec<usize>,
        supported_by_ids: Vec<usize>,
    }

    impl Brick {
        fn from_str(input: &str, id: usize, re: &Regex) -> Self {
            let captures = re.captures(input).unwrap();
            Self {
                id,
                x: Range { 
                    min: captures.name("min_x").unwrap().as_str().parse().unwrap(), 
                    max: captures.name("max_x").unwrap().as_str().parse().unwrap(),
                },
                y: Range { 
                    min: captures.name("min_y").unwrap().as_str().parse().unwrap(), 
                    max: captures.name("max_y").unwrap().as_str().parse().unwrap(),
                },
                z: Range { 
                    min: captures.name("min_z").unwrap().as_str().parse().unwrap(), 
                    max: captures.name("max_z").unwrap().as_str().parse().unwrap(),
                },
                supporting_ids: vec![],
                supported_by_ids: vec![],
            }
        }
    }

    #[derive(Default)]
    pub struct Soln {
        bricks: HashMap<usize, Brick>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let re = Regex::new(r"(?<min_x>\d+),(?<min_y>\d+),(?<min_z>\d+)\~(?<max_x>\d+),(?<max_y>\d+),(?<max_z>\d+)").unwrap();
            self.bricks = io_utils::file_to_lines(filename)
                .enumerate()
                .map(|(id, line)| (id, Brick::from_str(&line, id, &re)))
                .collect();
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(5); "example_1")]
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
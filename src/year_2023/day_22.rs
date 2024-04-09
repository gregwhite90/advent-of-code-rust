#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 22 };

pub mod part_one {

    use std::collections::HashMap;

    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, Clone, Copy)]
    struct Range {
        min: u32,
        max: u32,
    }

    #[derive(Debug, Clone)]
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
            self.settle_bricks();
            Answer::Usize(self.num_safe_to_disintegrate())
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

        fn settle_bricks(&mut self) {
            /* TODO:
            Go in order of the lowest starting minimum z.
            Then go through the settled bricks in order of the highest maximum z.
            Check for collision. Once a collision is found, we know the settled
            minimum z for the brick. Keep going until we find a brick with a 
            maximum settled z lower than this brick's settled z (impossible
            for it to rest on any more of the bricks).

            If not implemented carefully, could result in lots of searching in O(n)
            or lots of copying of data uneccessarily.
             */
        }

        fn num_safe_to_disintegrate(&self) -> usize {
            self.bricks.values()
                .filter(|brick| {
                    // all bricks it supports have at least one other supporting brick
                    brick.supporting_ids.iter().all(|id| {
                        self.bricks.get(id).unwrap().supported_by_ids.len() > 1
                    })
                })
                .count()                
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(5); "example_1")]
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
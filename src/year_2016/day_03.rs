#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 3 };

pub mod part_one {
    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, Default)]
    pub struct Soln {
        possible_triangles: u32,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.possible_triangles)
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            io_utils::file_to_lines(filename).for_each(|line| {
                let sides: Vec<u32> = line.split_whitespace().map(|num| num.parse().unwrap()).collect();
                let sum: u32 = sides.iter().sum();
                let max: u32 = *sides.iter().max().unwrap();
                if sum - max > max {
                    self.possible_triangles += 1;
                }
            });
        }   
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(0); "example_1")]
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
#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 19 };

pub mod part_one {
    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, Default)]
    pub struct Soln {
        start_num: usize,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::Usize(self.position_getting_all_presents())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.start_num = io_utils::file_to_string(filename).parse().unwrap();
        }

        fn position_getting_all_presents(&self) -> usize {
            let mut num_left = self.start_num;
            let mut start = 1;
            let mut depth = 1;
            while num_left != 1 {
                if num_left % 2 == 1 {
                    start += 2_usize.pow(depth);
                }
                num_left /= 2;
                depth += 1;
            }
            start
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(3); "example_1")]
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

    #[derive(Debug, Default)]
    pub struct Soln {
        start_num: usize,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::Usize(self.position_getting_all_presents())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.start_num = io_utils::file_to_string(filename).parse().unwrap();
        }

        fn position_getting_all_presents(&self) -> usize {
            let mut indices: Vec<usize> = (1..=self.start_num).collect();
            let mut i = 0;
            while indices.len() > 1 {
                indices.remove((i + indices.len() / 2) % indices.len());
                i = if i == indices.len() { 0 } else { i + 1 };
            }
            indices[0]
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(2); "example_1")]
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
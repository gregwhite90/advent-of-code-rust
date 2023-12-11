#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 9 };

mod utils {
    use crate::utils::io_utils;

    pub trait Year2023Day09Solution {
        fn set_sum_of_extrapolated_values(&mut self, value: i32);
        fn extrapolated_value(&self, history: &Vec<i32>) -> i32;
        
        fn parse_input_file(&mut self, filename: &str) {
            self.set_sum_of_extrapolated_values(
                io_utils::file_to_lines(filename)
                    .map(|line| {
                        let history: Vec<i32> = line.split(" ")
                            .map(|val| val.parse().unwrap())
                            .collect();
                        self.extrapolated_value(&history)
                    })
                    .sum()
            );
        }
    }
}

pub mod part_one {
    use itertools::Itertools;

    use crate::utils::solution::{Solution, Answer};

    use super::utils::Year2023Day09Solution;

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        sum_of_extrapolated_values: i32,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::I32(self.sum_of_extrapolated_values)
        }
    }

    impl Year2023Day09Solution for Soln {
        fn set_sum_of_extrapolated_values(&mut self, value: i32) {
            self.sum_of_extrapolated_values = value;
        }

        fn extrapolated_value(&self, history: &Vec<i32>) -> i32 {
            if history.iter().all(|e| *e == 0) { return 0; }
            let next_history: Vec<i32> = history.iter().tuple_windows()
                .map(|(l, r)| {
                    r - l
                })
                .collect();
            history[history.len() - 1] + self.extrapolated_value(&next_history)
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::I32(114); "example_1")]
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
    use itertools::Itertools;

    use crate::utils::solution::{Solution, Answer};

    use super::utils::Year2023Day09Solution;

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        sum_of_extrapolated_values: i32,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::I32(self.sum_of_extrapolated_values)
        }
    }

    impl Year2023Day09Solution for Soln {
        fn set_sum_of_extrapolated_values(&mut self, value: i32) {
            self.sum_of_extrapolated_values = value;   
        }

        fn extrapolated_value(&self, history: &Vec<i32>) -> i32 {
            if history.iter().all(|e| *e == 0) { return 0; }
            let next_history: Vec<i32> = history.iter().tuple_windows()
                .map(|(l, r)| {
                    r - l
                })
                .collect();
            history[0] - self.extrapolated_value(&next_history)
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::I32(2); "example_1")]
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
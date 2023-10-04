#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 6};

mod utils {
    use std::collections::HashMap;
    use crate::utils::{solution::Solution, io_utils};

    // TODO: test
    pub fn parse_input_file(filename: &str) -> Vec<u32> {
        io_utils::file_to_string(filename)
            .split_whitespace()
            .map(|num| num.parse().expect("Each part of the input should be an unsigned integer."))
            .collect()
    }

    pub trait Year2017Day06Solution {
        // Don't have shared/default implementation because needs to access fields on Soln struct
        fn get_banks(&self) -> &Vec<u32>;
        fn get_seen(&self) -> &HashMap<Vec<u32>, u32>;
        fn increment_bank(&mut self, idx: usize);
        fn zero_bank(&mut self, idx: usize);
        fn insert_into_seen(&mut self, banks: Vec<u32>, steps: u32);
    }

    pub fn steps<T>(soln: &mut T) -> u32
    where
        T: Solution + Year2017Day06Solution,
    {
        let mut steps = 0;
        let num_banks = soln.get_banks().len();
        while !soln.get_seen().contains_key(soln.get_banks()) {
            soln.insert_into_seen(soln.get_banks().clone(), steps);
            steps += 1;
            let max = soln.get_banks().iter().max().expect("There should be at least one bank.");
            let pos = soln.get_banks().iter().position(|&blocks| blocks == *max).expect("The max value should be in at least one bank.");
            let blocks = soln.get_banks()[pos];
            soln.zero_bank(pos);
            for offset in 1..(blocks + 1) {
                soln.increment_bank((pos + offset as usize) % num_banks);
            }                
        }
        steps
    }
}

pub mod part_one {
    use std::collections::HashMap;
    use crate::utils::solution::{Solution, Answer};
    use super::utils::{self, Year2017Day06Solution};

    #[derive(Default)]
    pub struct Soln {
        banks: Vec<u32>,
        seen: HashMap<Vec<u32>, u32>,
    }
 
    impl Solution for Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.banks = utils::parse_input_file(filename);
        }

        fn solve(&mut self) -> Answer {
            Answer::U32(utils::steps(self))
        }
    }

    impl Year2017Day06Solution for Soln {
        fn get_banks(&self) -> &Vec<u32> {
            &self.banks
        }

        fn get_seen(&self) -> &HashMap<Vec<u32>, u32> {
            &self.seen
        }

        fn increment_bank(&mut self, idx: usize) {
            self.banks[idx] += 1;
        }

        fn zero_bank(&mut self, idx: usize) {
            self.banks[idx] = 0;
        }

        fn insert_into_seen(&mut self, banks: Vec<u32>, steps: u32) {
            self.seen.insert(banks, steps);
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

pub mod part_two {
    use std::collections::HashMap;
    use crate::utils::solution::{Solution, Answer};
    use super::utils::{self, Year2017Day06Solution};

    #[derive(Default)]
    pub struct Soln {
        banks: Vec<u32>,
        seen: HashMap<Vec<u32>, u32>,
    }
 
    impl Solution for Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.banks = utils::parse_input_file(filename);
        }

        fn solve(&mut self) -> Answer {
            Answer::U32(
                utils::steps(self) - self.seen.get(&self.banks).expect("Current banks should have been seen previously.")
            )
        }
    }

    impl Year2017Day06Solution for Soln {
        fn get_banks(&self) -> &Vec<u32> {
            &self.banks
        }

        fn get_seen(&self) -> &HashMap<Vec<u32>, u32> {
            &self.seen
        }

        fn increment_bank(&mut self, idx: usize) {
            self.banks[idx] += 1;
        }

        fn zero_bank(&mut self, idx: usize) {
            self.banks[idx] = 0;
        }

        fn insert_into_seen(&mut self, banks: Vec<u32>, steps: u32) {
            self.seen.insert(banks, steps);
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(4); "example_1")]
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
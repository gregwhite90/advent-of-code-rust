#[cfg(test)]
const YEAR: u32 = 2017;
#[cfg(test)]
const DAY: u8 = 6;

pub mod utils { // TODO: make not pub?
    use std::{fs, collections::HashMap};
    use crate::utils::utils::Solution;

    pub fn parse_input_file(filename: &str) -> Vec<u32> {
        fs::read_to_string(filename)
            .expect("Should be able to read the file to a string.")
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
    pub use either::*;
    use crate::utils::utils::Solution;
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

        fn solve(&mut self) -> Either<i32, &str> {
            Left(utils::steps(self).try_into().expect("Steps should be convertible to signed integer."))
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
        use std::collections::HashMap;
        use either::*;
        use crate::utils::test_utils;
        use super::*;
        use super::super::{YEAR, DAY};

        #[test]
        fn examples_are_correct() {
            test_utils::check_example_cases(
                &mut Soln::default(),
                &HashMap::from([
                    (1u8, Left(5)),
                ]),
                YEAR,
                DAY,
            );
        }
    }    
}

pub mod part_two {
    use std::collections::HashMap;
    pub use either::*;
    use crate::utils::utils::Solution;
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

        fn solve(&mut self) -> Either<i32, &str> {
            Left(
                (utils::steps(self) - self.seen.get(&self.banks).expect("Current banks should have been seen previously."))
                    .try_into()
                    .expect("Steps should be convertable to an unsigned integer.")
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
        use std::collections::HashMap;
        use either::*;
        use crate::utils::test_utils;
        use super::*;
        use super::super::{YEAR, DAY};

        #[test]
        fn examples_are_correct() {
            test_utils::check_example_cases(
                &mut Soln::default(),
                &HashMap::from([
                    (1u8, Left(4)),
                ]),
                YEAR,
                DAY,
            );
        }
    }    
}
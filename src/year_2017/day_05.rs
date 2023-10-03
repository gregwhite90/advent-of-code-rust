#[cfg(test)]
const YEAR: u32 = 2017;
#[cfg(test)]
const DAY: u8 = 5;

pub mod utils {
    use std::fs;
    use crate::utils::utils::Solution;

    pub trait Year2017Day05Solution {
        #[allow(unused_variables)] // The default (part one) implementation does not use the instruction argument
        fn offset_change(instruction: i32) -> i32 {
            1
        }
        // Don't have shared/default implementation because needs to access fields on Soln struct
        fn get_instructions(&self) -> &Vec<i32>;
        fn offset_instruction(&mut self, idx: usize, offset: i32);
    }

    pub fn parse_input_file(filename: &str) -> Vec<i32> {
        fs::read_to_string(filename)
            .expect("Should be able to read file to string.")
            .lines()
            .map(|line| line.parse::<i32>().expect("Each line should be an integer."))
            .collect()
    }

    pub fn solve<T>(soln: &mut T) -> i32
    where
        T: Solution + Year2017Day05Solution,
    {
        let mut idx: i32 = 0;
        let mut steps: u32 = 0;
        while (idx as usize) < soln.get_instructions().len() && idx >= 0 {
            steps += 1;
            let jump = soln.get_instructions()[idx as usize];
            soln.offset_instruction(
                idx as usize,
                T::offset_change(soln.get_instructions()[idx as usize])
            );
            idx += jump;
        }
        steps as i32
    }
}

pub mod part_one {
    pub use either::*;
    use crate::utils::utils::Solution;
    use super::utils::{self, Year2017Day05Solution};

    #[derive(Default)]
    pub struct Soln {
        instructions: Vec<i32>,
    }
 
    impl Solution for Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.instructions = utils::parse_input_file(filename);
        }

        fn solve(&mut self) -> Either<i32, &str> {
            Left(utils::solve(self))
        }
    }

    impl Year2017Day05Solution for Soln {
        fn get_instructions(&self) -> &Vec<i32> {
            &self.instructions
        }

        fn offset_instruction(&mut self, idx: usize, offset: i32) {
            self.instructions[idx] += offset;
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
    pub use either::*;
    use crate::utils::utils::Solution;
    use super::utils::{self, Year2017Day05Solution};

    #[derive(Default)]
    pub struct Soln {
        instructions: Vec<i32>,
    }
 
    impl Solution for Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.instructions = utils::parse_input_file(filename);
        }

        fn solve(&mut self) -> Either<i32, &str> {
            Left(utils::solve(self))
        }
    }

    impl Year2017Day05Solution for Soln {
        fn offset_change(instruction: i32) -> i32 {
            if instruction >= 3 {
                -1
            } else {
                1
            }
        }

        fn get_instructions(&self) -> &Vec<i32> {
            &self.instructions
        }

        fn offset_instruction(&mut self, idx: usize, offset: i32) {
            self.instructions[idx] += offset;
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
                    (1u8, Left(10)),
                ]),
                YEAR,
                DAY,
            );
        }
    }    
}
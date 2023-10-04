#[cfg(test)]
const YEAR: u32 = 2017;
#[cfg(test)]
const DAY: u8 = 5;

pub mod utils {
    use crate::utils::{solution::Solution, io_utils};

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
        io_utils::file_to_lines(filename)
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

    // TODO: add tests
}

pub mod part_one {
    pub use either::*;
    use crate::utils::solution::Solution;
    use super::utils::{self, Year2017Day05Solution};

    #[derive(Default)]
    pub struct Soln {
        instructions: Vec<i32>,
    }
 
    impl Solution for Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.instructions = utils::parse_input_file(filename);
        }

        fn solve(&mut self) -> Either<i32, String> {
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
        use test_case::test_case;
        use either::*;
        use crate::utils::test_utils;
        use super::*;
        use super::super::{YEAR, DAY};

        #[test_case(1, Left(5); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Either<i32, String>) {
            test_utils::check_example_case(
                &mut Soln::default(),
                example_key,
                answer,
                YEAR,
                DAY,
            );
        }
    }    
}

pub mod part_two {
    pub use either::*;
    use crate::utils::solution::Solution;
    use super::utils::{self, Year2017Day05Solution};

    #[derive(Default)]
    pub struct Soln {
        instructions: Vec<i32>,
    }
 
    impl Solution for Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.instructions = utils::parse_input_file(filename);
        }

        fn solve(&mut self) -> Either<i32, String> {
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
        use test_case::test_case;
        use either::*;
        use crate::utils::test_utils;
        use super::*;
        use super::super::{YEAR, DAY};

        #[test_case(1, Left(10); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Either<i32, String>) {
            test_utils::check_example_case(
                &mut Soln::default(),
                example_key,
                answer,
                YEAR,
                DAY,
            );
        }
    }    
}
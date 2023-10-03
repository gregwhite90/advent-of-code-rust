#[cfg(test)]
const YEAR: u32 = 2017;
#[cfg(test)]
const DAY: u8 = 5;

pub mod part_one {
    use std::fs;
    pub use either::*;
    use crate::utils::utils::Solution;

    #[derive(Default)]
    pub struct Soln {
        instructions: Vec<i32>,
    }
 
    impl Solution for Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.instructions = fs::read_to_string(filename)
                .expect("Should be able to read file to string.")
                .lines()
                .map(|line| line.parse::<i32>().expect("Each line should be an integer."))
                .collect();
        }

        fn solve(&mut self) -> Either<i32, &str> {
            let mut idx: i32 = 0;
            let mut steps: u32 = 0;
            while (idx as usize) < self.instructions.len() && idx >= 0 {
                steps += 1;
                let jump = self.instructions[idx as usize];
                self.instructions[idx as usize] += 1;
                idx += jump;
            }
            Left(
                steps as i32
            )
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
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
                self.instructions[idx as usize] += 1; // TODO: implement += everywhere
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
        use crate::utils::utils::{InputFileType, input_filename};
        use super::*;
        use super::super::{YEAR, DAY};

        // TODO: refactor
        #[test]
        fn examples_are_correct() {
            let cases = HashMap::from([
                (1u8, 5),
            ]);
            for (&example_key, &answer) in &cases {
                let mut soln = Soln::default();
                soln.parse_input_file(&input_filename(YEAR, DAY, InputFileType::Example(example_key)));
                assert_eq!(
                    soln.solve().expect_left("Solution should be an integer."),
                    answer
                );
            }
        }
    }    
}
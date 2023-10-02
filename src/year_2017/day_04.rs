#[cfg(test)]
const YEAR: u32 = 2017;
#[cfg(test)]
const DAY: u8 = 4;

pub mod part_one {
    pub use either::*;
    use std::fs;
    use std::collections::HashSet;
    use crate::utils::utils::Solution;

    #[derive(Default)]
    pub struct Soln {
        num_valid: u32,
    }
 
    impl Solution for Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.num_valid = fs::read_to_string(filename)
                .expect("Should be able to read file to string.")
                .lines()
                .map(|line| self.is_valid(line))
                .filter(|valid| *valid)
                .count() as u32
        }

        fn solve(&mut self) -> Either<i32, &str> {
            Left(
                self.num_valid as i32
            )
        }
    }

    impl Soln {
        fn is_valid(&self, passphrase: &str) -> bool {
            let mut words = HashSet::new();
            for word in passphrase.split_whitespace() {
                if words.contains(word) {
                    return false;
                }
                words.insert(word);
            }
            true
        }
    }

    #[cfg(test)]
    mod tests {
        use std::collections::HashMap;
        use crate::utils::utils::{InputFileType, input_filename};
        use super::*;
        use super::super::{YEAR, DAY};

        #[test]
        fn examples_are_correct() {
            let cases = HashMap::from([
                (1u8, 2),
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
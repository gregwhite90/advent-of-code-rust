#[cfg(test)]
const YEAR: u32 = 2017;
#[cfg(test)]
const DAY: u8 = 4;

pub mod utils {
    use std::fs;
    use crate::utils::utils::Solution;

    pub trait Year2017Day04Solution {
        fn is_valid(&self, passphrase: &str) -> bool;
    }

    pub fn parse_input_file<T>(soln: &mut T, filename: &str) -> u32
    where
        T: Solution + Year2017Day04Solution,
    {
        fs::read_to_string(filename)
                .expect("Should be able to read file to string.")
                .lines()
                .map(|line| soln.is_valid(line))
                .filter(|valid| *valid)
                .count() as u32
    }
}

pub mod part_one {
    pub use either::*;
    use std::collections::HashSet;
    use crate::utils::utils::Solution;
    use super::utils::{self, Year2017Day04Solution};

    #[derive(Default)]
    pub struct Soln {
        num_valid: u32,
    }
 
    impl Solution for Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.num_valid = utils::parse_input_file(self, filename);
        }

        fn solve(&mut self) -> Either<i32, &str> {
            Left(
                self.num_valid as i32
            )
        }
    }

    impl Year2017Day04Solution for Soln {
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

pub mod part_two {
    pub use either::*;
    use std::collections::{HashSet, BTreeMap};
    use unicode_segmentation::UnicodeSegmentation;
    use crate::utils::utils::Solution;
    use super::utils::{self, Year2017Day04Solution};

    #[derive(Default)]
    pub struct Soln {
        num_valid: u32,
    }
 
    impl Solution for Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.num_valid = utils::parse_input_file(self, filename);
        }

        fn solve(&mut self) -> Either<i32, &str> {
            Left(
                self.num_valid as i32
            )
        }
    }

    impl Year2017Day04Solution for Soln {
        fn is_valid(&self, passphrase: &str) -> bool {
            // Uses BTreeMap instead of HashMap because BTreeMap implements Hash
            // and HashMap can not.
            let mut grapheme_counts: HashSet<BTreeMap<&str, u32>> = HashSet::new();
            for word in passphrase.split_whitespace() {
                let mut word_grapheme_count: BTreeMap<&str, u32> = BTreeMap::new();
                for grapheme in word.graphemes(true) {
                    let count = word_grapheme_count.entry(grapheme).or_insert(0);
                    *count += 1;
                }
                if grapheme_counts.contains(&word_grapheme_count) {
                    return false;
                }
                grapheme_counts.insert(word_grapheme_count);
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
                (2u8, 3),
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
#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 4};

mod utils {
    use crate::utils::{solution::Solution, io_utils};

    pub trait Year2017Day04Solution {
        fn is_valid(&self, passphrase: &str) -> bool;
    }

    pub fn parse_input_file<T>(soln: &mut T, filename: &str) -> u32
    where
        T: Solution + Year2017Day04Solution,
    {
        io_utils::file_to_lines(filename)
            .map(|line| soln.is_valid(&line))
            .filter(|valid| *valid)
            .count() as u32
    }
}

pub mod part_one {
    use std::collections::HashSet;
    use crate::utils::solution::{Solution, Answer};
    use super::utils::{self, Year2017Day04Solution};

    #[derive(Default)]
    pub struct Soln {
        num_valid: u32,
    }
 
    impl Solution for Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.num_valid = utils::parse_input_file(self, filename);
        }

        fn solve(&mut self) -> Answer {
            Answer::U32(self.num_valid)
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
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(2); "example_1")]
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
    use std::collections::{HashSet, BTreeMap};
    use unicode_segmentation::UnicodeSegmentation;
    use crate::utils::solution::{Solution, Answer};
    use super::utils::{self, Year2017Day04Solution};

    #[derive(Default)]
    pub struct Soln {
        num_valid: u32,
    }
 
    impl Solution for Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.num_valid = utils::parse_input_file(self, filename);
        }

        fn solve(&mut self) -> Answer {
            Answer::U32(self.num_valid)
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
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(2, Answer::U32(3); "example_2")]
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
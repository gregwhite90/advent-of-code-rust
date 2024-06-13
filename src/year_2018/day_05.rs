#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 5 };

mod utils {
    use std::collections::HashSet;

    use itertools::Itertools;

    pub fn react(mut polymer: String) -> String {
        loop {
            // find indices
            let mut indices_to_remove: HashSet<usize> = polymer.chars().tuple_windows().enumerate().filter(|(_idx, (l, r))| {
                (*l as u8).abs_diff(*r as u8) == 32
            })
                .map(|(idx, (_l, _r))| idx)
                .collect();

            // Ignore overlapped matches
            let overlapping_indices: HashSet<usize> = indices_to_remove.clone().into_iter()
                .filter(|idx| *idx != 0 && indices_to_remove.contains(&(*idx - 1)))
                .collect();
            indices_to_remove.retain(|idx| !overlapping_indices.contains(idx));

            if indices_to_remove.is_empty() {
                return polymer;
            }
            // remove indices
            let mut removed: HashSet<usize> = HashSet::new();
            polymer = polymer.char_indices().filter(|(idx, _ch)| {
                let remove = indices_to_remove.contains(idx) && !removed.contains(idx)
                    || *idx != 0 && indices_to_remove.contains(&(*idx - 1));
                removed.insert(*idx);                    
                !remove
            })
                .map(|(_idx, ch)| ch)
                .collect()
        }
    }
}

pub mod part_one {
    use crate::utils::{io_utils, solution::{Answer, Solution}};
    use super::utils;

    #[derive(Debug, Default)]
    pub struct Soln {
        polymer: String,    
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::Usize(utils::react(self.polymer.clone()).len())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.polymer = io_utils::file_to_string(filename);
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(10); "example_1")]
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
    use std::cmp::min;

    use crate::utils::{io_utils, solution::{Answer, Solution}};
    use super::utils;

    #[derive(Debug, Default)]
    pub struct Soln {
        polymer: String,    
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::Usize(self.min_length())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.polymer = io_utils::file_to_string(filename);
        }

        fn min_length(&self) -> usize {
            let mut min_length = usize::MAX;
            for ch in 'a'..='z' {
                let mut polymer = self.polymer.clone();
                polymer.retain(|c| c != ch && c != ch.to_ascii_uppercase());
                polymer = utils::react(polymer);
                min_length = min(min_length, polymer.len());
            }
            min_length
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(4); "example_1")]
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
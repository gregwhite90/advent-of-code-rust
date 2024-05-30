#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 14 };

pub mod part_one {
    use std::collections::{BinaryHeap, HashMap, HashSet};

    use md5::{Md5, Digest};
    use fancy_regex::Regex;
    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, Default)]
    pub struct Soln {
        salt: String,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::Usize(self.find_key_index(64))
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.salt = io_utils::file_to_string(filename);
        }   

        fn find_key_index(&self, n: usize) -> usize {
            let potential_key_re = Regex::new(r"(\h)\1{2}").unwrap();
            let matched_key_re = Regex::new(r"(\h)\1{4}").unwrap();
            let mut index: usize = 0;
            /* TODO:
            Decide if we need to be more thoughtful about the "order" in which we consider the
            key indices. This counts the 64th key found as the 64th key confirmed. It is still
            possible that other indices before the index of the 64th key found will also be keys.

            If we need to be more careful about this, will have to do something like:
            Once we reach n keys, stop adding new potential ones, just check another 1000 (or fewer,
            we could look at all the values in the potential keys hashsets to find the maximum one
            that is less than our current nth index and add 1000 to that to get the stopping index).
            Then pop off the binary heap until it has len == n and peek the result.
             */
            let mut potential_keys: HashMap<char, HashSet<usize>> = HashMap::new();
            let mut keys: BinaryHeap<usize> = BinaryHeap::new();
            while keys.len() < n {
                let mut hasher = Md5::new();
                hasher.update(format!("{}{}", self.salt, index).as_bytes());
                let hex = format!("{:x}", hasher.finalize());
                if !potential_keys.is_empty() {
                    for res in matched_key_re.find_iter(&hex) {
                        if let Ok(m) = res {
                            let ch = m.as_str().chars().next().unwrap();
                            if let Some(indices) = potential_keys.get_mut(&ch) {
                                indices.iter().filter(|idx| **idx + 1000 >= index).for_each(|idx| keys.push(*idx));
                                indices.clear();
                            }
                        }
                    }
                }
                if let Some(m) = potential_key_re.find(&hex).unwrap() {
                    potential_keys.entry(m.as_str().chars().next().unwrap()).or_default().insert(index);
                }
                index += 1;
            }
            *keys.peek().unwrap()
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(22728); "example_1")]
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
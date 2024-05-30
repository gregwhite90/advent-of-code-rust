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

pub mod part_two {
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
            let mut potential_keys: HashMap<char, HashSet<usize>> = HashMap::new();
            let mut keys: BinaryHeap<usize> = BinaryHeap::new();
            while keys.len() < n {
                let hex = hash(&self.salt, index, 2016);
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
            // TODO: refactor to DRY
            /* 
            We have found `n` keys, but it is still possible that other indices before the
            index of the `n`th key found will also be keys.

            We stop adding new potential ones, and just check another 1000 for if they
            verify any formerly found potential keys. We could further optimize by checking fewer:
            we could look at all the values in the potential keys hashsets to find the maximum one
            that is less than our current nth index and add 1000 to that to get the stopping index.

            Then we pop off the binary heap until it has len == n and peek the result.
             */
            for _ in 0..1000 {
                let hex = hash(&self.salt, index, 2016);
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
                index += 1;
            }
            while keys.len() > n {
                keys.pop();
            }
            *keys.peek().unwrap()
        }
    }

    fn hash(salt: &str, index: usize, repeats: usize) -> String {
        let mut hex = format!("{:x}", Md5::digest(format!("{}{}", salt, index).as_bytes()));
        for _ in 0..repeats {
            hex = format!("{:x}", Md5::digest(hex.as_bytes()));
        }
        hex
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(0, "577571be4de9dcce85a041ba0410f29f"; "0_repeats")]
        #[test_case(1, "eec80a0c92dc8a0777c619d9bb51e910"; "1_repeats")]
        #[test_case(2, "16062ce768787384c81fe17a7a60c7e3"; "2_repeats")]
        #[test_case(2016, "a107ff634856bb300138cac6568c0f24"; "2016_repeats")]
        fn hashes_are_correct(repeats: usize, answer: &str) {
            assert_eq!(hash("abc", 0, repeats), answer.to_string());
        }

        #[test_case(1, Answer::Usize(22551); "example_1")]
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
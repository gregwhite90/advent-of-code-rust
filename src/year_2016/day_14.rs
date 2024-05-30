#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 14 };

mod utils {
    pub const N: usize = 64;
    const LOOK_AHEAD: usize = 1000;

    use std::collections::{BinaryHeap, HashMap, HashSet};

    use md5::{Md5, Digest};
    use fancy_regex::Regex;
    use crate::utils::io_utils;

    #[derive(Debug, Default)]
    pub struct KeyGenerator {
        repeats: usize,
        salt: String,
    }

    impl KeyGenerator {
        pub fn with_repeats(repeats: usize) -> Self {
            Self {
                repeats,
                salt: String::new(),
            }
        }

        pub fn parse_input_file(&mut self, filename: &str) {
            self.salt = io_utils::file_to_string(filename);
        }   

        pub fn find_key_index(&self, n: usize) -> usize {
            let potential_key_re = Regex::new(r"(\h)\1{2}").unwrap();
            let matched_key_re = Regex::new(r"(\h)\1{4}").unwrap();
            let mut index: usize = 0;
            let mut potential_keys: HashMap<char, HashSet<usize>> = HashMap::new();
            let mut keys: BinaryHeap<usize> = BinaryHeap::new();
            while keys.len() < n {
                let hex = hash(&self.salt, index, self.repeats);
                verify_potential_keys(index, &hex, &matched_key_re, &mut potential_keys, &mut keys);
                if let Some(m) = potential_key_re.find(&hex).unwrap() {
                    potential_keys.entry(m.as_str().chars().next().unwrap()).or_default().insert(index);
                }
                index += 1;
            }
            /* 
            We have found `n` keys, but it is still possible that other indices before the
            index of the `n`th key found will also be keys.

            We stop adding new potential ones, and just check another 1000 for if they
            verify any formerly found potential keys. We could further optimize by checking fewer:
            we could look at all the values in the potential keys hashsets to find the maximum one
            that is less than our current nth index and add 1000 to that to get the stopping index.

            Then we pop off the binary heap until it has len == n and peek the result.
             */
            for i in index..(index + LOOK_AHEAD) {
                let hex = hash(&self.salt, i, self.repeats);
                verify_potential_keys(i, &hex, &matched_key_re, &mut potential_keys, &mut keys);
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

    fn verify_potential_keys(
        index: usize,
        hex: &str,
        matched_key_re: &Regex, 
        potential_keys: &mut HashMap<char, HashSet<usize>>,
        keys: &mut BinaryHeap<usize>,
    ) {
        if !potential_keys.is_empty() {
            for res in matched_key_re.find_iter(&hex) {
                if let Ok(m) = res {
                    let ch = m.as_str().chars().next().unwrap();
                    if let Some(indices) = potential_keys.get_mut(&ch) {
                        indices.iter().filter(|idx| **idx + LOOK_AHEAD >= index).for_each(|idx| keys.push(*idx));
                        indices.clear();
                    }
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use super::*;

        #[test_case(0, "577571be4de9dcce85a041ba0410f29f"; "0_repeats")]
        #[test_case(1, "eec80a0c92dc8a0777c619d9bb51e910"; "1_repeats")]
        #[test_case(2, "16062ce768787384c81fe17a7a60c7e3"; "2_repeats")]
        #[test_case(2016, "a107ff634856bb300138cac6568c0f24"; "2016_repeats")]
        fn hashes_are_correct(repeats: usize, answer: &str) {
            assert_eq!(hash("abc", 0, repeats), answer.to_string());
        }
    }    
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::{KeyGenerator, N};

    #[derive(Debug, Default)]
    pub struct Soln {
        key_generator: KeyGenerator,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.key_generator.parse_input_file(filename);
            Answer::Usize(self.key_generator.find_key_index(N))
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
    use crate::utils::solution::{Answer, Solution};

    use super::utils::{KeyGenerator, N};

    #[derive(Debug)]
    pub struct Soln {
        key_generator: KeyGenerator,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self {
                key_generator: KeyGenerator::with_repeats(2016),
            }
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.key_generator.parse_input_file(filename);
            Answer::Usize(self.key_generator.find_key_index(N))
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

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
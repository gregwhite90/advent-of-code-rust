use crate::utils::Day;
const DAY: Day = crate::utils::Day { year: 2015, day: 16 };

mod utils {
    use std::{cmp, collections::HashMap};

    use itertools::Itertools;
    use ndarray::{Array, Array1, Array2};
    use regex::Regex;

    use crate::utils::io_utils;
    use super::DAY;

    #[derive(Debug)]
    struct Letter {
        results: HashMap<String, usize>,
    }

    impl Letter {
        fn new(filename: &str) -> Self {
            let result_re = Regex::new(r"(?<feature_name>\w+): (?<feature_value>\d+)").unwrap();
            Self {
                results: io_utils::file_to_lines(filename).map(|line| {
                    let caps = result_re.captures(&line).unwrap();
                    let feature_name = caps.name("feature_name").unwrap().as_str();
                    let feature_value: usize = caps.name("feature_value").unwrap().as_str().parse().unwrap();
                    (feature_name.to_string(), feature_value)
                }).collect::<HashMap<String, usize>>(),
            }
        }

        pub fn disqualifies(&self, feature_name: &str, feature_value: usize) -> bool {
            *self.results.get(feature_name).unwrap() != feature_value
        }
    }

    #[derive(Debug)]
    pub struct LetterChecker {
        letter: Letter,
    }

    impl Default for LetterChecker {
        fn default() -> Self {
            Self {
                letter: Letter::new(&io_utils::filename(&DAY, "mfcsam_results.txt"))
            }
        }
    }

    impl LetterChecker {
        pub fn valid_id(&self, filename: &str) -> usize {
            let sue_re = Regex::new(r"Sue (?<id>\d+): (?<features>.+)").unwrap();
            let feature_re = Regex::new(r"(?<feature_name>\w+): (?<feature_value>\d+)").unwrap();
            let valid_ids: Vec<usize> = io_utils::file_to_lines(filename).filter_map(|line| {
                let caps = sue_re.captures(&line).unwrap();
                let id: usize = caps.name("id").unwrap().as_str().parse().unwrap();
                for feature in caps.name("features").unwrap().as_str().split(", ") {
                    let caps = feature_re.captures(feature).unwrap();
                    let feature_name = caps.name("feature_name").unwrap().as_str();
                    let feature_value = caps.name("feature_value").unwrap().as_str().parse().unwrap();
                    if self.letter.disqualifies(feature_name, feature_value) {
                        return None;
                    }
                }
                return Some(id)
            }).collect();
            assert_eq!(valid_ids.len(), 1);
            valid_ids[0]
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::LetterChecker;

    #[derive(Debug, Default)]
    pub struct Soln {
        letter_checker: LetterChecker,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            Answer::Usize(self.letter_checker.valid_id(filename))
        }
    }
}
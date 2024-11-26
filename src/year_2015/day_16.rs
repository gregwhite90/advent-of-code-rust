use crate::utils::Day;
const DAY: Day = crate::utils::Day { year: 2015, day: 16 };

mod utils {
    use std::collections::HashMap;

    use regex::Regex;

    use crate::utils::io_utils;
    use super::DAY;

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    pub enum RangeType {
        Greater,
        Less,
    }

    #[derive(Debug)]
    struct Letter {
        results: HashMap<String, usize>,
        feature_ranges: HashMap<String, RangeType>,
    }

    impl Letter {
        fn new(filename: &str, feature_ranges: HashMap<String, RangeType>) -> Self {
            let result_re = Regex::new(r"(?<feature_name>\w+): (?<feature_value>\d+)").unwrap();
            Self {
                results: io_utils::file_to_lines(filename).map(|line| {
                    let caps = result_re.captures(&line).unwrap();
                    let feature_name = caps.name("feature_name").unwrap().as_str();
                    let feature_value: usize = caps.name("feature_value").unwrap().as_str().parse().unwrap();
                    (feature_name.to_string(), feature_value)
                }).collect::<HashMap<String, usize>>(),
                feature_ranges,
            }
        }

        pub fn disqualifies(&self, feature_name: &str, feature_value: usize) -> bool {
            match self.feature_ranges.get(feature_name) {
                None => *self.results.get(feature_name).unwrap() != feature_value,
                Some(RangeType::Greater) => feature_value <= *self.results.get(feature_name).unwrap(),
                Some(RangeType::Less) => feature_value >= *self.results.get(feature_name).unwrap(),
            }
        }
    }

    #[derive(Debug)]
    pub struct LetterChecker {
        letter: Letter,
    }

    impl LetterChecker {
        pub fn new(feature_ranges: HashMap<String, RangeType>) -> Self {
            Self {
                letter: Letter::new(
                    &io_utils::filename(&DAY, "mfcsam_results.txt"),
                    feature_ranges,
                )
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
    use std::collections::HashMap;

    use crate::utils::solution::{Answer, Solution};

    use super::utils::LetterChecker;

    #[derive(Debug)]
    pub struct Soln {
        letter_checker: LetterChecker,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self {
                letter_checker: LetterChecker::new(HashMap::new())
            }
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            Answer::Usize(self.letter_checker.valid_id(filename))
        }
    }
}

pub mod part_two {
    use std::collections::HashMap;

    use crate::utils::solution::{Answer, Solution};

    use super::utils::{LetterChecker, RangeType};

    #[derive(Debug)]
    pub struct Soln {
        letter_checker: LetterChecker,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self {
                letter_checker: LetterChecker::new(HashMap::from([
                    ("cats".to_string(), RangeType::Greater),
                    ("trees".to_string(), RangeType::Greater),
                    ("pomeranians".to_string(), RangeType::Less),
                    ("goldfish".to_string(), RangeType::Less),
                ]))
            }
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            Answer::Usize(self.letter_checker.valid_id(filename))
        }
    }
}
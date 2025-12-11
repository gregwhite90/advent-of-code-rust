#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2025, day: 10 };

mod utils {
    use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}};

    use itertools::Itertools;
    use lazy_static::lazy_static;
    use regex::Regex;

    use crate::utils::io_utils;

    lazy_static! {
        pub static ref MACHINE_RE: Regex = Regex::new(r"\[(?<indicator_lights>[\.\#]+)\] (?<buttons>[\( \)\,\d]+) \{(?<joltages>[\d\,]+)\}").unwrap();
        pub static ref BUTTONS_RE: Regex = Regex::new(r"(?:\()([\d\,]+)(?:\))").unwrap();
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    struct Machine {
        on_indicator_lights: String, // TODO adjust
        buttons: Vec<String>, // TODO: adjust
        joltages: String,
    }

    impl Machine {
        pub fn from_str(input: &str) -> Self {
            let caps = MACHINE_RE.captures(input).unwrap();
            let buttons = BUTTONS_RE.captures_iter(caps.name("buttons").unwrap().as_str())
                .map(|cap| cap.get(1).unwrap().as_str().to_string())
                .collect();
            Self {
                on_indicator_lights: caps.name("indicator_lights").unwrap().as_str().to_string(),
                buttons,
                joltages: caps.name("joltages").unwrap().as_str().to_string(),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use super::*;

        #[test_case(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
            Machine {
                on_indicator_lights: ".##.".to_string(),
                buttons: vec![
                    "3".to_string(),
                    "1,3".to_string(),
                    "2".to_string(),
                    "2,3".to_string(),
                    "0,2".to_string(),
                    "0,1".to_string(),
                ],
                joltages: "3,5,4,7".to_string(),
            };
            "example_1"
        )]
        fn machine_from_str_is_correct(input: &str, expected: Machine) {
            assert_eq!(
                Machine::from_str(input),
                expected,
            );
        }
    }    
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    #[derive(Debug, Default)]
    pub struct Soln {}

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            unimplemented!();
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U64(7); "example_1")]
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
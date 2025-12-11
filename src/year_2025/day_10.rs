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

    // TODO: this may be overkill
    #[derive(Debug, Default, PartialEq, Eq)]
    struct Button {
        mask: u16,
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    struct Machine {
        on_indicator_lights: u16, // TODO adjust
        buttons: Vec<Button>, // TODO: decide if Vec is right?
        joltages: Vec<u64>,
    }

    impl Machine {
        pub fn from_str(input: &str) -> Self {
            let caps = MACHINE_RE.captures(input).unwrap();
            let on_indicator_lights = caps.name("indicator_lights").unwrap().as_str().to_string();
            let buttons = BUTTONS_RE.captures_iter(caps.name("buttons").unwrap().as_str())
                .map(|cap| {
                    let idxs: HashSet<usize> = cap.get(1)
                        .unwrap()
                        .as_str()
                        .split(',')
                        .map(|num| num.parse().unwrap())
                        .collect();
                    let mask = u16::from_str_radix(
                        (0..on_indicator_lights.len()).map(|idx| {
                            match idxs.contains(&idx) {
                                true => '1',
                                false => '0',
                            }
                        })
                        .collect::<String>()
                        .as_str(),
                        2,
                    ).unwrap();
                    Button { mask }
                })
                .collect();
            let on_indicator_lights = u16::from_str_radix(
                on_indicator_lights.chars()
                    .map(|ch| {
                        match ch {
                            '.' => '0',
                            '#' => '1',
                            _ => unreachable!(),
                        }
                    })
                    .collect::<String>()
                    .as_str(),
                    2,
                ).unwrap();
            let joltages = caps.name("joltages")
                .unwrap()
                .as_str()
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect();
            Self {
                on_indicator_lights,
                buttons,
                joltages,
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
                on_indicator_lights: 6,
                buttons: vec![
                    Button { mask:  1 },
                    Button { mask:  5 },
                    Button { mask:  2 },
                    Button { mask:  3 },
                    Button { mask: 10 },
                    Button { mask: 12 },
                ],
                joltages: vec![3, 5, 4, 7],
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
#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 15 };

mod utils {
    pub fn hash(input: &str) -> u32 {
        let mut value = 0;
        for ch in input.chars() {
            value += ch as u32;
            value *= 17;
            value %= 256;
        }
        value
    }
}

pub mod part_one {

    use crate::utils::{solution::{Solution, Answer}, io_utils};
    use super::utils;

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct Soln {
        sum_of_hashes: u32,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.sum_of_hashes)
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.sum_of_hashes = io_utils::file_to_string(filename)
                .split(",")
                .map(|step| utils::hash(step))
                .sum();
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(1_320); "example_1")]
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

    use std::collections::HashMap;

    use itertools::Itertools;
    use regex::Regex;

    use crate::utils::{solution::{Solution, Answer}, io_utils};
    use super::utils;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct LensData {
        slot: u32,
        focal_length: u32,
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct Soln {
        current_slots: HashMap<u32, u32>,
        map: HashMap<u32, HashMap<String, LensData>>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.focusing_power())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let re = Regex::new(r"(?<label>[a-z]+)(?<operation>[=\-])(?<focal_length>[1-9]?)").unwrap();
            io_utils::file_to_string(filename)
                .split(",")
                .for_each(|step| {
                    let captures = re.captures(&step)
                        .expect("Input line should match known form.");
                    let label = captures.name("label").unwrap().as_str().to_string();
                    let operation = captures.name("operation").unwrap().as_str();
                    let h = utils::hash(&label);
                    match operation {
                        "-" => {
                            if let Some(box_map) = self.map.get_mut(&h) {
                                box_map.remove(&label);
                            }
                        },
                        "=" => {
                            let focal_length = captures.name("focal_length").unwrap().as_str().parse().unwrap();
                            self.current_slots.entry(h).and_modify(|slot| *slot += 1).or_insert(0);
                            self.map.entry(h)
                                .and_modify(|box_map| {
                                    box_map.entry(label.clone())
                                        .and_modify(|lens_data| {
                                            lens_data.focal_length = focal_length;
                                        })
                                        .or_insert(LensData { slot: *self.current_slots.get(&h).unwrap(), focal_length });
                                })
                                .or_insert(HashMap::from([(
                                    label,
                                    LensData {
                                        slot: *self.current_slots.get(&h).unwrap(),
                                        focal_length,
                                    }
                                )]));
                        },
                        _ => panic!("Unrecognized operation."),
                    }
                });
        }

        fn focusing_power(&self) -> u32 {
            self.map.iter()
                .map(|(b, lenses)| {
                    let lenses_val: u32 = lenses.values().sorted().enumerate().map(|(idx, lens_data)| -> u32 {
                        (idx as u32 + 1) * lens_data.focal_length
                    }).sum();
                    (*b + 1) * lenses_val
                })
                .sum()
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(145); "example_1")]
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
#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2015, day: 12 };

pub mod part_one {
    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, Default)]
    pub struct Soln {
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            let number_re = Regex::new(r"\-?\d+").unwrap();
            Answer::I64(
                number_re.find_iter(&io_utils::file_to_string(filename)).map(|m| {
                    m.as_str().parse::<i64>().unwrap()
                }).sum()
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::I64(6); "example_1")]
        #[test_case(2, Answer::I64(6); "example_2")]
        #[test_case(3, Answer::I64(3); "example_3")]
        #[test_case(4, Answer::I64(3); "example_4")]
        #[test_case(5, Answer::I64(0); "example_5")]
        #[test_case(6, Answer::I64(0); "example_6")]
        #[test_case(7, Answer::I64(0); "example_7")]
        #[test_case(8, Answer::I64(0); "example_8")]
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
    use std::{collections::{HashMap, VecDeque}, f32::consts::PI};

    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, Default)]
    pub struct Soln {
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            let number_re = Regex::new(r"\-?\d+").unwrap();
            let red_re = Regex::new(r#"\{[^\{]+:"red""#).unwrap();
            let input = io_utils::file_to_string(filename);
            let closer_to_opener = HashMap::from([('}', '{'), (']', '[')]);
            /**
             *  TODO: find the "red"s in objects
             *        scan forward to find the closing of the object.
             *          use a VecDeque to track depth. once it's empty, we're done.
             *        that gives us exclusion zones
             *        then filter out the number matches with indices in any of those zones
             */
            // (start, end) tuples of the ojects with a "red"-valued property
            let exclusion_zones: Vec<(usize, usize)> = red_re.find_iter(&input).map(|m| {
                let mut stack = VecDeque::new();
                let mut input_chars = (&input[m.start()..]).char_indices();                
                loop {
                    let (idx, ch) = input_chars.next().unwrap();
                    match ch {
                        '{' | '[' => {
                            stack.push_back(ch);
                        },
                        '}' | ']' => {
                            let opener = stack.pop_back().unwrap();
                            assert_eq!(opener, *closer_to_opener.get(&ch).unwrap());
                        },
                        _ => (),
                    }
                    if stack.is_empty() {
                        return (m.start(), m.start() + idx)
                    }
                }
            }).collect();
            let included_numbers: i64 = number_re.find_iter(&input)
                .filter(|m|{
                    !exclusion_zones.iter().any(|(start, end)| {
                        *start <= m.start() && *end >= m.start()
                    })
                })
                .map(|m| {
                    m.as_str().parse::<i64>().unwrap()
                }).sum();
            Answer::I64(included_numbers)
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::I64(6); "example_1")]
        #[test_case(9, Answer::I64(4); "example_9")]
        #[test_case(10, Answer::I64(0); "example_10")]
        #[test_case(11, Answer::I64(6); "example_11")]
        #[test_case(12, Answer::I64(4); "example_12")]
        #[test_case(13, Answer::I64(4); "example_13")]
        #[test_case(14, Answer::I64(10); "example_14")]
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
#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 6 };

pub mod part_one {
    use std::collections::HashMap;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, Default)]
    pub struct Soln {
        most_frequent_chars: Vec<HashMap<char, u32>>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::String(self.most_frequent_string())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            for line in io_utils::file_to_lines(filename) {
                if self.most_frequent_chars.is_empty() {
                    for ch in line.chars() {
                        self.most_frequent_chars.push(HashMap::from([(ch, 1)]));
                    }
                } else {
                    for (idx, ch) in line.char_indices() {
                        *self.most_frequent_chars[idx].entry(ch).or_insert(0) += 1;
                    }
                }
            }
        }   

        fn most_frequent_string(&self) -> String {
            assert!(!self.most_frequent_chars.is_empty());
            self.most_frequent_chars.iter().map(|mfc| {
                mfc.iter().max_by_key(|(_, count)| *count).unwrap().0
            }).collect::<String>()
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::String(String::from("easter")); "example_1")]
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

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    fn least_frequent_char(char_frequency: &HashMap<char, u32>) -> char {
        *char_frequency.iter().min_by_key(|(_, count)| *count).unwrap().0
    }

    pub struct Soln {
        char_frequencies: Vec<HashMap<char, u32>>,
        frequency_to_char: Box<dyn Fn(&HashMap<char, u32>) -> char>
    }

    impl Default for Soln {
        fn default() -> Self {
            Self {
                char_frequencies: Vec::new(),
                frequency_to_char: Box::new(least_frequent_char),
            }
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::String(self.frequencies_to_string())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            for line in io_utils::file_to_lines(filename) {
                if self.char_frequencies.is_empty() {
                    for ch in line.chars() {
                        self.char_frequencies.push(HashMap::from([(ch, 1)]));
                    }
                } else {
                    for (idx, ch) in line.char_indices() {
                        *self.char_frequencies[idx].entry(ch).or_insert(0) += 1;
                    }
                }
            }
        }   

        fn frequencies_to_string(&self) -> String {
            assert!(!self.char_frequencies.is_empty());
            self.char_frequencies.iter()
                .map(&self.frequency_to_char)
                .collect::<String>()
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::String(String::from("advent")); "example_1")]
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
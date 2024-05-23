#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 6 };

mod utils {
    use std::collections::HashMap;

    use crate::utils::io_utils;

    pub struct RepetitionDecoder {
        char_frequencies: Vec<HashMap<char, u32>>,
        frequency_to_char: Box<dyn Fn(&HashMap<char, u32>) -> char>
    }

    impl RepetitionDecoder {
        pub fn new(frequency_to_char: Box<dyn Fn(&HashMap<char, u32>) -> char>) -> Self {
            Self {
                char_frequencies: Vec::new(),
                frequency_to_char: Box::new(frequency_to_char),
            }
        }
    }

    impl RepetitionDecoder {
        pub fn parse_input_file(&mut self, filename: &str) {
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

        pub fn frequencies_to_string(&self) -> String {
            assert!(!self.char_frequencies.is_empty());
            self.char_frequencies.iter()
                .map(&self.frequency_to_char)
                .collect::<String>()
        }
    }
}

pub mod part_one {
    use std::collections::HashMap;

    use crate::utils::solution::{Answer, Solution};

    use super::utils::RepetitionDecoder;

    fn least_frequent_char(char_frequency: &HashMap<char, u32>) -> char {
        *char_frequency.iter().max_by_key(|(_, count)| *count).unwrap().0
    }

    pub struct Soln {
        repetition_decoder: RepetitionDecoder,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self {
                repetition_decoder: RepetitionDecoder::new(Box::new(least_frequent_char)),
            }
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.repetition_decoder.parse_input_file(filename);
            Answer::String(self.repetition_decoder.frequencies_to_string())
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

    use crate::utils::solution::{Answer, Solution};

    use super::utils::RepetitionDecoder;

    fn least_frequent_char(char_frequency: &HashMap<char, u32>) -> char {
        *char_frequency.iter().min_by_key(|(_, count)| *count).unwrap().0
    }

    pub struct Soln {
        repetition_decoder: RepetitionDecoder,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self {
                repetition_decoder: RepetitionDecoder::new(Box::new(least_frequent_char)),
            }
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.repetition_decoder.parse_input_file(filename);
            Answer::String(self.repetition_decoder.frequencies_to_string())
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
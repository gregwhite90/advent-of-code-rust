#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 1};

/// Defines utilities shared by both parts of 2017-01 solution.
mod utils {
    pub use unicode_segmentation::UnicodeSegmentation;
    use regex::Regex;

    /// Creates a vector of integers from a string slice of numerical digits.
    /// Collects into a vector to satisfy shared requirements of client code
    /// (trait bounds on circular_tuple_windows and indexing).
    /// 
    /// # Arguments
    /// 
    /// * `text` A string slice of ASCII numerical digits
    pub fn digits(text: &str) -> Vec<u32> {
        // Confirm that text is made up of ASCII numerical digits
        let re = Regex::new(r"^[0-9]+$").unwrap();
        assert!(re.is_match(text));

        text
            .graphemes(true)
            .map(|digit| digit.parse::<u32>().unwrap())
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn parses_numerical_digits() {
            assert_eq!(digits("1234"), vec![1, 2, 3, 4]);
        }

        #[test]
        #[should_panic]
        fn panics_empty_input() {
            digits("");
        }

        #[test]
        #[should_panic]
        fn panics_with_non_numerical_characters() {
            digits("abc");
        }
    }

}

/// Solves 2017-01 part one
pub mod part_one {
    pub use itertools::Itertools;
    use super::utils;
    use crate::utils::{solution::{Solution, Answer}, io_utils};

    #[derive(Default)]
    pub struct Soln {
        text: String,
    }

    impl Solution for Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.text = io_utils::file_to_string(filename);
        }

        fn solve(&mut self) -> Answer {
            Answer::U32(self.sum_of_repeated_digits())
        }
    }
    
    impl Soln {
        /// Finds the sum of all repeated consecutive digits, considered circularly.
        fn sum_of_repeated_digits(&self) -> u32 {
            // Collect into a vector and then back to an iterator to satisfy trait bounds in 
            // `circular_tuple_windows
            let digits = utils::digits(&self.text);
            digits
                .iter()
                .circular_tuple_windows()
                .fold(0, |acc, (&elem, &next)| acc + if elem == next { elem } else { 0 })
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::solution::Answer;
        use crate::utils::test_utils;
        use super::*;    
        use super::super::DAY;

        #[test_case(1, Answer::U32(3); "example_1")]
        #[test_case(2, Answer::U32(4); "example_2")]
        #[test_case(3, Answer::U32(0); "example_3")]
        #[test_case(4, Answer::U32(9); "example_4")]
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

/// Solves 20017-01 part two
pub mod part_two {
    use super::utils;
    use crate::utils::{solution::{Solution, Answer}, io_utils};

    #[derive(Default)]
    pub struct Soln {
        text: String,
    }

    impl Solution for Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.text = io_utils::file_to_string(filename);
        }

        fn solve(&mut self) -> Answer {
            Answer::U32(self.sum_of_matching_halfway_around_digits())
        }
    }
    
    impl Soln {    
        /// Finds the sum of the digits halfway around the string (considered circularly)
        /// that match.
        pub fn sum_of_matching_halfway_around_digits(&self) -> u32 {
            // Collect into a vector and then back into an iterator to allow enumeration and indexing
            let digits = utils::digits(&self.text);
            assert!(digits.len() % 2 == 0);
            let mut sum: u32 = 0;
            for (i, &digit) in digits.iter().enumerate() {
                if digit == digits[(i + (digits.len() / 2)) % digits.len()] {
                    sum += digit;
                }
            }
            sum
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};    
        use super::*;
        use super::super::DAY;

        #[test_case(5, Answer::U32(6); "example_5")]
        #[test_case(6, Answer::U32(0); "example_6")]
        #[test_case(7, Answer::U32(4); "example_7")]
        #[test_case(8, Answer::U32(12); "example_8")]
        #[test_case(9, Answer::U32(4); "example_9")]
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
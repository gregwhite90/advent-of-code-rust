#[cfg(test)]
const YEAR: u32 = 2017;
#[cfg(test)]
const DAY: u8 = 1;

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
    /// 
    /// # Examples
    /// ```
    /// use crate::year_2017::day_01::utils::digits;
    /// assert_eq!(digits("1234"), vec![1, 2, 3, 4]);
    /// ```
    /// ```should_panic
    /// use crate::year_2017::day_01::utils::digits;
    /// digits("abc");
    /// ```
    pub fn digits(text: &str) -> Vec<i32> {
        // Confirm that text is made up of ASCII numerical digits
        let re = Regex::new(r"^[0-9]+$").unwrap();
        assert!(re.is_match(text));

        text
            .graphemes(true)
            .map(|digit| digit.parse::<i32>().unwrap())
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
    use std::fs;
    pub use itertools::Itertools;
    pub use either::*;
    use crate::year_2017::day_01::utils;
    use crate::utils::utils::Solution;

    #[derive(Default)]
    pub struct Soln {
        text: String,
    }

    impl Solution for Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.text = fs::read_to_string(filename)
                .expect("Should be able to read the file to a string.");
        }

        fn solve(&mut self) -> Either<i32, String> {
            Left(self.sum_of_repeated_digits())
        }
    }
    
    impl Soln {
        /// Finds the sum of all repeated consecutive digits, considered circularly.
        fn sum_of_repeated_digits(&self) -> i32 {
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
        use std::collections::HashMap;
        use either::*;
        use crate::utils::test_utils;
        use super::*;    
        use super::super::{YEAR, DAY};

        #[test]
        fn examples_are_correct() {
            test_utils::check_example_cases(
                &mut Soln::default(),
                &HashMap::from([
                    (1u8, Left(3)),
                    (2,   Left(4)),
                    (3,   Left(0)),
                    (4,   Left(9)),
                ]),
                YEAR,
                DAY,
            );
        }
    }

}

/// Solves 20017-01 part two
pub mod part_two {
    use std::fs;
    pub use either::*;
    use crate::year_2017::day_01::utils;
    use crate::utils::utils::Solution;

    #[derive(Default)]
    pub struct Soln {
        text: String,
    }

    impl Solution for Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.text = fs::read_to_string(filename)
                .expect("Should be able to read the file to a string.");
        }

        fn solve(&mut self) -> Either<i32, String> {
            Left(self.sum_of_matching_halfway_around_digits())
        }
    }
    
    impl Soln {    
        /// Finds the sum of the digits halfway around the string (considered circularly)
        /// that match.
        pub fn sum_of_matching_halfway_around_digits(&self) -> i32 {
            // Collect into a vector and then back into an iterator to allow enumeration and indexing
            let digits = utils::digits(&self.text);
            assert!(digits.len() % 2 == 0);
            let mut sum: i32 = 0;
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
        use std::collections::HashMap;
        use either::*;
        use crate::utils::test_utils;    
        use super::*;
        use super::super::{YEAR, DAY};

        #[test]
        fn examples_are_correct() {
            test_utils::check_example_cases(
                &mut Soln::default(),
                &HashMap::from([
                    (5u8, Left(6)),
                    (6,   Left(0)),
                    (7,   Left(4)),
                    (8,   Left(12)),
                    (9,   Left(4)),
                ]),
                YEAR,
                DAY,
            );
        }    
    }
}
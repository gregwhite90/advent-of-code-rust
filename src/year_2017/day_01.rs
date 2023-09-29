/// Defines utilities shared by both parts of 2017-01 solution.
mod utils {
    pub use unicode_segmentation::UnicodeSegmentation;
    use regex::Regex;

    /// Creates a vector of unsigned integers from a string slice of numerical digits.
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
    /// assert_eq!(digits("1234"), vec![1u32, 2, 3, 4]);
    /// ```
    /// ```should_panic
    /// use crate::year_2017::day_01::utils::digits;
    /// let panicky_call = digits("abc");
    /// ```
    pub fn digits(text: &str) -> Vec<u32> {

        // Confirm that text is made up of ASCII numerical digits
        let re = Regex::new(r"^[0-9]+$").unwrap();
        assert!(re.is_match(text));

        text
            .graphemes(true)
            .map(|digit| digit.parse::<u32>().unwrap())
            .collect()
    }
}

/// Solves 2017-01 part one.
pub mod part_one {
    pub use itertools::Itertools;
    use crate::year_2017::day_01::utils;
    
    /// Finds the sum of all repeated consecutive digits, considered circularly.
    pub fn sum_of_repeated_digits(text: &str) -> u32 {
        // Collect into a vector and then back to an iterator to satisfy trait bounds in 
        // `circular_tuple_windows
        let digits = utils::digits(text);
        digits
            .iter()
            .circular_tuple_windows()
            .fold(0, |acc, (&elem, &next)| acc + if elem == next { elem } else { 0 })
    }
}

/// Solves 20017-01 part two.
pub mod part_two {
    use crate::year_2017::day_01::utils;
    
    /// Finds the sum of the digits halfway around the string (considered circularly)
    /// that match.
    pub fn sum_of_matching_halfway_around_digits(text: &str) -> u32 {
        // Collect into a vector and then back into an iterator to allow enumeration and indexing
        let digits = utils::digits(text);
        let mut sum: u32 = 0;
        for (i, &digit) in digits.iter().enumerate() {
            if digit == digits[(i + (digits.len() / 2)) % digits.len()] {
                sum = sum + digit;
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    mod utils {
        use crate::year_2017::day_01::utils;

        #[test]
        fn parses_numerical_digits() {
            assert_eq!(utils::digits("1234"), vec![1u32, 2, 3, 4]);
        }

        #[test]
        #[should_panic]
        fn panics_empty_input() {
            utils::digits("");
        }

        #[test]
        #[should_panic]
        fn panics_with_non_numerical_characters() {
            utils::digits("abc");
        }
    }
    mod part_one {
        use crate::year_2017::day_01::part_one::sum_of_repeated_digits;

        #[test]
        fn examples_are_correct() {
            assert_eq!(3, sum_of_repeated_digits(include_str!("input/day_01/test_examples/example_1.txt")));
            assert_eq!(4, sum_of_repeated_digits(include_str!("input/day_01/test_examples/example_2.txt")));
            assert_eq!(0, sum_of_repeated_digits(include_str!("input/day_01/test_examples/example_3.txt")));
            assert_eq!(9, sum_of_repeated_digits(include_str!("input/day_01/test_examples/example_4.txt")));
        }
    }

    mod part_two {
        use crate::year_2017::day_01::part_two::sum_of_matching_halfway_around_digits;

        #[test]
        fn examples_are_correct() {
            assert_eq!(6, sum_of_matching_halfway_around_digits(include_str!("input/day_01/test_examples/example_5.txt")));
            assert_eq!(0, sum_of_matching_halfway_around_digits(include_str!("input/day_01/test_examples/example_6.txt")));
            assert_eq!(4, sum_of_matching_halfway_around_digits(include_str!("input/day_01/test_examples/example_7.txt")));
            assert_eq!(12, sum_of_matching_halfway_around_digits(include_str!("input/day_01/test_examples/example_8.txt")));
            assert_eq!(4, sum_of_matching_halfway_around_digits(include_str!("input/day_01/test_examples/example_9.txt")));
        }    
    }
}
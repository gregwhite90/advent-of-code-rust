#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2025, day: 2 };

pub mod part_one {
    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug)]
    struct Range {
        invalid_ids_sum: usize,
    }

    impl Range {
        pub fn from_str(input: &str) -> Self {
            unimplemented!()
        }

        pub fn invalid_ids_sum(&self) -> usize {
            self.invalid_ids_sum
        }
    }

    #[derive(Debug, Default)]
    pub struct Soln {}

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            Answer::Usize(
                io_utils::file_to_lines(filename)
                    .map(|line| {
                        Range::from_str(&line).invalid_ids_sum()
                    })
                    .sum()
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case("171-161", 0; "same_length_odd")]
        #[test_case("21-57", 154; "same_length_even_includes_both")]
        #[test_case("25-57", 132; "same_length_even_excludes_lower")]
        #[test_case("21-52", 99; "same_length_even_excludes_upper")]
        #[test_case("25-52", 77; "same_length_even_excludes_both")]
        #[test_case("1-31", 33; "different_lengths_odd_to_even")]
        #[test_case("1-161", 495; "different_lengths_odd_to_odd")]
        #[test_case("78-101", 187; "different_lengths_even_to_odd")]
        #[test_case("78-1320", 3_520; "different_lengths_even_to_even")]
        #[test_case("11-22", 33; "puzzle_11-22")]
        #[test_case("95-115", 99; "puzzle_95-115")]
        #[test_case("998-1012", 1010; "puzzle_998-1012")]
        #[test_case("1188511880-1188511890", 1_188_511_885; "puzzle_1188511880-1188511890")]
        #[test_case("1698522-1698528", 0; "puzzle_1698522-1698528")]
        #[test_case("446443-446449", 446_446; "puzzle_446443-446449")]
        #[test_case("38593856-38593862", 38_593_859; "puzzle_38593856-38593862")]
        fn individual_examples_are_correct(input: &str, invalid_ids_sum: usize) {
            assert_eq!(
                Range::from_str(input).invalid_ids_sum(),
                invalid_ids_sum,
            )
        }

        #[test_case(1, Answer::Usize(1_227_775_554); "example_1")]
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
#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2025, day: 2 };

mod utils {
    use std::collections::HashSet;

    #[derive(Debug)]
    pub struct Range {
        invalid_ids_sum: usize,
    }

    impl Range {
        pub fn from_str(
            input: &str,
            more_than_two_repeats_allowed: bool,
        ) -> Self {
            let bounds: Vec<&str> = input.split('-').collect();
            let bounds_digits: Vec<usize> = bounds.iter().map(|bound| bound.len()).collect();
            let invalid_ids: HashSet<usize> = (bounds_digits[0]..=bounds_digits[1])
                .flat_map(|num_digits| {
                    let lower: usize = std::cmp::max(bounds[0].parse().unwrap(), 10usize.pow((num_digits - 1) as u32));
                    let upper: usize = std::cmp::min(bounds[1].parse().unwrap(), 10usize.pow(num_digits as u32) - 1);
                    let max_repeats: usize = if more_than_two_repeats_allowed {
                        num_digits
                    } else {
                        2
                    };
                    (2..=max_repeats)
                        .filter(|repeats| num_digits % *repeats == 0)
                        .flat_map(|repeats| {
                            // If num_digits is divisible by the number of repeats, invalid IDs are possible
                            let prefix_len: usize = num_digits / repeats;
                            let lower_prefix = &lower.to_string()[..prefix_len];
                            let upper_prefix = &upper.to_string()[..prefix_len];
                            (lower_prefix.parse::<usize>().unwrap()..=upper_prefix.parse::<usize>().unwrap())
                                .map(|prefix| {
                                    prefix.to_string().repeat(repeats).parse::<usize>().unwrap()
                                })
                                .filter(|num| {
                                    lower <= *num && *num <= upper
                                })
                                .collect::<HashSet<usize>>()
                        })
                        .collect::<HashSet<usize>>()
                })
                .collect();
            Self { 
                invalid_ids_sum: invalid_ids.iter().sum()
            } 
        }

        pub fn invalid_ids_sum(&self) -> usize {
            self.invalid_ids_sum
        }
    }
}

pub mod part_one {
    use crate::utils::{io_utils, solution::{Answer, Solution}};
    use super::utils::Range;

    #[derive(Debug, Default)]
    pub struct Soln {}

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            Answer::Usize(
                io_utils::file_to_lines(filename)
                    .map(|line| {
                        line.split(',')
                            .map(|range_str| {
                                Range::from_str(range_str, false).invalid_ids_sum()
                            })
                            .sum::<usize>()
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

        #[test_case("161-171", 0; "same_length_odd")]
        #[test_case("21-57", 154; "same_length_even_includes_both")]
        #[test_case("25-57", 132; "same_length_even_excludes_lower")]
        #[test_case("21-52", 99; "same_length_even_excludes_upper")]
        #[test_case("25-52", 77; "same_length_even_excludes_both")]
        #[test_case("1-31", 33; "different_lengths_odd_to_even")]
        #[test_case("1-161", 495; "different_lengths_odd_to_odd")]
        #[test_case("78-101", 187; "different_lengths_even_to_odd")]
        #[test_case("78-1320", 4_833; "different_lengths_even_to_even")]
        #[test_case("11-22", 33; "puzzle_11-22")]
        #[test_case("95-115", 99; "puzzle_95-115")]
        #[test_case("998-1012", 1010; "puzzle_998-1012")]
        #[test_case("1188511880-1188511890", 1_188_511_885; "puzzle_1188511880-1188511890")]
        #[test_case("1698522-1698528", 0; "puzzle_1698522-1698528")]
        #[test_case("446443-446449", 446_446; "puzzle_446443-446449")]
        #[test_case("38593856-38593862", 38_593_859; "puzzle_38593856-38593862")]
        fn individual_examples_are_correct(input: &str, invalid_ids_sum: usize) {
            assert_eq!(
                Range::from_str(input, false).invalid_ids_sum(),
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

pub mod part_two {
    use crate::utils::{io_utils, solution::{Answer, Solution}};
    use super::utils::Range;

    #[derive(Debug, Default)]
    pub struct Soln {}

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            Answer::Usize(
                io_utils::file_to_lines(filename)
                    .map(|line| {
                        line.split(',')
                            .map(|range_str| {
                                let invalid_ids_sum = Range::from_str(range_str, true).invalid_ids_sum();
                                invalid_ids_sum
                            })
                            .sum::<usize>()
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

        #[test_case("101-121", 111; "same_length_odd")]
        #[test_case("21-57", 154; "same_length_even_includes_both")]
        #[test_case("25-57", 132; "same_length_even_excludes_lower")]
        #[test_case("21-52", 99; "same_length_even_excludes_upper")]
        #[test_case("25-52", 77; "same_length_even_excludes_both")]
        #[test_case("1-31", 33; "different_lengths_odd_to_even")]
        #[test_case("1-161", 606; "different_lengths_odd_to_odd")]
        #[test_case("78-101", 187; "different_lengths_even_to_odd")]
        #[test_case(
            "78-1320", 
            88 + 99 + 111 + 222 + 333 + 444 + 555 + 666 + 777 + 888 + 999 + 1010 + 1111 + 1212 + 1313;
            "different_lengths_even_to_even"
        )]
        #[test_case("11-22", 33; "puzzle_11-22")]
        #[test_case("95-115", 210; "puzzle_95-115")]
        #[test_case("998-1012", 2_009; "puzzle_998-1012")]
        #[test_case("1188511880-1188511890", 1_188_511_885; "puzzle_1188511880-1188511890")]
        #[test_case("222220-222224", 222_222; "puzzle_222220-222224")]
        #[test_case("1698522-1698528", 0; "puzzle_1698522-1698528")]
        #[test_case("446443-446449", 446_446; "puzzle_446443-446449")]
        #[test_case("38593856-38593862", 38_593_859; "puzzle_38593856-38593862")]
        #[test_case("565653-565659", 565_656; "puzzle_565653-565659")]
        #[test_case("824824821-824824827", 824_824_824; "puzzle_824824821-824824827")]
        #[test_case("2121212118-2121212124", 2_121_212_121; "puzzle_2121212118-2121212124")]
        fn individual_examples_are_correct(input: &str, invalid_ids_sum: usize) {
            assert_eq!(
                Range::from_str(input, true).invalid_ids_sum(),
                invalid_ids_sum,
            )
        }

        #[test_case(1, Answer::Usize(4_174_379_265); "example_1")]
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
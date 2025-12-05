#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2025, day: 5 };

mod utils {
    use std::collections::{HashMap, HashSet, VecDeque};
    use lazy_static::lazy_static;
    use regex::Regex;

    use crate::utils::io_utils;

    lazy_static! {
        pub static ref RANGE_RE: Regex = Regex::new(r"(?<start>\d+)\-(?<end>\d+)").unwrap();
    }

    #[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
    pub struct Range {
        start: u64,
        end: u64,
    }

    impl Range {
        pub fn new(start: u64, end: u64) -> Self {
            Self {
                start,
                end,
            }
        }

        pub fn from_str(input: &str) -> Self {
            let captures = RANGE_RE.captures(input).unwrap();
            Self {
                start: captures.name("start").unwrap().as_str().parse().unwrap(),
                end: captures.name("end").unwrap().as_str().parse().unwrap(),
            }
        }
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct FreshIngredientRanges {
        // Maintains sorted order, merges Ranges where applicable
        ranges: Vec<Range>,
    }

    impl FreshIngredientRanges {
        pub fn add_range_str(&mut self, input: &str) {
            let range = Range::from_str(input);
            let idx = self.ranges.partition_point(|&r| r.start <= range.start);
            // Merge with left if possible
            let mut right_idx = idx;
            let comparison_idx = if idx > 0 && self.ranges[idx - 1].end >= range.start {
                self.ranges[idx - 1].end = std::cmp::max(
                    self.ranges[idx - 1].end,
                    range.end,
                );
                idx - 1
            } else {
                self.ranges.insert(idx, range);
                right_idx += 1;
                idx
            };
            // Merge with as many right as possible
            let end_idx = self.ranges[right_idx..].partition_point(|&r| r.start <= self.ranges[comparison_idx].end);
            self.ranges[comparison_idx].end = std::cmp::max(
                self.ranges[comparison_idx].end,
                self.ranges[right_idx + end_idx - 1].end,
            );
            self.ranges.drain(right_idx..right_idx + end_idx);
        }

        pub fn ranges(&self) -> &Vec<Range> {
            &self.ranges
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    #[derive(Debug, Default)]
    pub struct Soln {
    }

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
        use super::super::{DAY, utils::{Range, FreshIngredientRanges}};

        #[test_case(
            vec!["3-5"],
            vec![Range::new(3, 5)];
            "adds_once"
        )]
        #[test_case(
            vec!["3-5", "6-10"],
            vec![Range::new(3, 5), Range::new(6, 10)];
            "adds_twice_no_overlap"
        )]
        #[test_case(
            vec!["3-5", "8-10", "6-7"],
            vec![Range::new(3, 5), Range::new(6, 7), Range::new(8, 10)];
            "adds_in_middle_with_no_overlap"
        )]
        #[test_case(
            vec!["3-5", "8-10", "4-7"],
            vec![Range::new(3, 7), Range::new(8, 10)];
            "adds_in_middle_with_left_overlap"
        )]
        #[test_case(
            vec!["3-5", "8-10", "7-9"],
            vec![Range::new(3, 5), Range::new(7, 10)];
            "adds_in_middle_with_right_overlap"
        )]
        #[test_case(
            vec!["3-5", "8-10", "7-11"],
            vec![Range::new(3, 5), Range::new(7, 11)];
            "adds_in_middle_with_right_full_overlap"
        )]
        #[test_case(
            vec!["3-5", "8-10", "4-9"],
            vec![Range::new(3, 10)];
            "adds_in_middle_with_both_overlap"
        )]
        #[test_case(
            vec!["3-5", "8-10", "12-15"],
            vec![Range::new(3, 5), Range::new(8, 10), Range::new(12, 15)];
            "adds_at_end"
        )]
        #[test_case(
            vec!["3-5", "8-10", "1-2"],
            vec![Range::new(1, 2), Range::new(3, 5), Range::new(8, 10)];
            "adds_at_start"
        )]
        #[test_case(
            vec!["3-5", "8-10", "11-13", "4-12"],
            vec![Range::new(3, 13)];
            "adds_with multiple overlap"
        )]
        fn fresh_ingredient_ranges_adds_correctly(
            input_range_strs: Vec<&str>,
            expected: Vec<Range>
        ) {
            let mut fresh_ingredient_ranges = FreshIngredientRanges::default();
            for input_range_str in input_range_strs.iter() {
                fresh_ingredient_ranges.add_range_str(input_range_str);
            }
            assert_eq!(
                *fresh_ingredient_ranges.ranges(),
                expected,
            )
        }

        #[test_case(1, Answer::Usize(3); "example_1")]
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
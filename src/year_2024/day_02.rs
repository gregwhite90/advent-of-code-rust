#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2024, day: 2 };

mod utils {
    use itertools::Itertools;


    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum ReportOrdering {
        Increasing,
        Decreasing,
    }

    #[derive(Debug)]
    pub struct Report {
        levels: Vec<usize>,
        tolerance: usize,
    }

    impl Report {
        pub fn from_str(input: &str, tolerance: usize) -> Self {
            Self {
                levels: input.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect(),
                tolerance,
            }
        }

        pub fn is_safe(&self) -> bool {
            for removals in 0..=self.tolerance {
                for levels in self.levels.iter().combinations(self.levels.len() - removals) {
                    if is_safe(&levels) { return true; }
                }
            }
            false
        }
    }

    fn is_safe(levels: &[&usize]) -> bool {
        let mut ordering: Option<ReportOrdering> = None;
        for window in levels.windows(2) {
            let diff = window[0].abs_diff(*window[1]);
            if diff == 0 || diff > 3 { 
                return false;
            }
            let window_ordering = if window[0] > window[1] { 
                ReportOrdering::Decreasing 
            } else { 
                ReportOrdering::Increasing 
            };
            if let Some(o) = ordering {
                if o != window_ordering { 
                    return false; 
                }
            } else {
                ordering = Some(window_ordering);
            }
        }
        true
    }
}

pub mod part_one {
    use crate::utils::{io_utils, solution::{Answer, Solution}};

    use super::utils::Report;

    #[derive(Debug, Default)]
    pub struct Soln {}

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            Answer::Usize(
                io_utils::file_to_lines(filename)
                    .filter(|line| {
                        let report = Report::from_str(&line, 0);
                        report.is_safe()
                    })
                    .count()
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(2); "example_1")]
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

    use super::utils::Report;

    #[derive(Debug, Default)]
    pub struct Soln {}

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            Answer::Usize(
                io_utils::file_to_lines(filename)
                    .filter(|line| {
                        let report = Report::from_str(&line, 1);
                        report.is_safe()
                    })
                    .count()
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(4); "example_1")]
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
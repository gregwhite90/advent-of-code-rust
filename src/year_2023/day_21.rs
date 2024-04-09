#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 21 };

pub mod part_one {

    use std::collections::HashSet;

    use crate::utils::solution::{Answer, Solution};

    #[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
    struct Point {
        row: usize,
        col: usize,
    }

    pub struct Soln {
        steps: u32,
        start: Point,
        rocks: HashSet<Point>,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self::with_steps(64)
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {}
    }

    impl Soln {
        fn with_steps(steps: u32) -> Self {
            Self {
                steps,
                start: Point { row: 0, col: 0 },
                rocks: HashSet::new(),
            }
        }

        fn parse_input_file(&mut self, filename: &str) {

        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(16); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::with_steps(6),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}
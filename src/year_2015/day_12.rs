#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2015, day: 12 };

pub mod part_one {
    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, Default)]
    pub struct Soln {
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            let number_re = Regex::new(r"\-?\d+").unwrap();
            Answer::I64(
                number_re.find_iter(&io_utils::file_to_string(filename)).map(|m| {
                    m.as_str().parse::<i64>().unwrap()
                }).sum()
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::I64(6); "example_1")]
        #[test_case(2, Answer::I64(6); "example_2")]
        #[test_case(3, Answer::I64(3); "example_3")]
        #[test_case(4, Answer::I64(3); "example_4")]
        #[test_case(5, Answer::I64(0); "example_5")]
        #[test_case(6, Answer::I64(0); "example_6")]
        #[test_case(7, Answer::I64(0); "example_7")]
        #[test_case(8, Answer::I64(0); "example_8")]
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
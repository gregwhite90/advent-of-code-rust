#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2024, day: 3 };

pub mod part_one {
    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, Default)]
    pub struct Soln {}

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            let mul_re = Regex::new(r"mul\((?<operand_0>\d{1,3})\,(?<operand_1>\d{1,3})\)").unwrap();
            Answer::Usize(
                io_utils::file_to_lines(filename)
                    .map(|line| {
                        mul_re.captures_iter(&line)
                            .map(|caps| {
                                let operand_0: usize = caps.name("operand_0").unwrap().as_str().parse().unwrap();
                                let operand_1: usize = caps.name("operand_1").unwrap().as_str().parse().unwrap();
                                operand_0 * operand_1
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

        #[test_case(1, Answer::Usize(161); "example_1")]
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
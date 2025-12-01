#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2025, day: 1 };

pub mod part_one {
    use lazy_static::lazy_static;
    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"(?<dir>[LR])(?<num>\d+)").unwrap();
    }

    #[derive(Debug, Default)]
    pub struct Soln {}

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            let mut cur: i32 = 50;
            let mut zero_count: usize = 0;
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    let captures = LINE_RE.captures(&line).unwrap();
                    let num: i32 = captures.name("num").unwrap().as_str().parse().unwrap();
                    match captures.name("dir").unwrap().as_str() {
                        "L" => {
                            cur -= num;
                        },
                        "R" => {
                            cur += num;
                        },       
                        _ => unreachable!(),
                    }
                    if cur % 100 == 0 {
                        zero_count += 1;
                    }
                });
            Answer::Usize(zero_count)
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

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
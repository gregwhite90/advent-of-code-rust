#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2025, day: 1 };

mod utils {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        pub static ref LINE_RE: Regex = Regex::new(r"(?<dir>[LR])(?<num>\d+)").unwrap();
    }

    pub fn div_round_up(numerator: i32, denominator: i32) -> i32 {
        (numerator + denominator - 1) / denominator
    }
}

pub mod part_one {
    use crate::utils::{io_utils, solution::{Answer, Solution}};
    use super::utils::LINE_RE;

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

pub mod part_two {
    use crate::{utils::{io_utils, solution::{Answer, Solution}}};
    use super::utils::{LINE_RE, div_round_up};

    #[derive(Debug, Default)]
    pub struct Soln {}

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            let size: i32 = 100;
            let mut cur: i32 = 50;
            let mut zero_count: i32 = 0;
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    let captures = LINE_RE.captures(&line).unwrap();
                    let num: i32 = captures.name("num").unwrap().as_str().parse().unwrap();
                    match captures.name("dir").unwrap().as_str() {
                        "L" => {
                            zero_count += div_round_up(num - cur, size);
                            // Need this to not double count with the increment when we landed on 0
                            // when moving left off 0.
                            if cur == 0 {
                                zero_count -= 1;
                            }
                            cur = (cur - num).rem_euclid(100);
                        },
                        "R" => {
                            zero_count += div_round_up(num - (size - cur), size);
                            cur = (cur + num).rem_euclid(100);
                        },       
                        _ => unreachable!(),
                    }
                    // Increment if landed on zero.
                    if cur == 0 {
                        zero_count += 1;
                    }
                });
            Answer::I32(zero_count)
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::I32(6); "example_1")]
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
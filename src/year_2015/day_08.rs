#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2015, day: 8 };

pub mod part_one {
    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, Default)]
    pub struct Soln {
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            Answer::Usize(
                io_utils::file_to_lines(filename).map(|line| {
                    let code_count = line.chars().count();
                    let mut string_literal_count = code_count - 2; // To account for the opening and closing quotations
                    let mut chars = line.chars();
                    while let Some(ch) = chars.next() {
                        if ch == '\\' {
                            match chars.next() {
                                Some('\\') => string_literal_count -= 1,
                                Some('"') => string_literal_count -= 1,
                                Some('x') => {
                                    for _ in 0..2 {
                                        chars.next();
                                    }
                                    string_literal_count -= 3;
                                }
                                _ => (),
                            }
                        }
                    }                                   
                    code_count - string_literal_count
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

        #[test_case(1, Answer::Usize(2); "example_1")]
        #[test_case(2, Answer::Usize(2); "example_2")]
        #[test_case(3, Answer::Usize(3); "example_3")]
        #[test_case(4, Answer::Usize(5); "example_4")]
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
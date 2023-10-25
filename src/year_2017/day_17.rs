#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 17 };

pub mod part_one {
    use crate::utils::{solution::{Solution, Answer}, io_utils};

    const INSERTIONS: u32 = 2017;

    #[derive(Debug, PartialEq, Eq)]
    pub struct Soln {
        steps: usize,
        position: usize,
        buf: Vec<u32>,
    }

    impl Default for Soln {
        fn default() -> Self {
            let mut buf = Vec::with_capacity((INSERTIONS + 1).try_into().unwrap());
            buf.push(0);
            Self {
                steps: 0,
                position: 0,
                buf,
            }
        }
    }

    impl Soln {
        pub fn parse_input_file(&mut self, filename: &str) {
            self.steps = io_utils::file_to_string(filename).parse().unwrap();
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            for insertion in 1..=INSERTIONS {
                self.position = (self.position + self.steps) % self.buf.len();
                if self.position == self.buf.len() - 1 {
                    self.buf.push(insertion);
                } else {
                    self.buf.insert(self.position + 1, insertion);
                }
                self.position += 1;
            }
            Answer::U32(self.buf[self.position + 1])
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(638); "example_1")]
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
    use std::cmp::Ordering;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    const INSERTIONS: u32 = 50_000_000;

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        steps: usize,
        position: usize,
        zero_position: usize,
        immediately_after_zero: u32,
    }

    impl Soln {
        pub fn parse_input_file(&mut self, filename: &str) {
            self.steps = io_utils::file_to_string(filename).parse().unwrap();
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            for insertion in 1..=INSERTIONS {
                self.position = (self.position + self.steps) % (insertion as usize);
                match self.position.cmp(&self.zero_position) {
                    Ordering::Less => self.zero_position += 1,
                    Ordering::Equal => self.immediately_after_zero = insertion,
                    Ordering::Greater => (),
                }
                self.position += 1;
            }
            Answer::U32(self.immediately_after_zero)
        }
    }
}
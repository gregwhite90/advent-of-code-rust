#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 15 };

pub mod part_one {

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    fn hash(input: &str) -> u32 {
        let mut value = 0;
        for ch in input.chars() {
            value += ch as u32;
            value *= 17;
            value %= 256;
        }
        value
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct Soln {
        sum_of_hashes: u32,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.sum_of_hashes)
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.sum_of_hashes = io_utils::file_to_string(filename)
                .split(",")
                .map(|step| hash(step))
                .sum();
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(1_320); "example_1")]
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
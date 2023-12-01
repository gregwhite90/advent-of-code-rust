#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 1 };

pub mod part_one {
    //! Note: only guaranteed to work correctly if the input text is ASCII.
    use crate::utils::{solution::{Solution, Answer}, io_utils};

    fn calibration_value(line: &str) -> u32 {
        let mut calibration_value = 0;
        for ch in line.chars() {
            if ch.is_ascii_digit() {
                calibration_value += 10 * ch.to_digit(10).unwrap();
                break;
            }
        }
        for ch in line.chars().rev() {
            if ch.is_ascii_digit() {
                calibration_value += ch.to_digit(10).unwrap();
                break;    
            }
        }
        calibration_value
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        sum_of_calibration_values: u32,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.sum_of_calibration_values)
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.sum_of_calibration_values = io_utils::file_to_lines(filename)
                .map(|line| {
                    calibration_value(&line)
                })
                .sum();
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case("1abc2", 12; "example_line_1")]
        #[test_case("pqr3stu8vwx", 38; "example_line_2")]
        #[test_case("a1b2c3d4e5f", 15; "example_line_3")]
        #[test_case("treb7uchet", 77; "example_line_4")]
        fn example_lines_are_correct(line: &str, value: u32) {
            assert_eq!(calibration_value(line), value);
        }

        #[test_case(1, Answer::U32(142); "example_1")]
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
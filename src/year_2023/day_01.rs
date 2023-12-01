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

pub mod part_two {
    //! Note: only guaranteed to work correctly if the input text is ASCII.
    use regex::Regex;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    fn calibration_value(line: &str, re: &Regex) -> u32 {
        // Workaround because overlapping regex matches not easily supported.
        let mut cur_match = re.find(line).unwrap();
        let first = cur_match.as_str();
        let mut last = first;
        loop {
            let c_m = re.find_at(line, cur_match.start() + 1);
            match c_m {
                None => break,
                Some(m) => {
                    cur_match = m;
                    last = cur_match.as_str();
                },
            }
        }
        10 * convert_digit(first) + convert_digit(last)
    }

    fn convert_digit(digit: &str) -> u32 {
        match digit {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            d => d.parse().unwrap(),
        }
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
            let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d").unwrap();
            self.sum_of_calibration_values = io_utils::file_to_lines(filename)
                .map(|line| {
                    calibration_value(&line, &re)
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

        #[test_case("two1nine", 29; "example_two1nine")]
        #[test_case("eightwothree", 83; "example_eightwothree")]
        #[test_case("abcone2threexyz", 13; "example_abcone2threexyz")]
        #[test_case("xtwone3four", 24; "example_xtwone3four")]
        #[test_case("4nineeightseven2", 42; "example_4nineeightseven2")]
        #[test_case("zoneight234", 14; "example_zoneight234")]
        #[test_case("7pqrstsixteen", 76; "example_7pqrstsixteen")]
        fn example_lines_are_correct(line: &str, value: u32) {
            let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d").unwrap();
            assert_eq!(calibration_value(line, &re), value);
        }

        #[test_case(2, Answer::U32(281); "example_2")]
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
#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2015, day: 10 };

mod utils {
    pub fn look_and_say(input: &str) -> String {
        let mut res = String::new();
        let mut chars = input.chars();
        let mut cur_ch: Option<char> = None;
        let mut cur_count: usize = 0;
        while let Some(ch) = chars.next() {
            if let Some(cur) = cur_ch {
                if cur == ch {
                    cur_count += 1;
                } else {
                    res.push_str(&format!("{}{}", cur_count, cur));
                    cur_ch = Some(ch);
                    cur_count = 1;
                }
            } else {
                cur_ch = Some(ch);
                cur_count += 1;
            }
        }
        res.push_str(&format!("{}{}", cur_count, cur_ch.unwrap()));
        res
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::io_utils;
        use super::*;
        use super::super::DAY;

        #[test_case(1, "11"; "example_1")]
        #[test_case(2, "21"; "example_2")]
        #[test_case(3, "1211"; "example_3")]
        #[test_case(4, "111221"; "example_4")]
        #[test_case(5, "312211"; "example_5")]
        fn examples_are_correct(example_key: u8, expanded: &str) {
            assert_eq!(
                &look_and_say(&io_utils::file_to_string(&io_utils::input_filename(&DAY, io_utils::InputFileType::Example(example_key)))),
                expanded,
            );
        }
    }    
}

pub mod part_one {
    use crate::utils::{io_utils, solution::{Answer, Solution}};

    use super::utils;

    #[derive(Debug, Default)]
    pub struct Soln {
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            let mut res = io_utils::file_to_string(filename);
            for _ in 0..40 {
                res = utils::look_and_say(&res);
            }
            Answer::Usize(res.len())
        }
    }
}
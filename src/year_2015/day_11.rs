#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2015, day: 11 };

mod utils {
    use itertools::Itertools;

    const PASSWORD_LENGTH: usize = 8;

    pub fn to_chars(password: &str) -> Vec<u8> {
        password.chars().map(|ch| ch as u8).collect()
    }

    pub fn to_string(password: &[u8]) -> String {
        String::from_iter(password.iter().map(|ch| *ch as char))
    }

    pub fn is_valid(password: &[u8]) -> bool {
        if password.len() != PASSWORD_LENGTH { return false; }
        if !req_2(password) { return false; }
        if !req_1(password) { return false; }
        if !req_3(password) { return false; }
        true
    }

    fn req_1(password: &[u8]) -> bool {
        password.windows(3).any(|window| {
            window[1] == window[0] + 1 && window[2] == window[1] + 1
        })
    }

    fn req_2(password: &[u8]) -> bool {
        !password.iter().any(|ch| {
            *ch == 'i' as u8
                || *ch == 'o' as u8
                || *ch == 'l' as u8
        })
    }

    fn req_3(password: &[u8]) -> bool {
        password.windows(2).enumerate()
            .filter(|(_idx, chs)| {
                chs[0] == chs[1]
            })
            .combinations(2)
            .any(|repeats| {
                repeats[1].0 - repeats[0].0 > 1
            })
    }

    pub fn increment(password: &[u8]) -> Vec<u8> {
        let mut res = Vec::from(password);
        let mut idx = PASSWORD_LENGTH - 1;
        while idx > 0 && res[idx] == 'z' as u8 {
            res[idx] = 'a' as u8;
            idx -= 1;
        }
        res[idx] += 1;
        if res[idx] == 'i' as u8 || res[idx] == 'o' as u8 || res[idx] == 'l' as u8 {
            res[idx] += 1;
        }
        res
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use super::*;

        #[test_case("hijklmmn", true; "hijklmmn")]
        #[test_case("abbceffg", false; "abbceffg")]
        #[test_case("abbcegjk", false; "abbcegjk")]
        #[test_case("abcdffaa", true; "abcdffaa")]
        #[test_case("ghjaabcc", true; "ghjaabcc")]
        fn req_1_is_correct(password: &str, answer: bool) {
            assert_eq!(req_1(&to_chars(password)), answer);
        }

        #[test_case("hijklmmn", false; "hijklmmn")]
        #[test_case("abbceffg", true; "abbceffg")]
        #[test_case("abbcegjk", true; "abbcegjk")]
        #[test_case("abcdffaa", true; "abcdffaa")]
        #[test_case("ghjaabcc", true; "ghjaabcc")]
        fn req_2_is_correct(password: &str, answer: bool) {
            assert_eq!(req_2(&to_chars(password)), answer);
        }

        #[test_case("hijklmmn", false; "hijklmmn")]
        #[test_case("abbceffg", true; "abbceffg")]
        #[test_case("abbcegjk", false; "abbcegjk")]
        #[test_case("abcdffaa", true; "abcdffaa")]
        #[test_case("ghjaabcc", true; "ghjaabcc")]
        fn req_3_is_correct(password: &str, answer: bool) {
            assert_eq!(req_3(&to_chars(password)), answer);
        }

        #[test_case("hijklmmn", false; "hijklmmn")]
        #[test_case("abbceffg", false; "abbceffg")]
        #[test_case("abbcegjk", false; "abbcegjk")]
        #[test_case("abcdffaa", true; "abcdffaa")]
        #[test_case("ghjaabcc", true; "ghjaabcc")]
        fn is_valid_is_correct(password: &str, answer: bool) {
            assert_eq!(is_valid(&to_chars(password)), answer);
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
            let mut password = utils::to_chars(&io_utils::file_to_string(filename));
            loop {
                password = utils::increment(&password);
                if utils::is_valid(&password) { break; }
            }
            Answer::String(utils::to_string(&password))
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::String("abcdffaa".to_string()); "example_1")]
        #[test_case(2, Answer::String("ghjaabcc".to_string()); "example_2")]
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
    use crate::utils::{io_utils, solution::{Answer, Solution}};

    use super::utils;

    #[derive(Debug, Default)]
    pub struct Soln {
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            let mut password = utils::to_chars(&io_utils::file_to_string(filename));
            loop {
                password = utils::increment(&password);
                if utils::is_valid(&password) { break; }
            }
            loop {
                password = utils::increment(&password);
                if utils::is_valid(&password) { break; }
            }
            Answer::String(utils::to_string(&password))
        }
    }
}
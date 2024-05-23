#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 7 };

pub mod part_one {
    use crate::utils::{io_utils, solution::{Answer, Solution}};

    struct IP {
        address: String,
    }

    impl IP {
        fn new(address: &str) -> Self {
            Self {
                address: address.to_string(),
            }
        }

        fn supports_tls(&self) -> bool {
            let mut abba_outside_hypernet = false;
            let mut hypernet_depth: u32 = 0;
            for window in self.address.as_bytes().windows(4) {
                if window[0] as char == '[' { hypernet_depth += 1; }
                else if window[0] as char == ']' { hypernet_depth -= 1; }
                else {
                    if window[0] == window[3]
                        && window[1] == window[2] 
                        && window[0] != window[1]
                        && window[1] as char != '['
                        && window[1] as char != ']' {
                            if hypernet_depth > 0 {
                                return false;
                            } else {
                                abba_outside_hypernet = true;
                            }
                    }
                }
            }
            abba_outside_hypernet
        }
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        addresses_supporting_tls: usize,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::Usize(self.addresses_supporting_tls)
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.addresses_supporting_tls = io_utils::file_to_lines(filename)
                .map(|line| {
                    let ip = IP::new(&line);
                    ip.supports_tls()
                })
                .filter(|address_supports_tls| *address_supports_tls)
                .count() 
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(2); "example_1")]
        #[test_case(2, Answer::Usize(1); "example_2")]
        #[test_case(3, Answer::Usize(0); "example_3")]
        #[test_case(4, Answer::Usize(0); "example_4")]
        #[test_case(5, Answer::Usize(1); "example_5")]
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
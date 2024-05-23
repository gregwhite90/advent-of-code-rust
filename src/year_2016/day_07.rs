#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 7 };

mod utils {
    use std::collections::HashSet;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Mode {
        Supernet,
        Hypernet(u32),
    }

    impl Mode {
        fn increment_hn_depth(&mut self) {
            match self {
                Self::Supernet => *self = Self::Hypernet(1),
                Self::Hypernet(depth) => *depth += 1,
            }
        }

        fn decrement_hn_depth(&mut self) {
            match self {
                Self::Supernet => panic!("Attempted to decrement hypernet depth of 0."),
                Self::Hypernet(1) => *self = Self::Supernet,
                Self::Hypernet(depth) => *depth -= 1,
            }
        }
    }

    pub struct IP {
        address: String,
    }

    impl IP {
        pub fn new(address: &str) -> Self {
            Self {
                address: address.to_string(),
            }
        }

        pub fn supports_tls(&self) -> bool {
            let mut abba_outside_hypernet = false;
            let mut mode = Mode::Supernet;
            for window in self.address.as_bytes().windows(4) {
                if window[0] as char == '[' { mode.increment_hn_depth(); }
                else if window[0] as char == ']' { mode.decrement_hn_depth(); }
                else {
                    if window[0] == window[3]
                        && window[1] == window[2] 
                        && window[0] != window[1]
                        && window[1] as char != '['
                        && window[1] as char != ']' {
                            if mode == Mode::Supernet {
                                abba_outside_hypernet = true;
                            } else {
                                return false;
                            }
                    }
                }
            }
            abba_outside_hypernet
        }

        pub fn supports_ssl(&self) -> bool {
            let mut supernet_abas: HashSet<String> = HashSet::new();
            let mut hypernet_babs: HashSet<String> = HashSet::new();
            let mut mode = Mode::Supernet;
            for window in self.address.as_bytes().windows(3) {
                if window[0] as char == '[' { mode.increment_hn_depth(); }
                else if window[0] as char == ']' { mode.decrement_hn_depth(); }
                else {
                    if window[0] == window[2]
                        && window[0] != window[1]
                        && window[1] as char != '['
                        && window[1] as char != ']' {
                            let found = window.iter().map(|ch| *ch as char).collect::<String>();
                            let inverse = [window[1], window[0], window[1]].iter().map(|ch| *ch as char).collect::<String>();
                            if mode == Mode::Supernet {
                                if hypernet_babs.contains(&inverse) { return true; }
                                supernet_abas.insert(found);
                            } else {
                                if supernet_abas.contains(&inverse) { return true; }
                                hypernet_babs.insert(found);
                            }
                    }
                }
            }
            false
        }
    }
}

pub mod part_one {
    use crate::utils::{io_utils, solution::{Answer, Solution}};
    use super::utils::IP;

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

pub mod part_two {
    use crate::utils::{io_utils, solution::{Answer, Solution}};
    use super::utils::IP;

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
                    ip.supports_ssl()
                })
                .filter(|address_supports_ssl| *address_supports_ssl)
                .count() 
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(6, Answer::Usize(3); "example_6")]
        #[test_case(7, Answer::Usize(1); "example_7")]
        #[test_case(8, Answer::Usize(0); "example_8")]
        #[test_case(9, Answer::Usize(1); "example_9")]
        #[test_case(10, Answer::Usize(1); "example_10")]
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
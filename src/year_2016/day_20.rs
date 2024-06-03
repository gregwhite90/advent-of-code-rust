#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 20 };

mod utils {
    use std::{cmp::{max, Reverse}, collections::BinaryHeap};

    use lazy_static::lazy_static;
    use regex::Regex;
    use crate::utils::io_utils;

    lazy_static! {
        static ref RANGE_RE: Regex = Regex::new(r"(?<min>\d+)\-(?<max>\d+)").unwrap();
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
    struct Range {
        min: u32,
        max: u32,
    }

    impl Range {
        fn from_str(input: &str) -> Self {
            let captures = RANGE_RE.captures(input).unwrap();
            Self {
                min: captures.name("min").unwrap().as_str().parse().unwrap(),
                max: captures.name("max").unwrap().as_str().parse().unwrap(),                
            }
        }
    }

    #[derive(Debug)]
    pub struct Blacklist {
        blocked: BinaryHeap<Reverse<Range>>,
        max_allowed: u32,
    }

    impl Default for Blacklist {
        fn default() -> Self {
            Self::with_max_allowed(u32::MAX)
        }
    }

    impl Blacklist {
        pub fn with_max_allowed(max_allowed: u32) -> Self {
            Self {
                blocked: BinaryHeap::new(),
                max_allowed,
            }
        }
        pub fn parse_input_file(&mut self, filename: &str) {
            io_utils::file_to_lines(filename).for_each(|line|{
                self.blocked.push(Reverse(Range::from_str(&line)));
            });
        }

        pub fn lowest_unblocked_ip(&mut self) -> u32 {
            let mut res = 0;
            while !self.blocked.is_empty() {
                let range = self.blocked.pop().unwrap().0;
                if res < range.min { break; }
                res = max(res, range.max + 1);
            }
            res
        }

        pub fn allowed_ips(&mut self) -> u32 {
            let mut res = 0;
            let mut lowest = 0;
            while !self.blocked.is_empty() {
                let range = self.blocked.pop().unwrap().0;
                if lowest < range.min {
                    res += range.min - lowest;
                }
                if range.max == self.max_allowed { return res; }
                lowest = max(lowest, range.max + 1);
            }
            res += self.max_allowed - lowest + 1;
            res
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::Blacklist;

    #[derive(Debug, Default)]
    pub struct Soln {
        blacklist: Blacklist,
    }
    
    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.blacklist.parse_input_file(filename);
            Answer::U32(self.blacklist.lowest_unblocked_ip())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(3); "example_1")]
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
    use crate::utils::solution::{Answer, Solution};

    use super::utils::Blacklist;

    #[derive(Debug, Default)]
    pub struct Soln {
        blacklist: Blacklist,
    }
    
    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.blacklist.parse_input_file(filename);
            Answer::U32(self.blacklist.allowed_ips())
        }
    }

    impl Soln {
        #[cfg(test)]
        fn with_max_allowed(max_allowed: u32) -> Self {
            Self {
                blacklist: Blacklist::with_max_allowed(max_allowed),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(2); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::with_max_allowed(9),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}
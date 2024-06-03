#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 20 };

pub mod part_one {
    use std::{cmp::{max, Reverse}, collections::BinaryHeap};

    use lazy_static::lazy_static;
    use regex::Regex;
    use crate::utils::{io_utils, solution::{Answer, Solution}};

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

    #[derive(Debug, Default)]
    struct Blacklist {
        blocked: BinaryHeap<Reverse<Range>>,
    }

    impl Blacklist {
        fn parse_input_file(&mut self, filename: &str) {
            io_utils::file_to_lines(filename).for_each(|line|{
                self.blocked.push(Reverse(Range::from_str(&line)));
            });
        }

        fn lowest_unblocked_ip(&mut self) -> u32 {
            let mut res = 0;
            while !self.blocked.is_empty() {
                let range = self.blocked.pop().unwrap().0;
                if res < range.min { break; }
                res = max(res, range.max + 1);
            }
            res
        }
    }

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
#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 4 };

pub mod part_one {
    use std::{cmp::Ordering, collections::HashMap};

    use itertools::Itertools;
    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    fn checksum(name: &str) -> String {
        let mut counter: HashMap<char, u32> = HashMap::new();
        for ch in name.chars() {
            if ch != '-' {
                *counter.entry(ch).or_insert(0) += 1;
            }
        }
        counter.iter().sorted_by(|a, b| {
            match b.1.cmp(&a.1) {
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
                Ordering::Equal => {
                    a.0.cmp(&b.0)
                },
            }
        })
            .map(|(ch, _)| *ch)
            .take(5)
            .collect::<String>()
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        sum_of_sector_ids: u32,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.sum_of_sector_ids)
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let re = Regex::new(r"(?<name>[a-z\-]+)\-(?<sector_id>\d+)\[(?<checksum>[a-z]{5})\]").unwrap();
            io_utils::file_to_lines(filename).for_each(|line| {
                let captures = re.captures(&line).unwrap();
                let name = captures.name("name").unwrap().as_str();
                let cs = captures.name("checksum").unwrap().as_str();
                if checksum(name) == cs {
                    self.sum_of_sector_ids += captures.name("sector_id").unwrap().as_str().parse::<u32>().unwrap();
                }
            });
        }   
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(1_514); "example_1")]
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
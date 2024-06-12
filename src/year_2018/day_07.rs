#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 7 };

mod utils {
    use std::collections::{HashMap, HashSet};

    use regex::Regex;   

    use crate::utils::io_utils;

    #[derive(Debug, Default)]
    pub struct SleighBuilder {
        prereqs: HashMap<char, HashSet<char>>,
    }

    impl SleighBuilder {
        pub fn parse_input_file(&mut self, filename: &str) {
            let line_re = Regex::new(r"Step (?<prereq>[A-Z]) must be finished before step (?<step>[A-Z]) can begin\.").unwrap();
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    let captures = line_re.captures(&line).unwrap();
                    let prereq = captures.name("prereq").unwrap().as_str().chars().next().unwrap();
                    let step = captures.name("step").unwrap().as_str().chars().next().unwrap();
                    self.prereqs.entry(step).or_default().insert(prereq);
                    if !self.prereqs.contains_key(&prereq) {
                        self.prereqs.insert(prereq, HashSet::new());
                    }
                });
        }

        pub fn order(&mut self) -> String {
            let mut order = String::new();
            while !self.prereqs.is_empty() {
                let next_char = *self.prereqs.iter().filter(|(_ch, prereqs)| {
                    prereqs.is_empty()
                })
                    .map(|(ch, _prereqs)| ch)
                    .min()
                    .unwrap();
                self.prereqs.remove(&next_char);
                self.prereqs.values_mut().for_each(|prereqs| { prereqs.remove(&next_char); });
                order.push(next_char);
            }
            order
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::SleighBuilder;

    #[derive(Debug, Default)]
    pub struct Soln {
        sleigh_builder: SleighBuilder,    
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.sleigh_builder.parse_input_file(filename);
            Answer::String(self.sleigh_builder.order())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::String("CABDFE".to_string()); "example_1")]
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
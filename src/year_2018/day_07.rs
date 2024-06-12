#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 7 };

mod utils {
    use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}};

    use itertools::Itertools;
    use regex::Regex;   

    use crate::utils::io_utils;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Step {
        end_time: usize,
        name: char,
    }

    #[derive(Debug)]
    pub struct SleighBuilder {
        prereqs: HashMap<char, HashSet<char>>,
        workers: usize,
        fixed_time: usize,
    }

    impl Default for SleighBuilder {
        fn default() -> Self {
            Self::new(5, 60)
        }
    }

    impl SleighBuilder {
        pub fn new(workers: usize, fixed_time: usize) -> Self {
            Self {
                prereqs: HashMap::new(),
                workers,
                fixed_time,
            }
        }

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

        pub fn time(&mut self) -> usize {
            let mut jobs: BinaryHeap<Reverse<Step>> = BinaryHeap::new();
            let mut time = 0;
            while !self.prereqs.is_empty() {
                let next_chars = self.prereqs.iter().filter(|(_ch, prereqs)| {
                    prereqs.is_empty()
                })
                    .map(|(ch, _prereqs)| *ch)
                    .sorted()
                    .take(self.workers - jobs.len());
                next_chars.for_each(|ch| {
                    self.prereqs.remove(&ch);
                    jobs.push(
                        Reverse(Step { 
                            end_time: time + self.fixed_time + (ch as u8 - 'A' as u8) as usize + 1, 
                            name: ch,
                        })
                    );
                });
                let finished_job = jobs.pop().unwrap().0;
                self.prereqs.values_mut().for_each(|prereqs| { prereqs.remove(&finished_job.name); });
                time = finished_job.end_time;
            }
            while !jobs.is_empty() {
                let job = jobs.pop().unwrap().0;
                time = job.end_time;
            }
            time
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

pub mod part_two {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::SleighBuilder;

    #[derive(Debug, Default)]
    pub struct Soln {
        sleigh_builder: SleighBuilder,    
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.sleigh_builder.parse_input_file(filename);
            Answer::Usize(self.sleigh_builder.time())
        }
    }

    impl Soln {
        #[cfg(test)]
        fn with_workers_and_fixed_time(workers: usize, fixed_time: usize) -> Self {
            Self {
                sleigh_builder: SleighBuilder::new(workers, fixed_time),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(15); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::with_workers_and_fixed_time(2, 0),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}
#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 8 };

mod utils {
    #[derive(Debug, PartialEq, Eq)]
    pub enum Instruction {
        L,
        R,
    }

    impl Instruction {
        pub fn from_char(ch: char) -> Self {
            match ch {
                'L' => Self::L,
                'R' => Self::R,
                _ => panic!("Unrecognized direction."),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct Node {
        val: String,
        l: String,
        r: String,
    }

    impl Node {
        pub fn new(val: &str, l: &str, r: &str) -> Self {
            Self {
                val: String::from(val),
                l: String::from(l),
                r: String::from(r),
            }
        }

        pub fn next(&self, instruction: &Instruction) -> &str {
            match instruction {
                Instruction::L => &self.l,
                Instruction::R => &self.r,
            }
        }
    }
}

pub mod part_one {
    use std::collections::HashMap;
    use regex::Regex;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    use super::utils::{Instruction, Node};

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        instructions: Vec<Instruction>,
        nodes: HashMap<String, Node>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.steps_to_reach_node_val(
                0,
                "AAA",
                "ZZZ"
            ))
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let node_re = Regex::new(r"(?<val>[A-Z]{3}) = \((?<l>[A-Z]{3}), (?<r>[A-Z]{3})\)").unwrap();
            let mut lines = io_utils::file_to_lines(filename);
            self.instructions = lines.next().unwrap()
                .chars()
                .map(|ch| Instruction::from_char(ch))
                .collect();
            assert!(lines.next().unwrap().len() == 0);
            lines.for_each(|line| {
                let captures = node_re.captures(&line).unwrap();
                let val = captures.name("val").unwrap().as_str();
                let l = captures.name("l").unwrap().as_str();
                let r = captures.name("r").unwrap().as_str();
                let node = Node::new(val, l, r);
                self.nodes.insert(String::from(val), node);
            });
        }

        fn steps_to_reach_node_val(
            &self, 
            start_instructions_idx: usize,
            start_node_val: &str, 
            end_node_val: &str,
        ) -> u32 {
            let mut instruction_idx = start_instructions_idx;
            let mut steps: u32 = 0;
            let mut cur_node_val: &str = start_node_val;
            while cur_node_val != end_node_val {
                let cur_node = self.nodes.get(cur_node_val).unwrap();
                cur_node_val = cur_node.next(&self.instructions[instruction_idx]);
                instruction_idx = (instruction_idx + 1) % self.instructions.len();
                steps += 1;
            }
            steps
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(6); "example_1")]
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
    // This solution would work (it works on the small example case), but it is very slow.
    // It needs to be short-circuited by finding a repeat loop.
    use std::{cmp, collections::HashMap};
    use regex::Regex;
    use prime_factorization::Factorization;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    use super::utils::{Instruction, Node};

    #[derive(Debug, PartialEq, Eq)]
    struct Path {
        start_node: String,
        current_node: String,
        ending_condition_statuses: Vec<EndingConditionStatus>,
        period: Option<u64>,
    }

    impl Path {
        fn new(node: &str) -> Self {
            Self {
                start_node: String::from(node),
                current_node: String::from(node),
                ending_condition_statuses: Vec::new(),
                period: None,
            }
        }

        fn add_ending_condition_status(&mut self, ecs: EndingConditionStatus) {
            if self.period != None { return; }
            if let Some(first_ecs) = self.ending_condition_statuses.get(0) {
                if first_ecs.node == ecs.node 
                    && first_ecs.instructions_idx == ecs.instructions_idx 
                    && ecs.steps > first_ecs.steps 
                    && ecs.steps % first_ecs.steps == 0 {
                        self.period = Some(first_ecs.steps);
                }
            }
            self.ending_condition_statuses.push(ecs);
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct EndingConditionStatus {
        node: String,
        instructions_idx: usize,
        steps: u64,
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        instructions: Vec<Instruction>,
        instructions_idx: usize,
        nodes: HashMap<String, Node>,
        paths: Vec<Path>,
        steps: u64,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U64(self.steps_to_reach_node_val())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let node_re = Regex::new(r"(?<val>[A-Z0-9]{3}) = \((?<l>[A-Z0-9]{3}), (?<r>[A-Z0-9]{3})\)").unwrap();
            let start_node_val_re = Regex::new(r"[A-Z0-9]{2}A").unwrap();
            let mut lines = io_utils::file_to_lines(filename);
            self.instructions = lines.next().unwrap()
                .chars()
                .map(|ch| Instruction::from_char(ch))
                .collect();
            assert!(lines.next().unwrap().len() == 0);
            lines.for_each(|line| {
                let captures = node_re.captures(&line).unwrap();
                let val = captures.name("val").unwrap().as_str();
                let l = captures.name("l").unwrap().as_str();
                let r = captures.name("r").unwrap().as_str();
                let node = Node::new(val, l, r);
                self.nodes.insert(String::from(val), node);
                if start_node_val_re.is_match(&val) {
                    self.paths.push(Path::new(val));
                }
            });
        }

        fn steps_to_reach_node_val(&mut self) -> u64 {
            while self.paths.iter().any(|path| path.period == None) {
                self.steps += 1;
                self.advance_paths();
                self.instructions_idx = (self.instructions_idx + 1) % self.instructions.len();
            }
            self.calculate_shared_period()
        }

        fn advance_paths(&mut self) {
            // TODO: could calculate only once
            let end_node_val_re = Regex::new(r"[A-Z0-9]{2}Z").unwrap();
            for i in 0..self.paths.len() {
                let path = self.paths.get_mut(i).unwrap();
                let cur_node = self.nodes.get(&path.current_node).unwrap();
                let next_node = cur_node.next(&self.instructions[self.instructions_idx]);
                path.current_node = String::from(next_node);
                if end_node_val_re.is_match(next_node) {
                    let ecs = EndingConditionStatus {
                        node: String::from(next_node),
                        instructions_idx: self.instructions_idx,
                        steps: self.steps,
                    };
                    path.add_ending_condition_status(ecs);
                }
            }
        }

        /// Calculates the least common multiple of the periods of all the paths.
        fn calculate_shared_period(&self) -> u64 {
            let periods = self.paths.iter()
                .map(|path| path.period.expect("Should only be called when all paths have a calculated period."));
            let period_factors_count = periods.clone()
                .map(|period| {
                    let factors = Factorization::run(period).factors;
                    let mut factors_count: HashMap<u64, u64> = HashMap::new();
                    for factor in factors {
                        factors_count.entry(factor).and_modify(|count| *count += 1).or_insert(1);
                    }
                    factors_count
                });
            let common_divisors = period_factors_count.reduce(|common_divisors, factors_count| {
                let mut cd = HashMap::new();
                for factor in common_divisors.keys() {
                    if factors_count.contains_key(factor) {
                        cd.insert(*factor, *cmp::min(common_divisors.get(factor).unwrap(), factors_count.get(factor).unwrap()));
                    }
                }
                cd
            }).unwrap();
            // TODO: share functionality.
            let greatest_common_divisor: u64 = common_divisors.iter().fold(1, |acc, (divisor, count)| acc * divisor.pow((*count).try_into().unwrap()));
            periods.map(|period| period / greatest_common_divisor).product::<u64>() * greatest_common_divisor
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(2, Answer::U64(6); "example_2")]
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
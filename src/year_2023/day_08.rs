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

        pub fn val(&self) -> &str {
            &self.val
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
    use std::collections::HashMap;
    use regex::Regex;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    use super::utils::{Instruction, Node};

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        instructions: Vec<Instruction>,
        nodes: HashMap<String, Node>,
        starting_node_vals: Vec<String>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.steps_to_reach_node_val(
                0,
            ))
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let node_re = Regex::new(r"(?<val>[A-Z0-9]{3}) = \((?<l>[A-Z0-9]{3}), (?<r>[A-Z0-9]{3})\)").unwrap();
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
        ) -> u32 {
            let mut instruction_idx = start_instructions_idx;
            let mut steps: u32 = 0;
            let start_node_val_re = Regex::new(r"[A-Z0-9]{2}A").unwrap();
            let end_node_val_re = Regex::new(r"[A-Z0-9]{2}Z").unwrap();
            let mut paths: Vec<String> = self.nodes.keys()
                .filter(|val| {
                    start_node_val_re.is_match(&val)
                })
                .map(|val| val.clone())
                .collect();
            while paths.iter().any(|val| {
                !end_node_val_re.is_match(self.nodes.get(val).unwrap().val())
            }) {
                paths = paths.iter().map(|cur_node_val| {
                    let cur_node = self.nodes.get(cur_node_val).unwrap();
                    String::from(
                        cur_node.next(&self.instructions[instruction_idx])
                    )
                }).collect();
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

        #[test_case(2, Answer::U32(6); "example_2")]
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
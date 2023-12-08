#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 8 };

pub mod part_one {
    use std::collections::HashMap;
    use regex::Regex;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    #[derive(Debug, PartialEq, Eq)]
    enum Instruction {
        L,
        R,
    }

    impl Instruction {
        fn from_char(ch: char) -> Self {
            match ch {
                'L' => Self::L,
                'R' => Self::R,
                _ => panic!("Unrecognized direction."),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Node {
        val: String,
        l: String,
        r: String,
    }

    impl Node {
        fn next(&self, instruction: &Instruction) -> &str {
            match instruction {
                Instruction::L => &self.l,
                Instruction::R => &self.r,
            }
        }
    }

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
                let val = String::from(captures.name("val").unwrap().as_str());
                let l = String::from(captures.name("l").unwrap().as_str());
                let r = String::from(captures.name("r").unwrap().as_str());
                let node = Node { val: val.clone(), l, r };
                self.nodes.insert(val, node);
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
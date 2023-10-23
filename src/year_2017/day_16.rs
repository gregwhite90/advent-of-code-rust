#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 16 };

pub mod part_one {
    use std::collections::VecDeque;
    use regex::Regex;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    const BASE_USV: u32 = 97;

    #[derive(Debug, PartialEq, Eq)]
    pub enum Instruction {
        Spin(usize),
        Exchange(usize, usize),
        Partner(char, char),
    }

    #[derive(Debug)]
    pub struct Soln {
        deq: VecDeque<char>,
    }

    impl Default for Soln {
        fn default() -> Self {
            Soln::with_len(16)
        }
    }

    impl Soln {
        pub fn with_len(len: u32) -> Self {
            Self {
                deq: (0..len).map(|num| char::from_u32(num + BASE_USV).unwrap()).collect(),
            }
        }

        pub fn parse_input_file(&mut self, filename: &str) {
            let re = Regex::new(r"^(?<operation>[sxp])(?<args>[\d/a-p]+)$").unwrap();
            io_utils::file_to_string(filename).split(",").for_each(|instr| {
                let captures = re.captures(&instr)
                    .expect("Line should match regex.");
                let operation = captures.name("operation").unwrap().as_str();
                let args = captures.name("args").unwrap().as_str();
                let instruction = match operation {
                    "s" => Instruction::Spin(args.parse().unwrap()),
                    "x" => {
                        let mut args = args.split("/").map(|arg| arg.parse().unwrap());
                        Instruction::Exchange(args.next().unwrap(), args.next().unwrap())
                    },
                    "p" => {
                        let mut args = args.split("/");
                        Instruction::Partner(
                            args.next().unwrap().chars().next().unwrap(), 
                            args.next().unwrap().chars().next().unwrap()
                        )
                    },
                    _ => panic!("Found unrecognized instruction: {operation}"),
                };
                self.handle_instruction(instruction);
            });    
        }

        pub fn handle_instruction(&mut self, instruction: Instruction) {
            match instruction {
                Instruction::Spin(len) => {
                    self.deq.rotate_right(len);
                },
                Instruction::Exchange(left, right) => {
                    self.deq.swap(left, right);
                },
                Instruction::Partner(left, right) => {
                    self.deq.swap(
                        self.deq.iter().position(|&x| x == left).unwrap(), 
                        self.deq.iter().position(|&x| x == right).unwrap()
                    );
                },
            }
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::String(self.deq.iter().collect())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::String(String::from("baedc")); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::with_len(5),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}
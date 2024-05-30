#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 12 };

mod utils {
    use std::collections::HashMap;

    use regex::Regex;

    use crate::utils::io_utils;

    #[derive(Debug, Default)]
    pub struct AssembunnyComputer {
        instruction_ptr: i64,
        registers: HashMap<char, i64>,
        instructions: Vec<Instruction>,
    }

    impl AssembunnyComputer {
        pub fn with_registers(registers: HashMap<char, i64>) -> Self {
            Self {
                instruction_ptr: 0,
                registers,
                instructions: Vec::new(),
            }
        }

        pub fn parse_input_file(&mut self, filename: &str) {
            let re = Regex::new(r"(?<instruction>(cpy)|(inc)|(dec)|(jnz)) (?<x>[a-d]|\d+)( (?<y>[a-d]|(\-?\d+)))?").unwrap();
            self.instructions = io_utils::file_to_lines(filename).map(|line| {
                let captures = re.captures(&line).unwrap();
                let instruction = captures.name("instruction").unwrap().as_str();
                match instruction {
                    "cpy" => {
                        let x = captures.name("x").unwrap().as_str();
                        let res = x.parse();
                        let x = if res.is_ok() { Argument::Value(res.unwrap()) } else { Argument::Register(x.chars().next().unwrap()) };
                        let y = captures.name("y").unwrap().as_str().chars().next().unwrap();
                        Instruction::Cpy(x, y)
                    },
                    "inc" => {
                        let register = captures.name("x").unwrap().as_str().chars().next().unwrap();
                        Instruction::Inc(register)
                    },
                    "dec" => {
                        let register = captures.name("x").unwrap().as_str().chars().next().unwrap();
                        Instruction::Dec(register)                        
                    },
                    "jnz" => {
                        let x = captures.name("x").unwrap().as_str();
                        let res = x.parse();
                        let x = if res.is_ok() { Argument::Value(res.unwrap()) } else { Argument::Register(x.chars().next().unwrap()) };
                        let y = captures.name("y").unwrap().as_str();
                        let res = y.parse();
                        let y = if res.is_ok() { Argument::Value(res.unwrap()) } else { Argument::Register(y.chars().next().unwrap()) };
                        Instruction::Jnz(x, y)
                    },
                    _ => panic!("Unrecognized instruction."),
                }
            }).collect();
        }

        fn execute(&mut self, instruction: Instruction) {
            match instruction {
                Instruction::Cpy(arg, register) => {
                    self.registers.insert(register, self.arg_to_value(&arg));
                    self.instruction_ptr += 1;
                },
                Instruction::Inc(register) => {
                    *self.registers.entry(register).or_default() += 1;
                    self.instruction_ptr += 1;
                },
                Instruction::Dec(register) => {
                    *self.registers.entry(register).or_default() -= 1;
                    self.instruction_ptr += 1;
                },
                Instruction::Jnz(x, y) => {
                    let x = self.arg_to_value(&x);
                    let y = self.arg_to_value(&y);
                    if x != 0 {
                        self.instruction_ptr += y;
                    } else {
                        self.instruction_ptr += 1;
                    }
                },
            }
        }

        fn arg_to_value(&self, arg: &Argument) -> i64 {
            match *arg {
                Argument::Value(val) => val,
                Argument::Register(register) => self.register_value(register),
            }
        }

        pub fn execute_all(&mut self) {
            while self.instruction_ptr < self.instructions.len() as i64 {
                let idx: usize = self.instruction_ptr.try_into().unwrap();
                self.execute(self.instructions[idx]);
            }
        }

        pub fn register_value(&self, register: char) -> i64 {
            self.registers.get(&register).cloned().unwrap_or_default()
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Instruction {
        Cpy(Argument, char),
        Inc(char),
        Dec(char),
        Jnz(Argument, Argument),
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Argument {
        Value(i64),
        Register(char),
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::AssembunnyComputer;

    #[derive(Debug, Default)]
    pub struct Soln {
        assembunny_computer: AssembunnyComputer,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer{
            self.assembunny_computer.parse_input_file(filename);
            self.assembunny_computer.execute_all();
            Answer::I64(self.assembunny_computer.register_value('a'))
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::I64(42); "example_1")]
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
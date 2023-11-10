#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 23 };

// TODO: abstract to shared utilities?
mod utils {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum ArgType {
        Char(char),
        I64(i64),
    }
}

pub mod part_one {
    use std::collections::HashMap;

    use regex::Regex;

    use crate::utils::{solution::{Solution, Answer}, io_utils};
    use super::utils::ArgType;

    #[derive(Debug, PartialEq, Eq)]
    enum Instruction {
        Set(char, ArgType),
        Sub(char, ArgType),
        Mul(char, ArgType),
        Jnz(ArgType, ArgType),
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        instructions: Vec<Instruction>,
        position: i64,
        registers: HashMap<char, i64>,
        muls: u32,
    }

    impl Soln {
        pub fn parse_input_file(&mut self, filename: &str) {
            let re = Regex::new(r"(?<operation>[a-z]{3}) (?<args>[ a-z\-\d]+)").unwrap();
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    let captures = re.captures(&line)
                        .expect("Line should match regex.");
                    let operation = captures.name("operation").unwrap().as_str();
                    let args = captures.name("args").unwrap().as_str();
                    let instruction = match operation {
                        "set" => {
                            let mut args = args.split(" ");
                            let arg_1 = args.next().unwrap().chars().next().unwrap();
                            let arg_2 = args.next().unwrap();
                            if let Ok(val) = arg_2.parse() {
                                Instruction::Set(
                                    arg_1,
                                    ArgType::I64(val),
                                )
                            } else {
                                Instruction::Set(
                                    arg_1,
                                    ArgType::Char(arg_2.chars().next().unwrap()),
                                )
                            }                
                        },
                        "sub" => {
                            let mut args = args.split(" ");
                            let arg_1 = args.next().unwrap().chars().next().unwrap();
                            let arg_2 = args.next().unwrap();
                            if let Ok(val) = arg_2.parse() {
                                Instruction::Sub(
                                    arg_1,
                                    ArgType::I64(val),
                                )
                            } else {
                                Instruction::Sub(
                                    arg_1,
                                    ArgType::Char(arg_2.chars().next().unwrap()),
                                )
                            }                
                        },
                        "mul" => {
                            let mut args = args.split(" ");
                            let arg_1 = args.next().unwrap().chars().next().unwrap();
                            let arg_2 = args.next().unwrap();
                            if let Ok(val) = arg_2.parse() {
                                Instruction::Mul(
                                    arg_1,
                                    ArgType::I64(val),
                                )
                            } else {
                                Instruction::Mul(
                                    arg_1,
                                    ArgType::Char(arg_2.chars().next().unwrap()),
                                )
                            }                
                        },
                        "jnz" => {
                            let mut args = args.split(" ");
                            let arg_1 = args.next().unwrap();
                            let val_1 = if let Ok(val) = arg_1.parse() {
                                ArgType::I64(val)
                            } else {
                                ArgType::Char(arg_1.chars().next().unwrap())
                            };
                            let arg_2 = args.next().unwrap();
                            let val_2 = if let Ok(val) = arg_2.parse() {
                                ArgType::I64(val)
                            } else {
                                ArgType::Char(arg_2.chars().next().unwrap())
                            };
                            Instruction::Jnz(
                                val_1,
                                val_2,
                            )
                        },
                        _ => panic!("Unrecognized operation: {operation}"),
                    };
                    self.instructions.push(instruction);
                });
        }

        fn handle_next_instruction(&mut self) -> bool {
            if self.position < 0 || self.position as usize >= self.instructions.len() {
                return true;
            }
            let instruction = &self.instructions[self.position as usize];
            match instruction {
                Instruction::Set(register, arg) => {
                    let val = self.get_value(*arg);
                    self.registers.insert(*register, val);
                },
                Instruction::Sub(register, arg) => {
                    let val = self.get_value(*arg);
                    self.registers.entry(*register)
                        .and_modify(|e| *e -= val)
                        .or_insert(val);
                },
                Instruction::Mul(register, arg) => {
                    self.muls += 1;
                    let val = self.get_value(*arg);
                    self.registers.entry(*register)
                        .and_modify(|e| *e *= val)
                        .or_insert(0);
                },
                Instruction::Jnz(arg_1, arg_2) => {
                    let val_1 = self.get_value(*arg_1);
                    let val_2 = self.get_value(*arg_2);
                    if val_1 != 0 { 
                        self.position += val_2 - 1;
                    }
                },
            };
            self.position += 1;
            false
        }

        // TODO: abstract to shared utilities?
        fn get_value(&self, arg: ArgType) -> i64 {
            match arg {
                ArgType::Char(register) => {
                    match self.registers.get(&register) {
                        None => 0,
                        Some(val) => *val,
                    }
                },
                ArgType::I64(value) => {
                    value
                }
            }
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            while !self.handle_next_instruction() {}
            Answer::U32(self.muls)
        }
    }
}
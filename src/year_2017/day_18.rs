#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 18 };

pub mod part_one {
    use std::collections::HashMap;
    use either::Either;

    use regex::Regex;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    #[derive(Debug, PartialEq, Eq)]
    enum Instruction {
        Snd(Either<char, i64>),
        Set(char, Either<char, i64>),
        Add(char, Either<char, i64>),
        Mul(char, Either<char, i64>),
        Mod(char, Either<char, i64>),
        Rcv(Either<char, i64>),
        Jgz(Either<char, i64>, Either<char, i64>),
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        instructions: Vec<Instruction>,
        position: i64,
        last_sound: i64,
        registers: HashMap<char, i64>,
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct InstructionResult {
        finished: bool,
        recovered_value: Option<i64>,
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
                        "snd" => {
                            if let Ok(val) = args.parse() {
                                Instruction::Snd(Either::Right(val))
                            } else {
                                Instruction::Snd(Either::Left(args.chars().next().unwrap()))
                            }                
                        },
                        "set" => {
                            let mut args = args.split(" ");
                            let arg_1 = args.next().unwrap().chars().next().unwrap();
                            let arg_2 = args.next().unwrap();
                            if let Ok(val) = arg_2.parse() {
                                Instruction::Set(
                                    arg_1,
                                    Either::Right(val),
                                )
                            } else {
                                Instruction::Set(
                                    arg_1,
                                    Either::Left(arg_2.chars().next().unwrap()),
                                )
                            }                
                        },
                        "add" => {
                            let mut args = args.split(" ");
                            let arg_1 = args.next().unwrap().chars().next().unwrap();
                            let arg_2 = args.next().unwrap();
                            if let Ok(val) = arg_2.parse() {
                                Instruction::Add(
                                    arg_1,
                                    Either::Right(val),
                                )
                            } else {
                                Instruction::Add(
                                    arg_1,
                                    Either::Left(arg_2.chars().next().unwrap()),
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
                                    Either::Right(val),
                                )
                            } else {
                                Instruction::Mul(
                                    arg_1,
                                    Either::Left(arg_2.chars().next().unwrap()),
                                )
                            }                
                        },
                        "mod" => {
                            let mut args = args.split(" ");
                            let arg_1 = args.next().unwrap().chars().next().unwrap();
                            let arg_2 = args.next().unwrap();
                            if let Ok(val) = arg_2.parse() {
                                Instruction::Mod(
                                    arg_1,
                                    Either::Right(val),
                                )
                            } else {
                                Instruction::Mod(
                                    arg_1,
                                    Either::Left(arg_2.chars().next().unwrap()),
                                )
                            }                
                        },
                        "rcv" => {
                            if let Ok(val) = args.parse() {
                                Instruction::Rcv(
                                    Either::Right(val),
                                )
                            } else {
                                Instruction::Rcv(
                                    Either::Left(args.chars().next().unwrap()),
                                )
                            }                
                        },
                        "jgz" => {
                            let mut args = args.split(" ");
                            let arg_1 = args.next().unwrap();
                            let val_1: Either<char, i64> = if let Ok(val) = arg_1.parse() {
                                Either::Right(val)
                            } else {
                                Either::Left(arg_1.chars().next().unwrap())
                            };
                            let arg_2 = args.next().unwrap();
                            let val_2: Either<char, i64> = if let Ok(val) = arg_2.parse() {
                                Either::Right(val)
                            } else {
                                Either::Left(arg_2.chars().next().unwrap())
                            };
                            Instruction::Jgz(
                                val_1,
                                val_2,
                            )
                        },
                        _ => panic!("Unrecognized operation: {operation}"),
                    };
                    self.instructions.push(instruction);
                });
        }

        fn handle_next_instruction(&mut self) -> InstructionResult {
            if self.position < 0 || self.position as usize >= self.instructions.len() {
                return InstructionResult {
                    finished: true,
                    recovered_value: None,
                }
            }
            let finished = false;
            let mut recovered_value: Option<i64> = None;
            let instruction = &self.instructions[self.position as usize];
            match instruction {
                Instruction::Snd(arg) => {
                    self.last_sound = self.get_value(*arg);
                    self.position += 1;
                    InstructionResult {
                        finished,
                        recovered_value,
                    }
                },
                Instruction::Set(register, arg) => {
                    let val = self.get_value(*arg);
                    self.registers.insert(*register, val);
                    self.position += 1;
                    InstructionResult {
                        finished,
                        recovered_value,
                    }
                },
                Instruction::Add(register, arg) => {
                    let val = self.get_value(*arg);
                    self.registers.entry(*register)
                        .and_modify(|e| *e += val)
                        .or_insert(val);
                    self.position += 1;
                    InstructionResult {
                        finished,
                        recovered_value,
                    }
                },
                Instruction::Mul(register, arg) => {
                    let val = self.get_value(*arg);
                    self.registers.entry(*register)
                        .and_modify(|e| *e *= val)
                        .or_insert(0);
                    self.position += 1;
                    InstructionResult {
                        finished,
                        recovered_value,
                    }
                },
                Instruction::Mod(register, arg) => {
                    let val = self.get_value(*arg);
                    self.registers.entry(*register)
                        .and_modify(|e| *e %= val)
                        .or_insert(0);
                    self.position += 1;
                    InstructionResult {
                        finished,
                        recovered_value,
                    }
                },
                Instruction::Rcv(arg) => {
                    let val = self.get_value(*arg);
                    if val != 0 { recovered_value = Some(self.last_sound); }
                    self.position += 1;
                    InstructionResult {
                        finished,
                        recovered_value,
                    }
                },
                Instruction::Jgz(arg_1, arg_2) => {
                    let val_1 = self.get_value(*arg_1);
                    let val_2 = self.get_value(*arg_2);
                    if val_1 > 0 { 
                        self.position += val_2;
                    } else {
                        self.position += 1;
                    }
                    InstructionResult {
                        finished,
                        recovered_value,
                    }
                },
            }
        }

        fn get_value(&self, arg: Either<char, i64>) -> i64 {
            match arg {
                Either::Left(register) => {
                    *self.registers.get(&register).unwrap()
                },
                Either::Right(value) => {
                    value
                }
            }
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            loop {
                let result = self.handle_next_instruction();
                if let Some(val) = result.recovered_value {
                    return Answer::I64(val);
                }
                if result.finished { 
                    panic!("Finished without finding a recovered value.") 
                }   
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::I64(4); "example_1")]
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
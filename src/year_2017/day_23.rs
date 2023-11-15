#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const _DAY: Day = crate::utils::Day { year: 2017, day: 23 };

// TODO: abstract to shared utilities with day 18?
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

pub mod part_two {
    //! Actually executing the instructions here is infeasible, so this is a "decompiling" approach to
    //! solving the problem. It turns out this is just counting primes in a specific range, stepping
    //! by a specific amount.
    //! 
    //! The decompiling steps to understand that are as follows.
    //! The solution is the value of the register `h` when the program terminates. The instructions are:
    //! 
    //! | Instruction ID | Instruction  | Control flow notes and questions |
    //! |----------------|--------------|----------------------------------|
    //! |  `0` | `set b 79`             | |
    //! |  `1` | `set c b`              | |
    //! |  `2` | `jnz a 2`              | |
    //! |  `3` | `jnz 1 5`              | |
    //! |  `4` | `mul b 100`            | |
    //! |  `5` | `sub b -100000`        | |
    //! |  `6` | `set c b`              | |
    //! |  `7` | `sub c -17000`         | Constant registers: `a = 1`, `c = 124_900`. `b = 107_900` to start, incremented by `17` at the end of each iteration of Loop 0. |
    //! |  `8` | `set f 1`              | Loop 0 starts. |
    //! |  `9` | `set d 2`              | |
    //! | `10` | `set e 2`              | Loop 1 starts. |
    //! | `11` | `set g d`              | Loop 2 starts. |
    //! | `12` | `mul g e`              | |
    //! | `13` | `sub g b`              | |
    //! | `14` | `jnz g 2`              | Conditional within Loop 2. |
    //! | `15` | `set f 0`              | When does `f` get set to `0` for the rest of the iteration of Loop 1 and 2 (it gets reset to `1` at the start of each Loop 0 iteration)? |
    //! | `16` | `sub e -1`             | |
    //! | `17` | `set g e`              | |
    //! | `18` | `sub g b`              | |
    //! | `19` | `jnz g -8`             | Loop 2 ends. |
    //! | `20` | `sub d -1`             | |
    //! | `21` | `set g d`              | |
    //! | `22` | `sub g b`              | |
    //! | `23` | `jnz g -13`            | Loop 1 ends. |
    //! | `24` | `jnz f 2`              | Conditional within Loop 0. |
    //! | `25` | `sub h -1`             | **This is the answer.** How many times is this instruction run (`f` needs to be `0` in iteration of Loop 0 for this to run)? |
    //! | `26` | `set g b`              | **To exit (`g == 0` at instruction 28), need `b == c`.** |
    //! | `27` | `sub g c`              | |
    //! | `28` | `jnz g 2`              | To exit, need `g == 0` here. |
    //! | `29` | `jnz 1 3`              | If this instruction is executed, the program terminates. |
    //! | `30` | `sub b -17`            | |
    //! | `31` | `jnz 1 -23`            | Loop 0 ends. |
    //! 
    //! The answer is the number of times Loop 1 exits with `f = 0` by the time it exits with values that meet the instructions 26-29 criteria to terminate the program.
    //! If `f` is ever set to `0` at instruction 15 at any point in Loop 2 for any iteration of Loop 1 for a given iteration of Loop 0, it remains `0`.
    //! 
    //! The program terminates if Loop 0 finishes an iteration with `b == c`. 
    //! `c` is `17_000` higher than `b`'s starting value (instruction 7), 
    //! and `c` does not change and `b` only changes by being incremented by `17` at the end of each Loop 0 iteration (instruction 30).
    //! 
    //! In Loop 2, we set `g = d * e - b` (instructions 11-13), and if `g == 0`, we set `f` to 1. So as soon as we find `d` and `e` such that `d * e == b`,
    //! `h` will be incremented that iteration of Loop 0. For each iteration of Loop 0, `d` and `e` each start at `2` and are incremented at the end of their
    //! respective loops. So this is really looking for any factorization of `b` other than `1 * b`. The answer is the count of the numbers in the sequence of
    //! `b` with the specified step that are not prime.
    use prime_factorization::Factorization;
    use crate::utils::solution::{Solution, Answer};

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {}

    impl Solution for Soln {
        fn solve(&mut self, _filename: &str) -> Answer {
            let c = 124_900_u32;
            let mut b = 107_900_u32;
            let step = 17;
            let mut h = 0;
            while b != c + step {
                let factor_repr = Factorization::run(b);
                if !factor_repr.is_prime {
                    h += 1;
                }
                b += step;
            }
            Answer::U32(h)
        }
    }
}
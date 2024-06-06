//! A collection of utilities used by multiple 2016 days' solutions.

/// An Assembunny computer (required by solutions to
/// days 12 and 23 (and maybe 25)).
pub mod assembunny_computer {
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
            let re = Regex::new(r"(?<instruction>(cpy)|(inc)|(dec)|(jnz)|(tgl)) (?<x>[a-d]|\-?\d+)( (?<y>[a-d]|(\-?\d+)))?").unwrap();
            self.instructions = io_utils::file_to_lines(filename).map(|line| {
                let captures = re.captures(&line).unwrap();
                let instruction = captures.name("instruction").unwrap().as_str();
                match instruction {
                    "cpy" => {
                        let x = captures.name("x").unwrap().as_str();
                        let res = x.parse();
                        let x = if res.is_ok() { Argument::Value(res.unwrap()) } else { Argument::Register(x.chars().next().unwrap()) };
                        let y = captures.name("y").unwrap().as_str().chars().next().unwrap();
                        Instruction::Cpy(x, Argument::Register(y))
                    },
                    "inc" => {
                        let register = captures.name("x").unwrap().as_str().chars().next().unwrap();
                        Instruction::Inc(Argument::Register(register))
                    },
                    "dec" => {
                        let register = captures.name("x").unwrap().as_str().chars().next().unwrap();
                        Instruction::Dec(Argument::Register(register))                        
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
                    "tgl" => {
                        let x = captures.name("x").unwrap().as_str();
                        let res = x.parse();
                        let x = if res.is_ok() { Argument::Value(res.unwrap()) } else { Argument::Register(x.chars().next().unwrap()) };
                        Instruction::Tgl(x)
                    },
                    _ => panic!("Unrecognized instruction."),
                }
            }).collect();
        }

        fn execute(&mut self, instruction: Instruction) {
            match instruction {
                Instruction::Cpy(arg, register) => {
                    if let Argument::Register(register) = register {
                        self.registers.insert(register, self.arg_to_value(&arg));
                    }
                    self.instruction_ptr += 1;
                },
                Instruction::Inc(register) => {
                    if let Argument::Register(register) = register {
                        *self.registers.entry(register).or_default() += 1;
                    }
                    self.instruction_ptr += 1;
                },
                Instruction::Dec(register) => {
                    if let Argument::Register(register) = register {
                        *self.registers.entry(register).or_default() -= 1;
                    }
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
                Instruction::Tgl(x) => {
                    let x = self.arg_to_value(&x);
                    if let Ok(idx) = TryInto::<usize>::try_into(self.instruction_ptr as i64 + x) {
                        if idx < self.instructions.len() {
                            self.instructions[idx].toggle();
                        }
                    }
                    self.instruction_ptr += 1;
                }
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
        Cpy(Argument, Argument),
        Inc(Argument),
        Dec(Argument),
        Jnz(Argument, Argument),
        Tgl(Argument),
    }

    impl Instruction {
        fn toggle(&mut self) {
            *self = match self {
                Self::Cpy(x, y) => Self::Jnz(*x, *y),
                Self::Jnz(x, y) =>  Self::Cpy(*x, *y),
                Self::Inc(x) =>  Self::Dec(*x),
                Self::Dec(x) | Self::Tgl(x) => Self::Inc(*x),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Argument {
        Value(i64),
        Register(char),
    }
}
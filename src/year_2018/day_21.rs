mod utils {
    use std::collections::HashSet;

    use regex::Regex;
    use strum_macros::EnumIter;

    use crate::utils::io_utils;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Part {
        One,
        Two,
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter)]
    pub enum Operation {
        ADDR(usize, usize, usize),
        ADDI(usize, usize, usize),
        MULR(usize, usize, usize),
        MULI(usize, usize, usize),
        BANR(usize, usize, usize),
        BANI(usize, usize, usize),
        BORR(usize, usize, usize),
        BORI(usize, usize, usize),
        SETR(usize, usize, usize),
        SETI(usize, usize, usize),
        GTIR(usize, usize, usize),
        GTRI(usize, usize, usize),
        GTRR(usize, usize, usize),
        EQIR(usize, usize, usize),
        EQRI(usize, usize, usize),
        EQRR(usize, usize, usize),
    }

    impl Operation {
        fn execute(&self, registers: &mut Vec<usize>) {
            match self {
                Self::ADDR(a, b, c) => {
                    registers[*c] = registers[*a] + registers[*b];
                },
                Self::ADDI(a, b, c) => {
                    registers[*c] = registers[*a] + *b;
                },
                Self::MULR(a, b, c) => {
                    registers[*c] = registers[*a] * registers[*b];
                },
                Self::MULI(a, b, c) => {
                    registers[*c] = registers[*a] * *b;
                },
                Self::BANR(a, b, c) => {
                    registers[*c] = registers[*a] & registers[*b];
                },
                Self::BANI(a, b, c) => {
                    registers[*c] = registers[*a] & *b;
                },
                Self::BORR(a, b, c) => {
                    registers[*c] = registers[*a] | registers[*b];
                },
                Self::BORI(a, b, c) => {
                    registers[*c] = registers[*a] | *b;
                },
                Self::SETR(a, _b, c) => {
                    registers[*c] = registers[*a];
                },
                Self::SETI(a, _b, c) => {
                    registers[*c] = *a;
                },
                Self::GTIR(a, b, c) => {
                    registers[*c] = if *a > registers[*b] { 1 } else { 0 }
                },
                Self::GTRI(a, b, c) => {
                    registers[*c] = if registers[*a] > *b { 1 } else { 0 }
                },
                Self::GTRR(a, b, c)=> {
                    registers[*c] = if registers[*a] > registers[*b] { 1 } else { 0 }
                },
                Self::EQIR(a, b, c) => {
                    registers[*c] = if *a == registers[*b] { 1 } else { 0 }
                },
                Self::EQRI(a, b, c) => {
                    registers[*c] = if registers[*a] == *b { 1 } else { 0 }
                },
                Self::EQRR(a, b, c) => {
                    registers[*c] = if registers[*a] == registers[*b] { 1 } else { 0 }
                },
            }
        }

        pub fn from_str(line: &str) -> Self {
            let args: Vec<&str> = line.split_whitespace().collect();
            match args[0] {
                "addr" => Self::ADDR(args[1].parse().unwrap(), args[2].parse().unwrap(), args[3].parse().unwrap()),
                "addi" => Self::ADDI(args[1].parse().unwrap(), args[2].parse().unwrap(), args[3].parse().unwrap()),
                "mulr" => Self::MULR(args[1].parse().unwrap(), args[2].parse().unwrap(), args[3].parse().unwrap()),
                "muli" => Self::MULI(args[1].parse().unwrap(), args[2].parse().unwrap(), args[3].parse().unwrap()),
                "banr" => Self::BANR(args[1].parse().unwrap(), args[2].parse().unwrap(), args[3].parse().unwrap()),
                "bani" => Self::BANI(args[1].parse().unwrap(), args[2].parse().unwrap(), args[3].parse().unwrap()),
                "borr" => Self::BORR(args[1].parse().unwrap(), args[2].parse().unwrap(), args[3].parse().unwrap()),
                "bori" => Self::BORI(args[1].parse().unwrap(), args[2].parse().unwrap(), args[3].parse().unwrap()),
                "setr" => Self::SETR(args[1].parse().unwrap(), args[2].parse().unwrap(), args[3].parse().unwrap()),
                "seti" => Self::SETI(args[1].parse().unwrap(), args[2].parse().unwrap(), args[3].parse().unwrap()),
                "gtir" => Self::GTIR(args[1].parse().unwrap(), args[2].parse().unwrap(), args[3].parse().unwrap()),
                "gtri" => Self::GTRI(args[1].parse().unwrap(), args[2].parse().unwrap(), args[3].parse().unwrap()),
                "gtrr" => Self::GTRR(args[1].parse().unwrap(), args[2].parse().unwrap(), args[3].parse().unwrap()),
                "eqir" => Self::EQIR(args[1].parse().unwrap(), args[2].parse().unwrap(), args[3].parse().unwrap()),
                "eqri" => Self::EQRI(args[1].parse().unwrap(), args[2].parse().unwrap(), args[3].parse().unwrap()),
                "eqrr" => Self::EQRR(args[1].parse().unwrap(), args[2].parse().unwrap(), args[3].parse().unwrap()),
                _ => panic!("Unrecognized operation."),
            }
        }
    }

    #[derive(Debug)]
    pub struct CPU {
        instruction_ptr: usize,
        instruction_ptr_register: usize,
        instructions: Vec<Operation>,
        registers: Vec<usize>,
    }

    impl Default for CPU {
        fn default() -> Self {
            Self {
                instruction_ptr: 0,
                instruction_ptr_register: 0,
                instructions: Vec::default(),
                registers: vec![0; 6],
            }
        }
    }

    impl CPU {
        pub fn parse_input_file(&mut self, filename: &str) {
            let ipr_re = Regex::new(r"\#ip (?<ipr>[0-5])").unwrap();
            io_utils::file_to_lines(filename).for_each(|line| {
                if let Some(caps) = ipr_re.captures(&line) {
                    self.instruction_ptr_register = caps.name("ipr").unwrap().as_str().parse().unwrap();
                } else {
                    self.instructions.push(Operation::from_str(&line));
                }
            })
        }

        pub fn register_value(&self, register: usize) -> usize {
            self.registers[register]
        }

        pub fn run_program(&mut self, part: Part) {
            let mut last_register_2_value: Option<usize> = None;
            let mut seen_register_2: HashSet<usize> = HashSet::new();
            loop {
                if self.instruction_ptr >= self.instructions.len() {
                    break;
                }
                if self.instruction_ptr == 28 {
                    // This is the result of decompiling the instructions.
                    // This instruction is the first time register 0 is referenced.
                    // For part one, if register 0 == register 2, we will break out of the
                    // program after the minimum number of instructions.
                    // For part two, we want to find the first time register 2 repeats itself,
                    // the solution that maximizes the number of instructions is the
                    // previous value of register 2.
                    match part {
                        Part::One => {
                            self.set_register(0, self.register_value(2));
                        },
                        Part::Two => {
                            if !seen_register_2.insert(self.register_value(2)) {
                                self.set_register(0, last_register_2_value.unwrap());
                            } else {
                                last_register_2_value = Some(self.register_value(2));
                            }
                        },
                    }
                }
                self.registers[self.instruction_ptr_register] = self.instruction_ptr;
                self.instructions[self.instruction_ptr].execute(&mut self.registers);
                self.instruction_ptr = self.registers[self.instruction_ptr_register];
                self.instruction_ptr += 1;
            }
        }

        pub fn set_register(&mut self, register: usize, value: usize) {
            self.registers[register] = value;
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::{CPU, Part};

    #[derive(Debug, Default)]
    pub struct Soln {
        cpu: CPU,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.cpu.parse_input_file(filename);
            self.cpu.run_program(Part::One);
            Answer::Usize(self.cpu.register_value(0))
        }
    }
}

pub mod part_two {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::{CPU, Part};

    #[derive(Debug, Default)]
    pub struct Soln {
        cpu: CPU,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.cpu.parse_input_file(filename);
            self.cpu.run_program(Part::Two);
            Answer::Usize(self.cpu.register_value(0))
        }
    }
}
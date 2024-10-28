#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 19 };

mod utils {
    use regex::Regex;
    use strum_macros::EnumIter;

    use crate::utils::io_utils;

    /**
     * TODO: clarify the instruction.
     */

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

        pub fn run_program(&mut self) {
            // TODO: deal with a negative instruction pointer?
            // TODO? account for when there is no instruction pointer register?
            loop {
                if self.instruction_ptr >= self.instructions.len() {
                    break;
                }
                if self.instruction_ptr == 1 {
                    // We know this from decompiling the instructions.
                    // The loop beginning at instruction 1 calculates 
                    // and stores in register 0 the
                    // cumulative sum of the multiplicative factors of the
                    // value in register 1 at the start of the loop.
                    self.registers[0] = sum_of_factors(self.registers[1]);
                    break;
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

    pub fn sum_of_factors(n: usize) -> usize {
        let mut sum_of_factors = 0;
        let mut i: usize = 1;
        loop {
            if i.pow(2) == n { sum_of_factors += i; }
            else if n % i == 0 {
                sum_of_factors += i;
                sum_of_factors += n / i;
            }
            if i.pow(2) >= n { break; }
            i += 1;
        }
        sum_of_factors
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::{self, CPU};

    #[derive(Debug, Default)]
    pub struct Soln {
        cpu: CPU,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.cpu.parse_input_file(filename);
            self.cpu.run_program();
            Answer::Usize(self.cpu.register_value(0))
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test]
        fn decompiled_is_correct() {
            assert_eq!(2_821, utils::sum_of_factors(900));
        }

        #[test_case(1, Answer::Usize(6); "example_1")]
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
    use crate::utils::solution::{Answer, Solution};

    use super::utils::CPU;

    #[derive(Debug, Default)]
    pub struct Soln {
        cpu: CPU,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.cpu.parse_input_file(filename);
            self.cpu.set_register(0, 1);
            self.cpu.run_program();
            Answer::Usize(self.cpu.register_value(0))
        }
    }
}
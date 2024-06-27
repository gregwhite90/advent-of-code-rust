#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 16 };

mod utils {
    use std::collections::{HashMap, HashSet};

    use strum::IntoEnumIterator;
    use strum_macros::EnumIter;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter)]
    pub enum Operation {
        ADDR,
        ADDI,
        MULR,
        MULI,
        BANR,
        BANI,
        BORR,
        BORI,
        SETR,
        SETI,
        GTIR,
        GTRI,
        GTRR,
        EQIR,
        EQRI,
        EQRR,
    }

    impl Operation {
        fn execute(&self, registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
            match self {
                Self::ADDR => {
                    registers[c] = registers[a] + registers[b];
                },
                Self::ADDI => {
                    registers[c] = registers[a] + b;
                },
                Self::MULR => {
                    registers[c] = registers[a] * registers[b];
                },
                Self::MULI => {
                    registers[c] = registers[a] * b;
                },
                Self::BANR => {
                    registers[c] = registers[a] & registers[b];
                },
                Self::BANI => {
                    registers[c] = registers[a] & b;
                },
                Self::BORR => {
                    registers[c] = registers[a] | registers[b];
                },
                Self::BORI => {
                    registers[c] = registers[a] | b;
                },
                Self::SETR => {
                    registers[c] = registers[a];
                },
                Self::SETI => {
                    registers[c] = a;
                },
                Self::GTIR => {
                    registers[c] = if a > registers[b] { 1 } else { 0 }
                },
                Self::GTRI => {
                    registers[c] = if registers[a] > b { 1 } else { 0 }
                },
                Self::GTRR => {
                    registers[c] = if registers[a] > registers[b] { 1 } else { 0 }
                },
                Self::EQIR => {
                    registers[c] = if a == registers[b] { 1 } else { 0 }
                },
                Self::EQRI => {
                    registers[c] = if registers[a] == b { 1 } else { 0 }
                },
                Self::EQRR => {
                    registers[c] = if registers[a] == registers[b] { 1 } else { 0 }
                },
            }
        }

        pub fn matching_operations(
            before: &Vec<usize>,
            a: usize,
            b: usize,
            c: usize,
            after: &Vec<usize>,
        ) -> HashSet<Self> {
            Self::iter().filter(|op| {
                let mut registers = before.clone();
                op.execute(&mut registers, a, b, c);
                registers == *after
            })
                .collect()
        }
    }

    #[derive(Debug)]
    pub struct CPU {
        opcodes: HashMap<usize, HashSet<Operation>>,
        registers: Vec<usize>,
    }

    impl Default for CPU {
        fn default() -> Self {
            Self {
                opcodes: (0..16).map(|opcode| {
                    (opcode, HashSet::from_iter(Operation::iter()))
                }).collect(),
                registers: vec![0; 4],
            }
            
        }
    }

    impl CPU {
        // TODO: pass in another vec instead of all the individual args?
        pub fn process_sample(
            &mut self,             
            before: &Vec<usize>,
            opcode: usize,
            a: usize,
            b: usize,
            c: usize,
            after: &Vec<usize>,
        ) {
            let matching = Operation::matching_operations(
                before,
                a,
                b,
                c,
                after,
            );
            self.opcodes.get_mut(&opcode).unwrap().retain(|op| matching.contains(op));
        }

        pub fn fully_determine_opcodes(&mut self) {
            // Iteratively reduce the sets of possible operations for each opcode
            // by the operations that are fully determinatively assigned to one opcode.
            // After the first call to perform_operation, this is trivial.
            let mut fully_determined = HashSet::new();
            self.opcodes.values().filter(|possible| possible.len() == 1)
                .for_each(|possible| {
                    possible.iter().for_each(|op| { fully_determined.insert(*op); });
                });
            while self.opcodes.values().any(|possible| possible.len() > 1) {
                self.opcodes.values_mut()
                .filter(|possible| possible.len() > 1)
                .for_each(|possible| {
                    possible.retain(|op| !fully_determined.contains(op));
                    if possible.len() == 1 {
                        possible.iter().for_each(|op| { fully_determined.insert(*op); });
                    }
                });
            }
        }

        pub fn perform_operation(&mut self, operation: Vec<usize>) {
            self.fully_determine_opcodes();
            let opcode = operation[0];
            let possible_operations = self.opcodes.get(&opcode).unwrap();
            assert_eq!(1, possible_operations.len());
            let op = possible_operations.iter().next().unwrap();
            op.execute(
                &mut self.registers, 
                operation[1],
                operation[2],
                operation[3], 
            );
        }

        pub fn register_value(&self, register: usize) -> usize {
            self.registers[register]
        }
    }
}

pub mod part_one {
    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    use super::utils::Operation;

    #[derive(Debug, Default)]
    pub struct Soln {}

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            let mut result = 0;
            let mut before = None;
            let mut operation: Option<Vec<usize>> = None;
            let mut after = None;
            let registers_re = Regex::new(r"(?<seq>(Before)|(After)):\s+\[(?<registers>[\d\, ]+)\]").unwrap();
            io_utils::file_to_lines(filename).for_each(|line| {
                if line.len() == 0 {
                    if let Some(af) = &after {
                        assert_ne!(before, None);
                        if Operation::matching_operations(
                            &before.as_ref().unwrap(), 
                            operation.as_ref().unwrap()[1], 
                            operation.as_ref().unwrap()[2], 
                            operation.as_ref().unwrap()[3], 
                            &af
                        ).len() >= 3 {
                            result += 1;
                        }
                    }
                    before = None;
                    operation = None;
                    after = None;
                } else if let Some(captures) = registers_re.captures(&line) {
                    let seq = captures.name("seq").unwrap().as_str();
                    let registers = captures.name("registers").unwrap().as_str().split(", ")
                        .map(|val| val.parse().unwrap())
                        .collect();
                    match seq {
                        "Before" => before = Some(registers),
                        "After"  => after  = Some(registers),
                        _ => panic!("Unrecognized sequence"),
                    }
                } else {
                    operation = Some(line.split(" ").map(|val| val.parse().unwrap()).collect());
                }
            });
            if let Some(af) = &after {
                assert_ne!(before, None);
                if Operation::matching_operations(
                    &before.as_ref().unwrap(), 
                    operation.as_ref().unwrap()[1], 
                    operation.as_ref().unwrap()[2], 
                    operation.as_ref().unwrap()[3], 
                    &af
                ).len() >= 3 {
                    result += 1;
                }
            }
            Answer::Usize(result)
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(1); "example_1")]
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
    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    use super::utils::CPU;

    #[derive(Debug, Default)]
    pub struct Soln {
        cpu: CPU,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            let mut before = None;
            let mut operation: Option<Vec<usize>> = None;
            let mut after = None;
            let registers_re = Regex::new(r"(?<seq>(Before)|(After)):\s+\[(?<registers>[\d\, ]+)\]").unwrap();
            io_utils::file_to_lines(filename).for_each(|line| {
                if line.len() == 0 {
                    if let Some(af) = &after {
                        assert_ne!(before, None);
                        // NOTE: changed from part one
                        self.cpu.process_sample(
                            &before.as_ref().unwrap(),
                            operation.as_ref().unwrap()[0],
                            operation.as_ref().unwrap()[1], 
                            operation.as_ref().unwrap()[2], 
                            operation.as_ref().unwrap()[3], 
                            &af
                        );
                        before = None;
                        operation = None;
                        after = None;                        
                    }
                } else if let Some(captures) = registers_re.captures(&line) {
                    let seq = captures.name("seq").unwrap().as_str();
                    let registers = captures.name("registers").unwrap().as_str().split(", ")
                        .map(|val| val.parse().unwrap())
                        .collect();
                    match seq {
                        "Before" => before = Some(registers),
                        "After"  => after  = Some(registers),
                        _ => panic!("Unrecognized sequence"),
                    }
                } else {
                    let op = line.split(" ").map(|val| val.parse().unwrap()).collect();
                    if before == None {
                        // perform operation
                        self.cpu.perform_operation(op);
                    } else {
                        operation = Some(op);
                    }
                }
            });
            Answer::Usize(self.cpu.register_value(0))
        }
    }
}
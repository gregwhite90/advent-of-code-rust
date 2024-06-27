#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 16 };

mod utils {
    use std::collections::HashSet;

    use strum::IntoEnumIterator;
    use strum_macros::EnumIter;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
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
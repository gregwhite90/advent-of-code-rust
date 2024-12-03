#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2024, day: 3 };

mod utils {
    use std::{cmp::Reverse, collections::BinaryHeap};

    use regex::Regex;

    use crate::utils::io_utils;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    enum Instruction {
        Mul(usize, usize),
        Do,
        Dont,
    }

    #[derive(Debug, Default)]
    pub struct CPU {
        instructions: BinaryHeap<Reverse<(usize, usize, Instruction)>>,
    }

    impl CPU {
        pub fn parse_input_file(&mut self, filename: &str) {
            let mul_re = Regex::new(r"mul\((?<operand_0>\d{1,3})\,(?<operand_1>\d{1,3})\)").unwrap();
            let do_re = Regex:: new(r"do\(\)").unwrap();
            let dont_re = Regex:: new(r"don't\(\)").unwrap();
            io_utils::file_to_lines(filename)
                .enumerate()
                .for_each(|(line_num, line)| {
                    self.instructions.extend(
                        mul_re.captures_iter(&line)
                            .map(|caps| {
                                let operand_0: usize = caps.name("operand_0").unwrap().as_str().parse().unwrap();
                                let operand_1: usize = caps.name("operand_1").unwrap().as_str().parse().unwrap();
                                let idx = caps.get(0).unwrap().start();
                                Reverse((line_num, idx, Instruction::Mul(operand_0, operand_1)))
                            })
                    );
                    self.instructions.extend(
                        do_re.find_iter(&line)
                            .map(|m| {
                                let idx = m.start();
                                Reverse((line_num, idx, Instruction::Do))
                            })
                    );
                    self.instructions.extend(
                        dont_re.find_iter(&line)
                            .map(|m| {
                                let idx = m.start();
                                Reverse((line_num, idx, Instruction::Dont))
                            })
                    );
                });
        }

        pub fn execute_instructions(&mut self) -> usize {
            let mut res = 0;
            let mut enabled = true;
            while !self.instructions.is_empty() {
                let Reverse((_line_num, _idx, instruction)) = self.instructions.pop().unwrap();
                match instruction {
                    Instruction::Do => enabled = true,
                    Instruction::Dont => enabled = false,
                    Instruction::Mul(operand_0, operand_1) => if enabled { res += operand_0 * operand_1; },
                }
            }
            res
        }
    }
}

pub mod part_one {
    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, Default)]
    pub struct Soln {}

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            let mul_re = Regex::new(r"mul\((?<operand_0>\d{1,3})\,(?<operand_1>\d{1,3})\)").unwrap();
            Answer::Usize(
                io_utils::file_to_lines(filename)
                    .map(|line| {
                        mul_re.captures_iter(&line)
                            .map(|caps| {
                                let operand_0: usize = caps.name("operand_0").unwrap().as_str().parse().unwrap();
                                let operand_1: usize = caps.name("operand_1").unwrap().as_str().parse().unwrap();
                                operand_0 * operand_1
                            })
                            .sum::<usize>()
                    })
                    .sum()
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(161); "example_1")]
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
            Answer::Usize(self.cpu.execute_instructions())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(2, Answer::Usize(48); "example_2")]
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
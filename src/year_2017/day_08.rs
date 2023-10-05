#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 8 };
// TODO: shared functionality
mod utils {
    use std::{str::FromStr, collections::HashMap, cmp};
    use regex::Regex;
    use crate::utils::io_utils;

    pub enum Operation {
        Increase,
        Decrease,
    }

    impl FromStr for Operation {
        type Err = (); // TODO: implement
        fn from_str(input: &str) -> Result<Operation, Self::Err> {
            match input {
                "inc" => Ok(Operation::Increase),
                "dec" => Ok(Operation::Decrease),
                _ => Err(()),
            }
        }
    }

    pub enum ComparisonOperator {
        GreaterThan,
        LessThan,
        GreaterThanOrEqual,
        LessThanOrEqual,
        Equal,
        NotEqual,
    }

    impl FromStr for ComparisonOperator {
        type Err = (); // TODO: implement
        fn from_str(input: &str) -> Result<ComparisonOperator, Self::Err> {
            match input {
                ">" => Ok(ComparisonOperator::GreaterThan),
                "<" => Ok(ComparisonOperator::LessThan),
                ">=" => Ok(ComparisonOperator::GreaterThanOrEqual),
                "<=" => Ok(ComparisonOperator::LessThanOrEqual),
                "==" => Ok(ComparisonOperator::Equal),
                "!=" => Ok(ComparisonOperator::NotEqual),
                _ => Err(()),
            }
        }
    }

    pub struct Instruction { // TODO: use lifetimes to share &strs?
        pub register: String,
        pub operation: Operation,
        pub value: i32,
        pub comparison_register: String,
        pub comparison_operator: ComparisonOperator,
        pub comparison_value: i32,
    }

    impl Instruction {
        pub fn from(line: &str) -> Self {
            let re = Regex::new(
                r"(?<register>[a-z]+) (?<operation>inc|dec) (?<value>\-?[0-9]+) if (?<comparison_register>[a-z]+) (?<comparison_operator><|>|<=|>=|==|!=) (?<comparison_value>\-?[0-9]+)"
            ).unwrap();
            let caps = re.captures(line).expect("Line should match the regex.");
            Instruction {
                register: String::from(caps.name("register").unwrap().as_str()),
                operation: Operation::from_str(caps.name("operation").unwrap().as_str()).expect("Operation should be convertible to enum."),
                value: caps.name("value").unwrap().as_str().parse().expect("Value should be convertible to signed integer."),
                comparison_register: String::from(caps.name("comparison_register").unwrap().as_str()),
                comparison_operator: ComparisonOperator::from_str(caps.name("comparison_operator").unwrap().as_str()).expect("Comparison operator should be convertible to enum."),
                comparison_value: caps.name("comparison_value").unwrap().as_str().parse().expect("Comparison value should be convertible to signed integer."),
            }           
        }
    }

    pub fn parse_input_file(
        registers: &mut HashMap<String, i32>,
        max_register_value: &mut i32,
        filename: &str
    ) {
        *max_register_value = i32::MIN;
        io_utils::file_to_lines(filename).for_each(|line| {
            let instruction = Instruction::from(&line);
            let comparison_register = registers
                .entry(instruction.comparison_register.clone())
                .or_insert(0);
            let comparison: bool = match instruction.comparison_operator {
                ComparisonOperator::GreaterThan => *comparison_register > instruction.comparison_value,
                ComparisonOperator::LessThan => *comparison_register < instruction.comparison_value,
                ComparisonOperator::GreaterThanOrEqual => *comparison_register >= instruction.comparison_value,
                ComparisonOperator::LessThanOrEqual => *comparison_register <= instruction.comparison_value,
                ComparisonOperator::Equal => *comparison_register == instruction.comparison_value,
                ComparisonOperator::NotEqual => *comparison_register != instruction.comparison_value,
            };
            if !comparison { return; }
            let added_value = match instruction.operation { Operation::Increase => 1, Operation::Decrease => -1 } * instruction.value;
            let register_value = registers
                .entry(instruction.register.clone())
                .and_modify(|register_val| {
                    *register_val += added_value;
                })
                .or_insert(added_value); // Default = 0, then add added_value
            *max_register_value = cmp::max(*max_register_value, *register_value);
        });
    }
}

pub mod part_one {
    use std::collections::HashMap;
    use crate::utils::solution::{Solution, Answer};
    use super::utils;

    #[derive(Default)]
    pub struct Soln {
        registers: HashMap<String, i32>,
    }
 
    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            let mut _max_register_value = i32::MIN;
            utils::parse_input_file(
                &mut self.registers,
                &mut _max_register_value,
                filename
            );
            Answer::I32(
                self.registers
                    .iter()
                    .map(|(_name, &val)| val)
                    .max()
                    .expect("There should be at least one register.")
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::I32(1); "example_1")]
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
    use std::collections::HashMap;
    use crate::utils::solution::{Solution, Answer};
    use super::utils;

    #[derive(Default)]
    pub struct Soln {
        registers: HashMap<String, i32>,
        max_register_value: i32,
    }
 
    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            utils::parse_input_file(&mut self.registers, &mut self.max_register_value, filename);
            Answer::I32(self.max_register_value)
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::I32(10); "example_1")]
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
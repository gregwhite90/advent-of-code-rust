#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2024, day: 7 };

mod utils {
    use std::collections::VecDeque;

    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        static ref EQUATION_RE: Regex = Regex::new(r"(?<test_value>\d+): (?<operands>[\d ]+)").unwrap();
    }

    pub struct Equation {
        test_value: usize,
        operands: VecDeque<usize>,
    }

    impl Equation {
        pub fn from_str(input: &str) -> Self {
            let caps = EQUATION_RE.captures(input).unwrap();
            let test_value = caps.name("test_value").unwrap().as_str().parse().unwrap();
            let operands = caps.name("operands").unwrap().as_str().split(' ').map(|num| num.parse().unwrap()).collect();
            Self {
                test_value,
                operands,
            }
        }

        pub fn is_possibly_true(&mut self, concatenation_operator_avail: bool) -> bool {
            if self.operands.iter().any(|operand| *operand > self.test_value) { return false; }
            if self.operands.len() == 1 {
                return self.operands[0] == self.test_value;
            }
            let op_1 = self.operands.pop_front().unwrap();
            let op_2 = self.operands.pop_front().unwrap();
            let mut add_operands = self.operands.clone();
            add_operands.push_front(op_1 + op_2);
            let mut add_equation = Equation {
                test_value: self.test_value,
                operands: add_operands,
            };
            let mut mul_operands = self.operands.clone();
            mul_operands.push_front(op_1 * op_2);
            let mut mul_equation = Equation {
                test_value: self.test_value,
                operands: mul_operands,
            };
            if !concatenation_operator_avail {
                return add_equation.is_possibly_true(concatenation_operator_avail) || mul_equation.is_possibly_true(concatenation_operator_avail);
            } else {
                let mut concat_operands = self.operands.clone();
                concat_operands.push_front(format!("{}{}", op_1, op_2).parse().unwrap());
                let mut concat_equation = Equation {
                    test_value: self.test_value,
                    operands: concat_operands,
                };
                return add_equation.is_possibly_true(concatenation_operator_avail) 
                    || mul_equation.is_possibly_true(concatenation_operator_avail)
                    || concat_equation.is_possibly_true(concatenation_operator_avail);
            }
        }

        pub fn test_value(&self) -> usize {
            self.test_value
        }
    }
}

pub mod part_one {
    use crate::utils::{io_utils, solution::{Answer, Solution}};

    use super::utils::Equation;

    #[derive(Debug, Default)]
    pub struct Soln {}

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            Answer::Usize(
                io_utils::file_to_lines(filename)
                    .filter_map(|line| {
                        let mut equation = Equation::from_str(&line);
                        if equation.is_possibly_true(false) {
                            Some(equation.test_value())
                        } else {
                            None
                        }
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

        #[test_case(1, Answer::Usize(3749); "example_1")]
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
    use crate::utils::{io_utils, solution::{Answer, Solution}};

    use super::utils::Equation;

    #[derive(Debug, Default)]
    pub struct Soln {}

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            Answer::Usize(
                io_utils::file_to_lines(filename)
                    .filter_map(|line| {
                        let mut equation = Equation::from_str(&line);
                        if equation.is_possibly_true(true) {
                            Some(equation.test_value())
                        } else {
                            None
                        }
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

        #[test_case(1, Answer::Usize(11387); "example_1")]
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
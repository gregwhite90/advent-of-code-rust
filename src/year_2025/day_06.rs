#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2025, day: 6 };

mod utils {
    use crate::utils::io_utils;

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    enum Operation {
        Add,
        Multiply,
    }

    impl Operation {
        fn from_str(input: &str) -> Self {
            match input {
                "+" => Self::Add,
                "*" => Self::Multiply,
                other => panic!("Unrecognized Operation: {:?}", other),
            }
        }
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct Problems {
        totals: Vec<i64>,
        operations: Vec<Operation>,
    }

    impl Problems {
        pub fn parse_input_file(&mut self, filename: &str) {
            let mut lines = io_utils::file_to_lines(filename);
            self.totals = Self::parse_problem_line(&lines.next().unwrap()).unwrap();
            self.operations = lines.last().unwrap()
                .trim()
                .split_whitespace()
                .map(|op| Operation::from_str(op))
                .collect();
            io_utils::file_to_lines(filename)
                .skip(1)
                .for_each(|line| {
                    if let Some(problem_line) = Self::parse_problem_line(&line) {
                        for ((total, new), op) in self.totals.iter_mut().zip(problem_line.iter()).zip(self.operations.iter()) {
                            match *op {
                                Operation::Add => *total += *new,
                                Operation::Multiply => *total *= *new,
                            }
                        }
                    }
                });
            
        }

        fn parse_problem_line(line: &str) -> Option<Vec<i64>> {
            // TODO: maybe could be implemented to return an iterator, but would need to figure out lifetimes
            let res: Vec<Result<i64, _>> = line
                .trim() 
                .split_whitespace()
                .map(|num| num.parse::<i64>())
                .collect();
            match res[0] {
                Ok(_) => Some(
                    res.into_iter()
                        .map(|num| {
                            num.unwrap()
                        })
                        .collect()
                ),
                Err(_) => None,
            }
        }

        pub fn sum_all(&self) -> i64 {
            self.totals.iter().sum()
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use super::*;

        #[test_case(
            "*   +   *   +  ",
            None;
            "operations_line"
        )]
        #[test_case(
            " 45 64  387 23 ",
            Some(vec![45, 64, 387, 23]);
            "numbers_line"
        )]
        fn parse_problem_line_is_correct(
            line: &str,
            expected: Option<Vec<i64>>,
        ) {
            assert_eq!(
                Problems::parse_problem_line(line),
                expected,
            );
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};
    use super::utils::Problems;

    #[derive(Debug, Default)]
    pub struct Soln {
        problems: Problems
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.problems.parse_input_file(filename);
            Answer::I64(self.problems.sum_all())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::I64(4277556); "example_1")]
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
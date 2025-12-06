#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2025, day: 6 };

mod utils {
    use ndarray::Array2;

    use crate::utils::io_utils;

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    enum Operation {
        Add,
        Multiply,
    }

    impl Operation {
        fn from_str(input: &str) -> Option<Self> {
            match input {
                "+" => Some(Self::Add),
                "*" => Some(Self::Multiply),
                _ => None,
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
                .map(|op| Operation::from_str(op).unwrap())
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

     #[derive(Debug, Default, PartialEq, Eq)]
    struct CephalopodProblem {
        op: Option<Operation>,
        nums: Vec<u64>,
        finished: bool,
    }

    impl CephalopodProblem {
        fn parse(&mut self, col: &[char]) {
            if let Some(op) = Operation::from_str(&col[col.len() - 1].to_string()) {
                self.op = Some(op);
            }
            self.nums.push(
                col[..col.len() - 1].iter()
                    .collect::<String>()
                    .trim()
                    .parse::<u64>()
                    .unwrap()
            );
        }

        fn finish(&mut self) -> u64 {
            self.finished = true;
            match self.op {
                None => panic!("Can't finish a cephalopod problem without an operation."),
                Some(Operation::Add) => self.nums.iter().sum(),
                Some(Operation::Multiply) => self.nums.iter().product(),
            }
        }
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct CephalopodProblems {
        total: u64,
    }

    impl CephalopodProblems {
        pub fn parse_input_file(&mut self, filename: &str) {
            let grid: Vec<Vec<char>> = io_utils::file_to_lines(filename)
                .map(|line| line.chars().collect())
                .collect();
            let rows = grid.len();
            let cols = grid[0].len();
            let grid: Array2<char> = Array2::from_shape_vec(
                (rows, cols),
                grid.into_iter().flatten().collect(),
            ).unwrap();
            let mut cephalopod_problem = CephalopodProblem::default();
            for col in (0..cols).rev() {
                if grid.column(col).iter().all(|ch| *ch == ' ' || *ch == '\0') {
                    self.total += cephalopod_problem.finish();
                    cephalopod_problem = CephalopodProblem::default();
                    continue;
                }
                cephalopod_problem.parse(
                    &grid.column(col).iter()
                    .cloned()
                    .collect::<Vec<char>>()
                );
            }
            if !cephalopod_problem.finished { 
                self.total += cephalopod_problem.finish();
            }
        }

        pub fn total(&self) -> u64 {
            self.total
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

    /**
     * Approach for part two:
     * 
     * Read all into memory in an ndarray of characters.
     * If all ' ', finish problem, add to running total.
     * If last is an operation, store the operation.
     * Combine all the numbers in order from top to bottom to parse into a number (unsigned).
     * Keep a running vec of the numbers in a problem until the problem is finished.
     */
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

pub mod part_two {
    use crate::utils::solution::{Answer, Solution};
    use super::utils::CephalopodProblems;

    #[derive(Debug, Default)]
    pub struct Soln {
        problems: CephalopodProblems,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.problems.parse_input_file(filename);
            Answer::U64(self.problems.total())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U64(3263827); "example_1")]
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
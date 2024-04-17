#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 14 };

pub mod part_one {

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct Soln {
        total_north_load_inv: usize,
        total_round_rocks: usize,
        rows: usize,
        northernmost_idx: Vec<usize>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::Usize(self.total_north_load())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    if self.northernmost_idx.is_empty() {
                        self.northernmost_idx = vec![0; line.len()]
                    }
                    for (idx, ch) in line.char_indices() {
                        match ch {
                            'O' => {
                                self.total_north_load_inv += self.northernmost_idx[idx];
                                self.northernmost_idx[idx] += 1;
                                self.total_round_rocks += 1;
                            },
                            '#' => {
                                self.northernmost_idx[idx] = self.rows + 1;
                            },
                            '.' => {},
                            _ => panic!("Unrecognized character.")
                        }
                    }
                    self.rows += 1;
                });
        }

        fn total_north_load(&self) -> usize {
            self.rows * self.total_round_rocks - self.total_north_load_inv
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(136); "example_1")]
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

    use std::collections::{BTreeSet, HashMap, HashSet};

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    struct Point {
        row: usize,
        col: usize,
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct Soln {
        rows: usize,
        cols: usize,
        square_rocks: HashSet<Point>,
        round_rocks: BTreeSet<Point>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            self.spin_n_times(1_000_000_000);
            Answer::Usize(self.total_north_load())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    self.cols = line.len();
                    for (col, ch) in line.char_indices() {
                        match ch {
                            'O' => {
                                self.round_rocks.insert(Point { row: self.rows, col });
                            },
                            '#' => {
                                self.square_rocks.insert(Point { row: self.rows, col });
                            },
                            '.' => {},
                            _ => panic!("Unrecognized character.")
                        }
                    }
                    self.rows += 1;
                });
        }

        fn spin_n_times(&mut self, n: usize) {
            // This short-circuits when the same state is reached. We 
            // then calculate the period on which the states are
            // repeating, and jump to the end state after the
            // required (large, infeasible to actually brute-force-calculate)
            // number of iterations.
            let mut cache = HashMap::from([(self.round_rocks.clone(), vec![0])]);
            for i in 0..n {
                self.spin_once();
                cache.entry(self.round_rocks.clone())
                    .and_modify(|iters| iters.push(i + 1))
                    .or_insert(vec![i + 1]);
                let iters = cache.get(&self.round_rocks).unwrap();
                if iters.len() == 2 {
                    let period = iters[1] - iters[0];
                    let remaining_iterations = (n - iters[1]) % period;
                    for _ in 0..remaining_iterations {
                        self.spin_once();
                    }
                    break;
                }
            }
        }

        fn spin_once(&mut self) {
            self.tilt_north();
            self.tilt_west();
            self.tilt_south();
            self.tilt_east();
        }

        fn tilt_north(&mut self) {
            let mut northernmost_idx = vec![0; self.cols];
            let mut new_round_rocks = BTreeSet::new();
            for row in 0..self.rows {
                self.square_rocks.iter()
                    .filter(|pt| pt.row == row)
                    .for_each(|pt| {
                        northernmost_idx[pt.col] = row + 1;
                    });
                self.round_rocks.iter()
                    .filter(|pt| pt.row == row)
                    .for_each(|pt| {
                        new_round_rocks.insert(Point { row: northernmost_idx[pt.col], col: pt.col });
                        northernmost_idx[pt.col] += 1;
                    });
            }
            self.round_rocks = new_round_rocks;
        }

        fn tilt_west(&mut self) {
            let mut westernmost_idx = vec![0; self.rows];
            let mut new_round_rocks = BTreeSet::new();
            for col in 0..self.cols {
                self.square_rocks.iter()
                    .filter(|pt| pt.col == col)
                    .for_each(|pt| {
                        westernmost_idx[pt.row] = col + 1;
                    });
                self.round_rocks.iter()
                    .filter(|pt| pt.col == col)
                    .for_each(|pt| {
                        new_round_rocks.insert(Point { row: pt.row, col: westernmost_idx[pt.row] });
                        westernmost_idx[pt.row] += 1;
                    });
            }
            self.round_rocks = new_round_rocks;
        }

        fn tilt_south(&mut self) {
            let mut southernmost_idx = vec![self.rows - 1; self.cols];
            let mut new_round_rocks = BTreeSet::new();
            for row in (0..self.rows).rev() {
                if row > 0 {
                    self.square_rocks.iter()
                        .filter(|pt| pt.row == row)
                        .for_each(|pt| {
                            southernmost_idx[pt.col] = row - 1;
                        });
                }
                self.round_rocks.iter()
                    .filter(|pt| pt.row == row)
                    .for_each(|pt| {
                        new_round_rocks.insert(Point { row: southernmost_idx[pt.col], col: pt.col });
                        if southernmost_idx[pt.col] != 0 {
                            southernmost_idx[pt.col] -= 1;
                        }
                    });
            }
            self.round_rocks = new_round_rocks;
        }

        fn tilt_east(&mut self) {
            let mut easternmost_idx = vec![self.cols - 1; self.rows];
            let mut new_round_rocks = BTreeSet::new();
            for col in (0..self.cols).rev() {
                if col > 0 {
                    self.square_rocks.iter()
                        .filter(|pt| pt.col == col)
                        .for_each(|pt| {
                            easternmost_idx[pt.row] = col - 1;
                        });
                }
                self.round_rocks.iter()
                    .filter(|pt| pt.col == col)
                    .for_each(|pt| {
                        new_round_rocks.insert(Point { row: pt.row, col: easternmost_idx[pt.row] });
                        if easternmost_idx[pt.row] != 0 {
                            easternmost_idx[pt.row] -= 1;
                        }
                    });
            }
            self.round_rocks = new_round_rocks;
        }

        fn total_north_load(&self) -> usize {
            self.round_rocks.iter()
                .map(|pt| self.rows - pt.row)
                .sum()
        }
    }

    impl std::fmt::Display for Soln {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut repr = String::new();
            for row in 0..self.rows {
                for col in 0..self.cols {
                    if self.round_rocks.contains(&Point { row, col }) {
                        repr.push('O');
                    } else if self.square_rocks.contains(&Point{ row, col }) {
                        repr.push('#');
                    } else {
                        repr.push('.');
                    }
                }
                repr.push('\n');
            }
            write!(f, "{}", repr)
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(64); "example_1")]
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
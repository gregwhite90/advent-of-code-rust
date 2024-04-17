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

    use std::collections::HashSet;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    struct Point {
        row: usize,
        col: usize,
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct Soln {
        rows: usize,
        cols: usize,
        square_rocks: HashSet<Point>,
        round_rocks: HashSet<Point>,
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
            let prev_rrs = self.round_rocks.clone();
            // This short-circuits when the same state is reached at the
            // end of a spin cycle that started the spin cycle.
            // TODO: If this does not actually short circuit,
            // may need to update to look for loops of the state instead.
            for _ in 0..n {
                self.spin_once();
                if self.round_rocks == prev_rrs { break; }
            }
        }

        fn spin_once(&mut self) {
            self.tilt_north();
            self.tilt_west();
            self.tilt_south();
            self.tilt_east();
        }

        fn tilt_north(&mut self) {
            // TODO: implement
        }

        fn tilt_west(&mut self) {
            // TODO: implement
        }

        fn tilt_south(&mut self) {
            // TODO: implement
        }

        fn tilt_east(&mut self) {
            // TODO: implement
        }

        fn total_north_load(&self) -> usize {
            self.round_rocks.iter()
                .map(|pt| self.rows - pt.row)
                .sum()
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
#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 14 };

/// Solution to [2023-13 part one](https://adventofcode.com/2023/day/13). The way this is written relies
/// on a sparse representation of the pictured pattern, and will only work if each row and each column
/// in each pattern has at least one ash ('.') entry.
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
#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2025, day: 4 };

mod utils {
    use std::collections::HashSet;
    use itertools::iproduct;

    use crate::utils::io_utils;

    #[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
    struct Position {
        // Uses isize instead of usize to obviate the need for edge case checking for neighbors
        row: isize,
        col: isize,
    }

    impl Position {
        pub fn neighbors(&self) -> HashSet<Self> {
            iproduct!(
                (self.row - 1..=self.row + 1),
                (self.col - 1..=self.col + 1)
            )
            .filter(|(row, col)| self.row != *row || self.col != *col)
            .map(|(row, col)| {
                Self {
                    row,
                    col,
                }
            })
            .collect()
        }
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct Grid {
        paper: HashSet<Position>,
    }

    impl Grid {
        pub fn parse_input_file(&mut self, filename: &str) {
            self.paper = io_utils::file_to_lines(filename)
                .enumerate()
                .flat_map(|(row, line)| {
                    line.char_indices()
                        .filter(|(_, ch)| *ch == '@')
                        .map(|(col, _)| {
                            Position {
                                row: row as isize,
                                col: col as isize,
                            }
                        })
                        .collect::<HashSet<Position>>()
                })
                .collect()
        }

        // Modifies in place and eturns the count of paper removed in one round.
        pub fn remove_accessible_paper_round(&mut self, max_neighbors: usize) -> usize {
            let to_remove: HashSet<Position> = self.paper.iter()
                .filter(|paper| paper.neighbors().intersection(&self.paper).count() <= max_neighbors)
                .cloned()
                .collect();
            self.paper.retain(|pos| !to_remove.contains(pos));
            to_remove.len()
        }

        // Modifies in place and returns the count of all paper removed.
        pub fn remove_accessible_paper_all_rounds(&mut self, max_neighbors: usize) -> usize {
            let mut removed: usize = 0;
            loop {
                let removed_round = self.remove_accessible_paper_round(max_neighbors);
                if removed_round == 0 {
                    break;
                }
                removed += removed_round;
            }
            removed
        }
    }

}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};
    use super::utils::Grid;

    #[derive(Debug, Default)]
    pub struct Soln {
        grid: Grid,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.grid.parse_input_file(filename);
            Answer::Usize(self.grid.remove_accessible_paper_round(3))
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(13); "example_1")]
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
    use super::utils::Grid;

    #[derive(Debug, Default)]
    pub struct Soln {
        grid: Grid,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.grid.parse_input_file(filename);
            Answer::Usize(self.grid.remove_accessible_paper_all_rounds(3))
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(43); "example_1")]
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
#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2025, day: 4 };

mod utils {
    use std::collections::{HashMap, HashSet, VecDeque};
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
        paper_to_num_neighbors: HashMap<Position, usize>,
    }

    impl Grid {
        pub fn parse_input_file(&mut self, filename: &str) {
            let paper: HashSet<Position> = io_utils::file_to_lines(filename)
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
                .collect();
            self.paper_to_num_neighbors = paper.iter()
                .map(|pos| {
                    (pos.clone(), pos.neighbors().intersection(&paper).count())
                })
                .collect::<HashMap<Position, usize>>();
        }

        pub fn remove_paper(
            &mut self, 
            max_neighbors: usize,
            iterative: bool,
        ) -> usize {
            let mut to_remove: VecDeque<Position> = self.paper_to_num_neighbors
                .iter()
                .filter_map(|(pos, neighbors)| {
                    if *neighbors <= max_neighbors {
                        Some(*pos)
                    } else {
                        None
                    }
                })
                .collect();
            if !iterative {
                return to_remove.len()
            } else {
                let mut removed: usize = 0;
                while let Some(pos) = to_remove.pop_front() {
                    if let None = self.paper_to_num_neighbors.remove(&pos) { 
                        // Already removed
                        continue;
                    }
                    removed += 1;
                    pos.neighbors()
                        .iter()
                        .for_each(|neighbor| {
                            self.paper_to_num_neighbors
                                .entry(*neighbor)
                                .and_modify(|num_neighbors| {
                                    *num_neighbors -= 1;
                                    if *num_neighbors <= max_neighbors {
                                        to_remove.push_back(*neighbor);
                                    }
                                });
                        });
                }
                removed
            }
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
            Answer::Usize(self.grid.remove_paper(3, false))
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
            Answer::Usize(self.grid.remove_paper(3, true))
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
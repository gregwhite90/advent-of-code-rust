#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2025, day: 7 };

mod utils {
    use std::collections::{HashMap, HashSet, VecDeque};
    use crate::utils::io_utils;

    #[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
    struct Position {
        row: usize,
        col: usize,
    }

    #[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
    pub struct ManifoldSimulationResult {
        times_beam_split: usize,
        // expect this for part two. total_beams: usize,
    }

    impl ManifoldSimulationResult {
        pub fn times_beam_split(&self) -> usize {
            self.times_beam_split
        }
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct Manifold {
        start: Position,
        // Maps a column index to the rows of the splitters in that column
        // Note: using usize for the column is not robust for splitters or starts in the 0 col.
        // That doesn't come up in the example or my puzzle input though.
        splitters: HashMap<usize, Vec<usize>>,
    }

    impl Manifold {
        pub fn parse_input_file(&mut self, filename: &str) {
            io_utils::file_to_lines(filename)
                .enumerate()
                .for_each(|(row, line)| {
                    line.char_indices()
                        .filter(|(_, ch)| *ch == 'S' || *ch == '^')
                        .for_each(|(col, ch)| {
                            match ch {
                                'S' => self.start = Position { row, col },
                                '^' => {
                                    self.splitters.entry(col)
                                        .and_modify(|splitters| splitters.push(row))
                                        .or_insert(vec![row]);
                                },
                                _ => unreachable!(),
                            }
                        });
                })
        }

        pub fn simulate(&self) -> ManifoldSimulationResult {
            let mut to_process: VecDeque<Position> = VecDeque::from([self.start]);
            let mut splitters_hit: HashSet<Position> = HashSet::new();

            while let Some(beam_start) = to_process.pop_front() {
                if let Some(splitter_rows) = self.splitters.get(&beam_start.col) {
                    let splitter_row_idx = splitter_rows
                        .partition_point(|splitter_row| *splitter_row < beam_start.row);
                    if splitter_row_idx != splitter_rows.len() {
                        // Not all elements match the predicate => we found a splitter this would hit
                        let splitter = Position {
                            row: splitter_rows[splitter_row_idx],
                            col: beam_start.col,
                        };
                        if splitters_hit.insert(splitter) {
                            // Newly inserted, must be processed
                            for col in [splitter.col - 1, splitter.col + 1] {
                                to_process.push_back(
                                    Position {
                                        row: splitter.row,
                                        col,
                                    }
                                );
                            }
                        }
                    }
                }
            }

            ManifoldSimulationResult {
                times_beam_split: splitters_hit.len(),
            }
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};
    use super::utils::Manifold;

    #[derive(Debug, Default)]
    pub struct Soln {
        manifold: Manifold,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.manifold.parse_input_file(filename);
            Answer::Usize(self.manifold.simulate().times_beam_split())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(21); "example_1")]
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
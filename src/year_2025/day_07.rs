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

        fn next_splitter(&self, beam_start: &Position) -> Option<Position> {
            if let Some(splitter_rows) = self.splitters.get(&beam_start.col) {
                let splitter_row_idx = splitter_rows
                    .partition_point(|splitter_row| *splitter_row < beam_start.row);
                if splitter_row_idx != splitter_rows.len() {
                    // Not all elements match the predicate => we found a splitter this would hit
                    return Some(
                        Position {
                            row: splitter_rows[splitter_row_idx],
                            col: beam_start.col,
                        }
                    )
                }
            }
            None
        }

        /**
         * Returns the number of splitters that split beams.
         */
        pub fn simulate_classical(&self) -> usize {
            let mut to_process: VecDeque<Position> = VecDeque::from([self.start]);
            let mut splitters_hit: HashSet<Position> = HashSet::new();

            while let Some(beam_start) = to_process.pop_front() {
                if let Some(splitter) = self.next_splitter(&beam_start) {
                    if splitters_hit.insert(splitter) {
                        // Newly inserted, must be processed
                        [splitter.col - 1, splitter.col + 1].into_iter()
                            .for_each(|col| {
                                to_process.push_back(
                                    Position {
                                        row: splitter.row,
                                        col,
                                    }
                                );
                            });
                    }
                }
            }
            splitters_hit.len()
        }

        /**
         * Returns the number of "timelines" a single starting particle would end up on.
         */
        pub fn simulate_quantum(&self) -> usize {
            // Maps beam starting positions to the number of timelines starting from that position
            let mut cache: HashMap<Position, usize> = HashMap::new();
            self.timelines(&mut cache, &self.start)
        }

        fn timelines(&self, cache: &mut HashMap<Position, usize>, beam_start: &Position) -> usize {
            if let Some(timelines) = cache.get(beam_start) {
                return *timelines
            }
            let timelines = if let Some(splitter) = self.next_splitter(beam_start) {
                [splitter.col - 1, splitter.col + 1].into_iter()
                    .map(|col| self.timelines(
                        cache,
                        &Position {
                            row: splitter.row,
                            col,
                        },
                    ))
                    .sum()
            } else {
                1
            };
            cache.insert(*beam_start, timelines);
            timelines
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
            Answer::Usize(self.manifold.simulate_classical())
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

pub mod part_two {
    use crate::utils::solution::{Answer, Solution};
    use super::utils::Manifold;

    #[derive(Debug, Default)]
    pub struct Soln {
        manifold: Manifold,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.manifold.parse_input_file(filename);
            Answer::Usize(self.manifold.simulate_quantum())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(40); "example_1")]
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
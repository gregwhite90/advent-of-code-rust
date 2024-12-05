#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2024, day: 4 };

mod utils {
    use std::collections::{HashMap, HashSet};

    use itertools::iproduct;

    use crate::utils::io_utils;

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    struct DeltaPoint {
        col: isize,
        row: isize,
    }

    impl DeltaPoint {
        fn new(cur: &Point, prev: &Point) -> Self {
            Self {
                col: cur.col - prev.col,
                row: cur.row - prev.row,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    struct Point {
        col: isize,
        row: isize,
    }

    impl Point {
        fn neighbors(&self) -> HashSet<Self> {
            iproduct!(self.col - 1..=self.col + 1, self.row - 1..=self.row + 1)
                .filter_map(|(col, row)| {
                    let pt = Point { col, row };
                    if pt == *self {
                        None
                    } else {
                        Some(pt)
                    }
                })
                .collect()
        }

        fn add_delta(&self, delta: &DeltaPoint) -> Self {
            Self {
                col: self.col + delta.col,
                row: self.row + delta.row,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    struct WordCandidate {
        delta: DeltaPoint,
        point: Point,
    }

    impl WordCandidate {
        fn new(cur: &Point, prev: &Point) -> Self {
            Self {
                delta: DeltaPoint::new(cur, prev),
                point: cur.clone(),
            }
        }

        fn next_point(&self) -> Point {
            self.point.add_delta(&self.delta)
        }

        fn progress(&mut self) {
            self.point = self.next_point();
        }
    }

    #[derive(Debug, Default)]
    pub struct WordSearch {
        chars: HashMap<char, HashSet<Point>>,
    }

    impl WordSearch {
        pub fn parse_input_file(&mut self, filename: &str) {
            io_utils::file_to_lines(filename)
                .enumerate()
                .for_each(|(row, line)| {
                    line.char_indices().for_each(|(col, ch)| {
                        let pt = Point { col: col.try_into().unwrap(), row: row.try_into().unwrap() };
                        self.chars.entry(ch)
                            .and_modify(|pts| {
                                pts.insert(pt);
                            })
                            .or_insert(HashSet::from([pt]));
                    });
                });
        }

        pub fn word_count(&self, word: &str) -> usize {
            assert_ne!(word.len(), 0);
            // TODO, maybe I can do this as a mut sitch
            let mut word_chars = word.chars();
            let char_0_points = self.chars.get(&word_chars.next().unwrap()).unwrap().clone();
            if let Some(char_1) = word_chars.next() {
                if let Some(char_1_points) = self.chars.get(&char_1) {
                    let mut word_candidates: Vec<WordCandidate> = char_0_points.iter()
                        .map(|char_0_pt| {
                            char_0_pt.neighbors().intersection(char_1_points)
                                .map(|char_1_pt| {
                                    WordCandidate::new(char_1_pt, char_0_pt)
                                })
                                .collect::<Vec<WordCandidate>>()
                        })
                        .flatten()
                        .collect();
                    while let Some(ch) = word_chars.next() {
                        if let Some(ch_pts) = self.chars.get(&ch) {
                            word_candidates.retain_mut(|wc| {
                                if ch_pts.contains(&wc.next_point()) {
                                    wc.progress();
                                    return true;
                                } else {
                                    return false;
                                }
                            });
                        } else {
                            return 0;
                        }
                    }
                    return word_candidates.len();
                } else {
                    return 0;
                }
            } else {
                return char_0_points.len();
            }
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::WordSearch;

    #[derive(Debug, Default)]
    pub struct Soln {
        word_search: WordSearch,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.word_search.parse_input_file(filename);
            Answer::Usize(self.word_search.word_count("XMAS"))
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(18); "example_1")]
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
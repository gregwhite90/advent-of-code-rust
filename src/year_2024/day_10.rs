#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2024, day: 10 };

mod utils {
    use std::collections::{HashMap, HashSet};

    use crate::utils::io_utils;

    #[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
    struct Point {
        row: usize,
        col: usize,
    }

    #[derive(Debug, Default)]
    pub struct TrailMap {
        heights: Vec<Vec<u8>>,
    }

    impl TrailMap {
        fn rows(&self) -> usize {
            self.heights.len()
        }
        
        fn cols(&self) -> usize {
            self.heights[0].len()
        }

        fn height(&self, point: &Point) -> u8 {
            self.heights[point.row][point.col]
        }

        fn trailheads(&self) -> impl Iterator<Item = Point> + '_ {
            self.heights.iter()
                .enumerate()
                .map(|(row, heights)| {
                    heights.iter()
                        .enumerate()
                        .filter_map(move |(col, height)| {
                            if *height == 0 {
                                Some(Point{ row, col })
                            } else {
                                None
                            }
                        })
                })
                .flatten()
        }

        pub fn parse_input_file(&mut self, filename: &str) {
            self.heights = io_utils::file_to_lines(filename)
                .map(|line| {
                    line.chars()
                        .map(|ch| ch.to_string().parse().unwrap())
                        .collect()
                })
                .collect()
        }

        pub fn sum_of_trailhead_scores(&self) -> usize {
            let mut cache: HashMap<Point, (HashSet<Point>, usize)> = HashMap::new();
            self.trailheads()
                .map(|trailhead| {
                    self.ends_reachable(&trailhead, &mut cache).0.len()
                })
                .sum()
        }

        pub fn sum_of_trailhead_ratings(&self) -> usize {
            let mut cache: HashMap<Point, (HashSet<Point>, usize)> = HashMap::new();
            self.trailheads()
                .map(|trailhead| {
                    self.ends_reachable(&trailhead, &mut cache).1
                })
                .sum()
        }

        fn ends_reachable(
            &self, 
            pt: &Point,
            mut cache: &mut HashMap<Point, (HashSet<Point>, usize)>,
        ) -> (HashSet<Point>, usize) {
            if let Some(ends) = cache.get(pt) {
                return ends.clone()
            }
            let ends = if self.height(pt) == 9 {
                (HashSet::from([*pt]), 1)
            } else {
                if let Some(reachable_ends) = self.reachable_neighbors(pt)
                    .into_iter()
                    .map(|neighbor| self.ends_reachable(&neighbor, &mut cache))
                    .reduce(|acc, e| {
                        (acc.0.union(&e.0).cloned().collect(), acc.1 + e.1)
                    }) {
                        reachable_ends
                    } else {
                        (HashSet::new(), 0)
                    }
            };
            cache.insert(*pt, ends.clone());
            ends
        }

        fn reachable_neighbors(&self, pt: &Point) -> Vec<Point> {
            let mut neighbors = Vec::new();
            if pt.row > 0 {
                neighbors.push(Point { row: pt.row - 1, col: pt.col });
            }
            if pt.row < self.rows() - 1 {
                neighbors.push(Point { row: pt.row + 1, col: pt.col });
            }
            if pt.col > 0 {
                neighbors.push(Point { row: pt.row, col: pt.col - 1 });
            }
            if pt.col < self.cols() - 1 {
                neighbors.push(Point { row: pt.row, col: pt.col + 1 });
            }
            neighbors.retain(|neighbor| self.height(neighbor) == self.height(pt) + 1);
            neighbors
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::TrailMap;

    #[derive(Debug, Default)]
    pub struct Soln {
        trail_map: TrailMap,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.trail_map.parse_input_file(filename);
            Answer::Usize(self.trail_map.sum_of_trailhead_scores())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(1); "example_1")]
        #[test_case(2, Answer::Usize(36); "example_2")]
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

    use super::utils::TrailMap;

    #[derive(Debug, Default)]
    pub struct Soln {
        trail_map: TrailMap,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.trail_map.parse_input_file(filename);
            Answer::Usize(self.trail_map.sum_of_trailhead_ratings())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(2, Answer::Usize(81); "example_2")]
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
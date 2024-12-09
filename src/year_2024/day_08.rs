#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2024, day: 8 };

mod utils {
    use std::{cmp::max, collections::{HashMap, HashSet}};

    use crate::utils::io_utils;

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    struct Point {
        row: isize,
        col: isize,
    }

    impl Point {
        fn mirror_of_other(&self, other: &Point) -> Self {
            Self {
                row: self.row + (self.row - other.row),
                col: self.col + (self.col - other.col)
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct AntennaMap {
        rows: usize,
        cols: usize,
        antennas: HashMap<char, Vec<Point>>,
    }

    impl AntennaMap {
        pub fn parse_input_file(&mut self, filename: &str) {
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    self.cols = max(self.cols, line.len());
                    line.char_indices().for_each(|(col, ch)| {
                        if ch != '.' {
                            let pt = Point { row: self.rows as isize, col: col as isize };
                            self.antennas.entry(ch).and_modify(|pts| pts.push(pt)).or_insert(Vec::from([pt]));
                        }
                    });
                    self.rows += 1;
                })
        }

        fn antinodes(&self) -> HashSet<Point> {
            let mut res = HashSet::new();
            self.antennas.values()
                .for_each(|pts| {
                    for i in 0..pts.len() {
                        for j in i + 1..pts.len() {
                            res.insert(pts[i].mirror_of_other(&pts[j]));
                            res.insert(pts[j].mirror_of_other(&pts[i]));
                        }
                    }
                });
            res
        }

        pub fn num_antinodes_within_bounds(&self) -> usize {
            self.antinodes().iter()
                .filter(|pt| pt.col >= 0 && pt.col < self.cols as isize && pt.row >= 0 && pt.row < self.rows as isize)
                .count()
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::AntennaMap;

    #[derive(Debug, Default)]
    pub struct Soln {
        antenna_map: AntennaMap,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.antenna_map.parse_input_file(filename);
            Answer::Usize(self.antenna_map.num_antinodes_within_bounds())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(14); "example_1")]
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
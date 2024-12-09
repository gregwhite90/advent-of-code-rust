#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2024, day: 8 };

mod utils {
    use std::{cmp::max, collections::{HashMap, HashSet}};

    use gcd::Gcd;

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

        // TODO: instead return iterator?
        fn all_along_slope(&self, other: &Point, rows: usize, cols: usize) -> HashSet<Self> {
            let mut delta_row = self.row - other.row;
            let mut delta_col = self.col - other.col;
            let gcd = (delta_row.abs() as usize).gcd(delta_col.abs() as usize) as isize;
            delta_row /= gcd;
            delta_col /= gcd;
            let mut res = HashSet::from([*self, *other]);
            let mut cur = *self;
            loop {
                if !cur.is_within_bounds(rows, cols) { break; }
                res.insert(cur);
                cur = Point { row: cur.row + delta_row, col: cur.col + delta_col };
            }
            let mut cur = *self;
            loop {
                if !cur.is_within_bounds(rows, cols) { break; }
                res.insert(cur);
                cur = Point { row: cur.row - delta_row, col: cur.col - delta_col };
            }
            res
        }

        fn is_within_bounds(&self, rows: usize, cols: usize) -> bool {
            self.row >= 0 && self.row < rows as isize && self.col >= 0 && self.col < cols as isize
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

        fn antinodes(&self, exact_dist: bool) -> HashSet<Point> {
            // TODO: this one is when exact_dist is true
            let mut res = HashSet::new();
            self.antennas.values()
                .for_each(|pts| {
                    for i in 0..pts.len() {
                        for j in i + 1..pts.len() {
                            if exact_dist {
                                res.insert(pts[i].mirror_of_other(&pts[j]));
                                res.insert(pts[j].mirror_of_other(&pts[i]));
                            } else {
                                res.extend(pts[i].all_along_slope(&pts[j], self.rows, self.cols));
                            }
                        }
                    }
                });
            res
        }

        pub fn num_antinodes_within_bounds(&self, exact_dist: bool) -> usize {
            self.antinodes(exact_dist).iter()
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
            Answer::Usize(self.antenna_map.num_antinodes_within_bounds(true))
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

pub mod part_two {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::AntennaMap;

    #[derive(Debug, Default)]
    pub struct Soln {
        antenna_map: AntennaMap,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.antenna_map.parse_input_file(filename);
            Answer::Usize(self.antenna_map.num_antinodes_within_bounds(false))
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(34); "example_1")]
        #[test_case(2, Answer::Usize(9); "example_2")]
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
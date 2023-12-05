#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 3 };

pub mod part_one {
    use std::collections::{BTreeSet, HashSet};

    use itertools::iproduct;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
    struct Point {
        row: usize,
        col: usize,
    }

    impl Point {
        fn adjacent_points(&self) -> BTreeSet<Point> {
            let min_row = if self.row == 0 { self.row } else { self.row - 1 };
            let max_row = self.row + 1;
            let min_col = if self.col == 0 { self.col } else { self.col - 1 };
            let max_col = self.col + 1;
            iproduct!(min_row..=max_row, min_col..=max_col)
                .map(|(r, c)| {
                    Point { row: r, col: c }
                })
                .filter(|pt| {
                    pt.row != self.row || pt.col != self.col
                })
                .collect()
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash, Default)]
    struct PartNumber {
        val: u32,
        points: BTreeSet<Point>,
    }

    impl PartNumber {
        fn adjacent_points(&self) -> BTreeSet<Point> {
            let mut result = BTreeSet::new();
            self.points
                .iter()
                .map(|pt| {
                    pt.adjacent_points()
                })
                .for_each(|adj_points| {
                    for pt in adj_points {
                        if !self.points.contains(&pt) {
                            result.insert(pt);
                        }
                    }
                });
            result
        }
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        part_numbers: HashSet<PartNumber>,
        symbols: BTreeSet<Point>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.sum_of_part_numbers())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            io_utils::file_to_lines(filename).enumerate().for_each(|(row, line)| {
                let cols = line.len();
                let mut cur_part_number = PartNumber::default();
                for (col, ch) in line.chars().enumerate() {
                    if !ch.is_ascii_digit() { 
                        self.part_numbers.insert(cur_part_number);
                        cur_part_number = PartNumber::default();
                        if ch != '.' { 
                            self.symbols.insert(Point { row, col });
                        }
                    } else {
                        cur_part_number.val = cur_part_number.val * 10 + ch.to_digit(10).unwrap();
                        cur_part_number.points.insert(Point { row, col });
                        if col == cols - 1 {
                            self.part_numbers.insert(cur_part_number);
                            cur_part_number = PartNumber::default();
                        }
                    }
                }
            });
        }

        fn sum_of_part_numbers(&self) -> u32 {
            self.part_numbers
                .iter()
                .filter(|part_number| {
                    part_number.adjacent_points().intersection(&self.symbols).next() != None
                })
                .map(|part_number| part_number.val)
                .sum()
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(4361); "example_1")]
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
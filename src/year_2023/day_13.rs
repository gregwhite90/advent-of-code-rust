#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 13 };

/// Solution to [2023-13 part one](https://adventofcode.com/2023/day/13). The way this is written relies
/// on a sparse representation of the pictured pattern, and will only work if each row and each column
/// in each pattern has at least one ash ('.') entry.
pub mod part_one {

    use std::{cmp, collections::HashMap};

    use itertools::Itertools;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    #[derive(Debug, Default)]
    struct Pattern {
        rows: usize,
        cols: usize,
        row_to_ash_cols: HashMap<usize, Vec<usize>>,
        col_to_ash_rows: HashMap<usize, Vec<usize>>,
        ash_cols_to_rows: HashMap<Vec<usize>, Vec<usize>>,
        ash_rows_to_cols: HashMap<Vec<usize>, Vec<usize>>,
    }

    impl Pattern {
        pub fn parse_line(&mut self, line: &str) {
            self.cols = line.len();
            line.char_indices()
                .filter(|(_idx, ch)| *ch == '.')
                .map(|(idx, _ch)| idx)
                .for_each(|idx| {
                    // TODO: factor out shared functionality?
                    self.row_to_ash_cols.entry(self.rows)
                        .and_modify(|v| v.push(idx))
                        .or_insert(vec![idx]);
                    self.col_to_ash_rows.entry(idx)
                        .and_modify(|v| v.push(self.rows))
                        .or_insert(vec![self.rows]);
                });
            self.rows += 1;
        }

        pub fn note(&mut self) -> usize {
            if self.ash_cols_to_rows.is_empty() || self.ash_rows_to_cols.is_empty() {
                self.invert();
            }
            let mut note = 0;
            // These reflection axis functions return a 0-indexed result, we need to convert to
            // 1-indexed for the note calculation
            if let Some(axis) = self.vertical_reflection_axis() {
                note += axis + 1;
            }
            if let Some(axis) = self.horizontal_reflection_axis() {
                note += 100 * (axis + 1);
            }
            note
        }

        // TODO: factor out shared functionality?
        fn invert(&mut self) {
            for (row, cols) in &self.row_to_ash_cols {
                self.ash_cols_to_rows.entry(cols.to_vec())
                    .and_modify(|rows| {
                        rows.push(*row);
                    })
                    .or_insert(vec![*row]);
            }
            for (col, rows) in &self.col_to_ash_rows {
                self.ash_rows_to_cols.entry(rows.to_vec())
                    .and_modify(|cols| {
                        cols.push(*col);
                    })
                    .or_insert(vec![*col]);
            }
        }

        // TODO: share functionality?
        // Return value is 0-indexed.
        fn vertical_reflection_axis(&self) -> Option<usize> {
            for cols in self.ash_rows_to_cols.values() {
                if cols.len() >= 2 {
                    'implied_axis: for (col_0, col_1) in cols.iter().tuple_combinations() {
                        // calculate implied axis
                        let min_col = cmp::min(col_0, col_1);
                        let max_col = cmp::max(col_0, col_1);
                        let implied_axis = min_col + (max_col - min_col) / 2; // intentionally uses integer division rounding to 0.
                        // check that implied axis works
                        let start_min_col = implied_axis;
                        let start_max_col = implied_axis + 1;
                        let max_total_col_idx = self.cols - 1;
                        let counting_to_max = max_total_col_idx - start_max_col + 1;
                        let counting_to_zero = start_min_col + 1;
                        let count_to_check = cmp::min(counting_to_max, counting_to_zero);
                        for offset in 0..count_to_check {
                            if self.col_to_ash_rows.get(&(start_min_col - offset)).unwrap() != self.col_to_ash_rows.get(&(start_max_col + offset)).unwrap() {
                                continue 'implied_axis;
                            }
                        }
                        return Some(implied_axis);
                    }
                }
            }
            None
        }

        // TODO: share functionality?
        // Return-value is 0-indexed.
        fn horizontal_reflection_axis(&self) -> Option<usize> {
            for rows in self.ash_cols_to_rows.values() {
                if rows.len() >= 2 {
                    'implied_axis: for (row_0, row_1) in rows.iter().tuple_combinations() {
                        // calculate implied axis
                        let min_row = cmp::min(row_0, row_1);
                        let max_row = cmp::max(row_0, row_1);
                        let implied_axis = min_row + (max_row - min_row) / 2; // intentionally uses integer division rounding to 0.
                        // check that implied axis works
                        let start_min_row = implied_axis;
                        let start_max_row = implied_axis + 1;
                        let max_total_row_idx = self.rows - 1;
                        let counting_to_max = max_total_row_idx - start_max_row + 1;
                        let counting_to_zero = start_min_row + 1;
                        let count_to_check = cmp::min(counting_to_max, counting_to_zero);
                        for offset in 0..count_to_check {
                            if self.row_to_ash_cols.get(&(start_min_row - offset)).unwrap() != self.row_to_ash_cols.get(&(start_max_row + offset)).unwrap() {
                                continue 'implied_axis;
                            }
                        }
                        return Some(implied_axis);
                    }
                }
            }
            None
        }

    }

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        summarized_notes: usize,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::Usize(self.summarized_notes())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let mut pattern = Pattern::default();
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    if line.is_empty() {
                        self.summarized_notes += pattern.note();
                        pattern = Pattern::default();
                    } else {
                        pattern.parse_line(&line);
                    }
                });
            self.summarized_notes += pattern.note();
        }

        fn summarized_notes(&self) -> usize {
            self.summarized_notes
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(405); "example_1")]
        #[test_case(11, Answer::Usize(6); "example_11")]
        #[test_case(12, Answer::Usize(4); "example_12")]
        #[test_case(13, Answer::Usize(1_300); "example_13")]
        #[test_case(14, Answer::Usize(12); "example_14")]
        #[test_case(15, Answer::Usize(3); "example_15")]
        #[test_case(16, Answer::Usize(200); "example_16")]
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
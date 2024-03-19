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

    fn push_to_hashmap_value(hm: &mut HashMap<usize, Vec<usize>>, key: usize, val: usize) {
        hm.entry(key).and_modify(|v| v.push(val)).or_insert(vec![val]);
    }

    fn invert(invertee: &HashMap<usize, Vec<usize>>) -> HashMap<Vec<usize>, Vec<usize>> {
        let mut inverted = HashMap::new();
        for (key, vals) in invertee {
            inverted.entry(vals.to_vec())
                .and_modify(|keys: &mut Vec<usize>| {
                    keys.push(*key);
                })
                .or_insert(vec![*key]);
        }
        inverted
    }

    fn reflection_axis(
        vec_to_vec: &HashMap<Vec<usize>, Vec<usize>>,
        total: usize,
        idx_to_vec: &HashMap<usize, Vec<usize>>
    ) -> Option<usize> {
        for idxs in vec_to_vec.values() {
            if idxs.len() >= 2 {
                'implied_axis: for (idx_0, idx_1) in idxs.iter().tuple_combinations() {
                    // calculate implied axis
                    let min_idx = cmp::min(idx_0, idx_1);
                    let max_idx = cmp::max(idx_0, idx_1);
                    let implied_axis = min_idx + (max_idx - min_idx) / 2; // intentionally uses integer division rounding to 0.
                    // check that implied axis works
                    let start_min_idx = implied_axis;
                    let start_max_idx = implied_axis + 1;
                    let max_total_idx = total - 1;
                    let counting_to_max = max_total_idx - start_max_idx + 1;
                    let counting_to_zero = start_min_idx + 1;
                    let count_to_check = cmp::min(counting_to_max, counting_to_zero);
                    for offset in 0..count_to_check {
                        if idx_to_vec.get(&(start_min_idx - offset)).unwrap() != idx_to_vec.get(&(start_max_idx + offset)).unwrap() {
                            continue 'implied_axis;
                        }
                    }
                    return Some(implied_axis);
                }
            }
        }
        None
    }

    #[derive(Debug, PartialEq, Eq)]
    enum Axis {
        Horizontal,
        Vertical,
    }

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
                    push_to_hashmap_value(&mut self.row_to_ash_cols, self.rows, idx);
                    push_to_hashmap_value(&mut self.col_to_ash_rows, idx, self.rows);
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
            if let Some(axis) = self.reflection_axis(Axis::Vertical) {
                note += axis + 1;
            }
            if let Some(axis) = self.reflection_axis(Axis::Horizontal) {
                note += 100 * (axis + 1);
            }
            note
        }

        fn invert(&mut self) {
            self.ash_cols_to_rows = invert(&self.row_to_ash_cols);
            self.ash_rows_to_cols = invert(&self.col_to_ash_rows);
        }

        // Return value is 0-indexed.
        fn reflection_axis(&self, axis: Axis) -> Option<usize> {
            let total = match axis { 
                Axis::Horizontal => self.rows,
                Axis::Vertical => self.cols,
            };
            let vec_to_vec = match axis {
                Axis::Horizontal => &self.ash_cols_to_rows,
                Axis::Vertical => &self.ash_rows_to_cols,
            };
            let idx_to_vec = match axis {
                Axis::Horizontal => &self.row_to_ash_cols,
                Axis::Vertical => &self.col_to_ash_rows,
            };
            reflection_axis(vec_to_vec, total, idx_to_vec)
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
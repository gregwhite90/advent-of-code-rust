#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 13 };

mod utils {

    use std::{cmp, collections::{HashMap, HashSet}};

    use itertools::Itertools;

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
        smudges_necessary: usize,
        vec_to_vec: &HashMap<Vec<usize>, Vec<usize>>,
        total: usize,
        idx_to_vec: &HashMap<usize, Vec<usize>>
    ) -> Option<usize> {
        for idxs in vec_to_vec.values() {
            if idxs.len() >= 2 {
                for (idx_0, idx_1) in idxs.iter().tuple_combinations() {
                    // calculate implied axis
                    let min_idx = cmp::min(idx_0, idx_1);
                    let max_idx = cmp::max(idx_0, idx_1);
                    let implied_axis = min_idx + (max_idx - min_idx) / 2; // intentionally uses integer division rounding to 0.
                    if check_axis(smudges_necessary, implied_axis, total, idx_to_vec) {
                        return Some(implied_axis);
                    }
                }
            }
        }
        None
    }

    fn check_axis(smudges_necessary: usize, axis: usize, total: usize, idx_to_vec: &HashMap<usize, Vec<usize>>) -> bool {
        let start_min_idx = axis;
        let start_max_idx = axis + 1;
        let max_total_idx = total - 1;
        let counting_to_max = max_total_idx - start_max_idx + 1;
        let counting_to_zero = start_min_idx + 1;
        let count_to_check = cmp::min(counting_to_max, counting_to_zero);
        let mut smudges = 0;
        for offset in 0..count_to_check {
            let min_idx_idxs: HashSet<usize> = HashSet::from_iter(idx_to_vec.get(&(start_min_idx - offset)).unwrap().to_vec());
            let max_idx_idxs: HashSet<usize> = HashSet::from_iter(idx_to_vec.get(&(start_max_idx + offset)).unwrap().to_vec());
            let diff: HashSet<&usize> = min_idx_idxs.symmetric_difference(&max_idx_idxs).collect();
            smudges += diff.len();
            if smudges > smudges_necessary {
                return false;
            }
        }
        if smudges == smudges_necessary {
            return true;
        }
        false
    }

    #[derive(Debug, PartialEq, Eq)]
    enum Axis {
        Horizontal,
        Vertical,
    }

    #[derive(Debug, Default)]
    pub struct Pattern {
        rows: usize,
        cols: usize,
        smudges_necessary: usize,
        row_to_ash_cols: HashMap<usize, Vec<usize>>,
        col_to_ash_rows: HashMap<usize, Vec<usize>>,
        ash_cols_to_rows: HashMap<Vec<usize>, Vec<usize>>,
        ash_rows_to_cols: HashMap<Vec<usize>, Vec<usize>>,
    }

    impl Pattern {
        pub fn with_smudges_necessary(smudges_necessary: usize) -> Self {
            if smudges_necessary > 1 {
                panic!("Approach is not guaranteed to work when more than one smudge is necessary.");
            }
            Self {
                rows: 0,
                cols: 0,
                smudges_necessary,
                row_to_ash_cols: HashMap::new(),
                col_to_ash_rows: HashMap::new(),
                ash_cols_to_rows: HashMap::new(),
                ash_rows_to_cols: HashMap::new(),
            }
        }

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
            if let Some(axis) = reflection_axis(self.smudges_necessary, vec_to_vec, total, idx_to_vec) {
                return Some(axis);
            }
            // Standard way of finding the reflection axis will not work if all of the
            // row/col pairs need at least one smudge. In the case of this part two,
            // there is only one allowable/necessary smudge, so we can narrow this to
            // the case where smudges are necessary in the first or last two rows or
            // columns (anywhere else and the axis will be checked by the directly
            // matching rows or columns that don't require smudges). By relying on this,
            // we are limiting the applicability of this solution to 0 or 1 smudges
            // (which is all we need for this puzzle but not more broadly generalized).
            if check_axis(self.smudges_necessary, 0, total, idx_to_vec) {
                return Some(0);
            }
            if check_axis(self.smudges_necessary, total - 2, total, idx_to_vec) {
                return Some(total - 2);
            }
            None
        }
    }
}

/// Solution to [2023-13 part one](https://adventofcode.com/2023/day/13). The way this is written relies
/// on a sparse representation of the pictured pattern, and will only work if each row and each column
/// in each pattern has at least one ash ('.') entry.
pub mod part_one {

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    use super::utils::Pattern;

    #[derive(Debug, Default, PartialEq, Eq)]
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
            let smudges_necessary = 0;
            let mut pattern = Pattern::with_smudges_necessary(smudges_necessary);
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    if line.is_empty() {
                        self.summarized_notes += pattern.note();
                        pattern = Pattern::with_smudges_necessary(smudges_necessary);
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

pub mod part_two {

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    use super::utils::Pattern;

    #[derive(Debug, Default, PartialEq, Eq)]
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
            let smudges_necessary = 1;
            let mut pattern = Pattern::with_smudges_necessary(smudges_necessary);
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    if line.is_empty() {
                        self.summarized_notes += pattern.note();
                        pattern = Pattern::with_smudges_necessary(smudges_necessary);
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

        #[test_case(1, Answer::Usize(400); "example_1")]
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
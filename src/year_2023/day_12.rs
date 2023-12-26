#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 12 };

/// Solution to [2023-12 part one](https://adventofcode.com/2023/day/12).
/// 
/// We need to find a way to break down the problem into smaller (memo-ized, recursive)
/// versions of the same problem, because a brute force solution checking all combinations
/// of `#` and `.` for each `?` will be $O(2^n)$ for each line, where $n$ is the number of `?`s.
/// 
/// Manually solved examples with notes on how this algorithm was developed:
/// 
/// | Condition record      | Contiguous groups of `#`  | Size blocks that can house contiguous groups of `#` |
/// | ----------------      | ------------------------  | --- |
/// | `???.###`             | `1,1,3`                   | `3 3`  `1,1 3`    |  Last block has to be 3.
/// | `.??..??...?##.`      | `1,1,3`                   |  Last block has to be 3. each of first 2 blocks has to be 1.     |
/// | `?#?#?#?#?#?#?#?`     | `1,3,1,6`                 |  All one block. First block has to be solo. Then the 3 needs to be completed. This is fully determined.    |
/// | `????.#...#...`       | `4,1,1`                   |  Also fully determined. could start from back and know that.     |
/// | `????.######..#####.` | `1,6,5`                   |  Same as above.     |
/// | `?###????????`        | `3,2,1`                   |  This becomes 2,1 in 7     |
/// 
pub mod part_one {
    use std::{collections::{HashSet, HashMap}, cell::{RefCell, Ref, RefMut}};

    use crate::utils::solution::{Solution, Answer};

    #[derive(Debug, PartialEq, Eq)]
    pub struct Soln {
        empty_cols: HashSet<usize>,
        empty_rows: HashSet<usize>,
        galaxies: Vec<Point>,
        distances: RefCell<HashMap<(Point, Point), usize>>,
        sum_of_min_distances: Option<usize>,
        expansion_factor: usize,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self::with_expansion_factor(2)
        }
    }

    impl Year2023Day11Solution for Soln {
        fn add_empty_row(&mut self, row: usize) {
            self.empty_rows.insert(row);
        }

        fn add_galaxy(&mut self, galaxy: Point) {
            self.galaxies.push(galaxy);
        }

        fn galaxies(&self) -> &Vec<Point> {
            &self.galaxies
        }

        fn set_empty_cols(&mut self, empty_cols: HashSet<usize>) {
            self.empty_cols = empty_cols;
        }

        fn empty_cols(&self) -> &HashSet<usize> {
            &self.empty_cols
        }

        fn empty_rows(&self) -> &HashSet<usize> {
            &self.empty_rows
        }

        fn distances(&self) -> Ref<HashMap<(Point, Point), usize>> {
            self.distances.borrow()
        }

        fn distances_mut(&self) -> RefMut<HashMap<(Point, Point), usize>> {
            self.distances.borrow_mut()
        }        

        fn expansion_factor(&self) -> usize {
            self.expansion_factor
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.sum_of_min_distances().try_into().unwrap())
        }
    }

    impl Soln {
        fn with_expansion_factor(expansion_factor: usize) -> Self {
            Self {
                empty_cols: HashSet::new(),
                empty_rows: HashSet::new(),
                galaxies: vec![],
                distances: RefCell::new(HashMap::new()),
                sum_of_min_distances: None,
                expansion_factor,    
            }    
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(374); "example_1")]
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
#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 24 };

pub mod part_one {
    use std::{collections::{HashSet, HashMap}, cell::{RefCell, Ref, RefMut}};

    use crate::utils::solution::{Solution, Answer};

    #[derive(Debug, PartialEq, Eq)]
    struct Position {
        x: f64,
        y: f64,
        z: f64,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Velocity {
        x: f64,
        y: f64,
        z: f64,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Hailstone {
        pos: Position,
        vel: Velocity,
        time: f64,
    }

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

        #[test_case(1, Answer::U32(2); "example_1")]
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
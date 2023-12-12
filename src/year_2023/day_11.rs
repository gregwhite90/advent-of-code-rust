#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 11 };

mod utils {
    use std::{collections::{HashSet, HashMap}, cmp::Ordering, cell::{Ref, RefMut}};

    use crate::utils::io_utils;

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct Point {
        row: usize,
        col: usize,
    }

    impl Point {
        pub fn manhattan_distance(&self, other: &Point) -> usize {
            self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
        }

        pub fn col(&self) -> usize {
            self.col
        }
    }

    pub trait Year2023Day11Solution {
        fn add_empty_row(&mut self, row: usize);
        fn add_galaxy(&mut self, galaxy: Point);
        fn galaxies(&self) -> &Vec<Point>;
        fn set_empty_cols(&mut self, empty_cols: HashSet<usize>);
        fn empty_cols(&self) -> &HashSet<usize>;
        fn empty_rows(&self) -> &HashSet<usize>;
        fn distances(&self) -> Ref<HashMap<(Point, Point), usize>>;
        fn distances_mut(&self) -> RefMut<HashMap<(Point, Point), usize>>;

        fn parse_input_file(&mut self, filename: &str) {
            let mut row: usize = 0;
            let mut cols: usize = 0;
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    cols = line.len();
                    let galaxies: Vec<usize> = line.chars().enumerate()
                        .filter(|(_col, ch)| *ch == '#')
                        .map(|(col, _ch)| col)
                        .collect();
                    if galaxies.len() == 0 { self.add_empty_row(row); }
                    for col in galaxies {
                        self.add_galaxy(Point { row, col });
                    }
                    row += 1;
                });
            self.calculate_empty_cols(cols);
        }

        fn calculate_empty_cols(&mut self, num_cols: usize) {
            let nonempty_cols: HashSet<usize> = self.galaxies().iter().map(|pt| {
                pt.col()
            }).collect();
            self.set_empty_cols(
                    (0_usize..num_cols)
                    .collect::<HashSet<usize>>()
                    .difference(&nonempty_cols)
                    .map(|col| *col)
                    .collect()
            );
        }

        /// Recalculates the sum of the minimum distances from scratch
        fn sum_of_min_distances(&mut self, expansion_factor: usize) -> usize {
            for (id, galaxy) in self.galaxies().iter().enumerate() {
                for galaxy_inner in self.galaxies()[id + 1..self.galaxies().len()].iter() {
                    self.distances_mut().insert(
                        (*galaxy, *galaxy_inner), 
                        self.distance(galaxy, galaxy_inner, expansion_factor),
                    );
                }
            }
            self.distances().values().sum()
        }

        fn distance(&self, l: &Point, r: &Point, expansion_factor: usize) -> usize {
            let expanded_rows: usize = match l.row.cmp(&r.row) {
                Ordering::Equal => 0,
                Ordering::Less => {
                    self.empty_rows().iter()
                        .filter(|row| row > &&l.row && row < &&r.row)
                        .count()
                },
                Ordering::Greater => {
                    self.empty_rows().iter()
                        .filter(|row| row > &&r.row && row < &&l.row)
                        .count()
                },
            };
            let expanded_cols: usize = match l.col.cmp(&r.col) {
                Ordering::Equal => 0,
                Ordering::Less => {
                    self.empty_cols().iter()
                        .filter(|col| col > &&l.col && col < &&r.col)
                        .count()
                },
                Ordering::Greater => {
                    self.empty_cols().iter()
                        .filter(|col| col > &&r.col && col < &&l.col)
                        .count()
                },
            };
            l.manhattan_distance(r) + (expanded_rows + expanded_cols) * expansion_factor
        }
    }
}

pub mod part_one {
    use std::{collections::{HashSet, HashMap}, cell::{RefCell, Ref, RefMut}};

    use crate::utils::solution::{Solution, Answer};

    use super::utils::{Year2023Day11Solution, Point};

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        empty_cols: HashSet<usize>,
        empty_rows: HashSet<usize>,
        galaxies: Vec<Point>,
        distances: RefCell<HashMap<(Point, Point), usize>>,
        sum_of_min_distances: Option<usize>,
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
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.sum_of_min_distances(1) as u32)
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
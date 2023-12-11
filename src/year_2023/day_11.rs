#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 11 };

pub mod part_one {
    use std::{collections::{HashSet, HashMap}, cmp::Ordering};

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    struct Point {
        row: usize,
        col: usize,
    }

    impl Point {
        fn manhattan_distance(&self, other: &Point) -> usize {
            self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
        }
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        empty_cols: HashSet<usize>,
        empty_rows: HashSet<usize>,
        galaxies: Vec<Point>,
        distances: HashMap<(Point, Point), usize>,
        sum_of_min_distances: Option<usize>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.sum_of_min_distances() as u32)
        }
    }

    impl Soln {
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
                    if galaxies.len() == 0 { self.empty_rows.insert(row); }
                    for col in galaxies {
                        self.galaxies.push(Point { row, col });
                    }
                    row += 1;
                });
            self.calculate_empty_cols(cols);
        }

        fn calculate_empty_cols(&mut self, num_cols: usize) {
            let nonempty_cols: HashSet<usize> = self.galaxies.iter().map(|pt| {
                pt.col
            }).collect();
            self.empty_cols = (0_usize..num_cols)
                .collect::<HashSet<usize>>()
                .difference(&nonempty_cols)
                .map(|col| *col)
                .collect();
        }

        fn sum_of_min_distances(&mut self) -> usize {
            if let Some(sum_of_min_distances) = self.sum_of_min_distances { return sum_of_min_distances; }
            for (id, galaxy) in self.galaxies.iter().enumerate() {
                for galaxy_inner in self.galaxies[id + 1..self.galaxies.len()].iter() {
                    self.distances.insert(
                        (*galaxy, *galaxy_inner), 
                        self.distance(galaxy, galaxy_inner)
                    );
                }
            }
            self.sum_of_min_distances = Some(self.distances.values().sum());
            self.sum_of_min_distances.unwrap()
        }

        fn distance(&self, l: &Point, r: &Point) -> usize {
            let expanded_rows: usize = match l.row.cmp(&r.row) {
                Ordering::Equal => 0,
                Ordering::Less => {
                    self.empty_rows.iter()
                        .filter(|row| row > &&l.row && row < &&r.row)
                        .count()
                },
                Ordering::Greater => {
                    self.empty_rows.iter()
                        .filter(|row| row > &&r.row && row < &&l.row)
                        .count()
                },
            };
            let expanded_cols: usize = match l.col.cmp(&r.col) {
                Ordering::Equal => 0,
                Ordering::Less => {
                    self.empty_cols.iter()
                        .filter(|col| col > &&l.col && col < &&r.col)
                        .count()
                },
                Ordering::Greater => {
                    self.empty_cols.iter()
                        .filter(|col| col > &&r.col && col < &&l.col)
                        .count()
                },
            };
            l.manhattan_distance(r) + expanded_rows + expanded_cols
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
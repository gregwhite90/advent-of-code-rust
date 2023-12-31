#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 14 };

const ROWS: usize = 128;
const COLS: usize = 128;

pub mod part_one {
    use crate::{utils::{solution::{Solution, Answer}, io_utils}, year_2017::utils::knot_hasher::KnotHasher};

    use super::ROWS;

    #[derive(PartialEq, Eq, Debug, Default)]
    pub struct Soln {
        ones: u32,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.ones)
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let key = io_utils::file_to_string(filename);
            self.ones = (0..ROWS)
                .map(|idx| {
                    let mut hasher = KnotHasher::default();
                    hasher.parse_key(&format!("{key}-{idx}"));
                    hasher.all_steps();
                    u128::from_str_radix(&hasher.knot_hash(), 16)
                        .expect("Should be able to parse base 16 string to u128.")
                        .count_ones()
                })
                .sum()
        }
    }
 
    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(8108); "example_1")]
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
    use std::fmt;

    use crate::utils::{solution::{Solution, Answer}, io_utils};
    use crate::year_2017::utils::{knot_hasher::KnotHasher, map_of_groups::MapOfGroups};

    use super::{ROWS, COLS};

    #[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Default)]
    struct Point {
        col: i32,
        row: i32,
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Point (col: {}, row: {})", self.col, self.row)
        }    
    }

    impl Point {
        fn up_and_right_neighbors(&self) -> [Point; 2] {
            [ 
                Point { col: self.col + 1, row: self.row },
                Point { col: self.col, row: self.row - 1 },
            ]
        }
    }

    #[derive(PartialEq, Eq, Debug, Default)]
    pub struct Soln {
        map_of_groups: MapOfGroups<Point>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.map_of_groups.groups())
        }
    }

    impl Soln {
        fn handle_point(&mut self, pt: &Point) {
            self.map_of_groups.add_member(
                *pt, 
                pt.up_and_right_neighbors()
                    .iter()
                    .filter(|neighbor| self.map_of_groups.contains(neighbor))
                    .map(|pt| *pt)
                    .collect()
            );
        }

        fn parse_input_file(&mut self, filename: &str) {
            let key = io_utils::file_to_string(filename);
            (0..ROWS)
                .for_each(|row| {
                    let mut hasher = KnotHasher::default();
                    hasher.parse_key(&format!("{key}-{row}"));
                    hasher.all_steps();
                    let mut num = u128::from_str_radix(&hasher.knot_hash(), 16)
                        .expect("Should be able to parse base 16 string to u128.");
                    self.handle_num(&mut num, row);
                })
        }

        fn handle_num(&mut self, num: &mut u128, row: usize) {
            let mut col = COLS as i32 - 1;
            while *num != 0 {
                if *num & 1 == 1 { 
                    self.handle_point(&Point { col: col as i32, row: row as i32 });
                }
                col -= 1;
                *num >>= 1;
            }
        }
    }
 
    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test]
        fn handle_num_is_correct() {
            let mut soln = Soln::default();
            let mut num: u128 = 0b101;
            soln.handle_num(&mut num, 0);
            assert_eq!(soln.map_of_groups.groups(), 2);
            num = 0b111;
            soln.handle_num(&mut num, 1);
            assert_eq!(soln.map_of_groups.groups(), 1);
        }

        #[test_case(1, Answer::U32(1242); "example_1")]
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
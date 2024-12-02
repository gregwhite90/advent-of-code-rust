#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2015, day: 18 };

mod utils {
    use std::collections::BTreeSet;

    use itertools::iproduct;

    use crate::utils::io_utils;

    #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    struct Point {
        col: isize,
        row: isize,
    }

    impl Point {
        fn neighbors(&self) -> BTreeSet<Self> {
            iproduct!(self.col - 1..=self.col + 1, self.row - 1..=self.row + 1)
                .filter_map(|(col, row)| {
                    let pt = Point { col, row };
                    if pt == *self {
                        None
                    } else {
                        Some(pt)
                    }
                })
                .collect()
        }
    }

    #[derive(Debug, Default)]
    pub struct LightGrid {
        on: BTreeSet<Point>,
        rows: usize,
    }

    impl LightGrid {
        pub fn parse_input_file(&mut self, filename: &str) {
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    line.chars().enumerate()
                        .filter_map(|(idx, ch)| if ch == '#' { Some(idx) } else { None })
                        .for_each(|col| { 
                            self.on.insert(Point { col: col as isize, row: self.rows as isize }); 
                        });
                    self.rows += 1;
                });
        }

        fn tick(&mut self) {
            self.on = iproduct!(0..self.rows, 0..self.rows)
                .filter_map(|(col, row)| {
                    let pt = Point { col: col as isize, row: row as isize };
                    let neighbors_on = pt.neighbors().intersection(&self.on).count();
                    match self.on.contains(&pt) {
                        true => {
                            if neighbors_on >= 2 && neighbors_on <= 3 {
                                Some(pt)
                            } else {
                                None
                            }
                        },
                        false => {
                            if neighbors_on == 3 {
                                Some(pt)
                            } else {
                                None
                            }
                        },
                    }
                })
                .collect();
        }

        pub fn num_on_after(&mut self, ticks: usize) -> usize {
            for _ in 0..ticks {
                self.tick();
            }
            self.on.len()
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::LightGrid;

    #[derive(Debug)]
    pub struct Soln {
        light_grid: LightGrid,
        ticks: usize,
    }

    impl Soln {
        fn with_ticks(ticks: usize) -> Self {
            Self {
                light_grid: LightGrid::default(),
                ticks,
            }
        }
    }

    impl Default for Soln {
        fn default() -> Self {
            Self::with_ticks(100)
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.light_grid.parse_input_file(filename);
            Answer::Usize(self.light_grid.num_on_after(self.ticks))
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(4); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::with_ticks(4),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}
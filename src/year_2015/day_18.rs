#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2015, day: 17 };

mod utils {
    use std::collections::{BTreeMap, BTreeSet, HashMap};

    use crate::utils::io_utils;

    #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    struct Point {
        col: usize,
        row: usize,
    }

    #[derive(Debug, Default,)]
    pub struct LightGrid {
        on: BTreeSet<Point>,
    }

    impl LightGrid {
        pub fn parse_input_file(&mut self, filename: &str) {
            let mut rows = 0;
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    line.chars().enumerate()
                        .filter_map(|(idx, ch)| if ch == '#' { Some(idx) } else { None })
                        .for_each(|col| { 
                            self.on.insert(Point { col, row: rows }); 
                        });
                    rows += 1;
                });
        }

        fn tick(&mut self) {
            // TODO: implement
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

    #[derive(Debug, Default)]
    pub struct Soln {
        light_grid: LightGrid,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.light_grid.parse_input_file(filename);
            Answer::Usize(self.light_grid.num_on_after(100))
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
                &mut Soln::default(),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}
#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2025, day: 9 };

mod utils {
    use std::collections::BinaryHeap;
    use itertools::Itertools;

    use crate::utils::io_utils;

    #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct Point {
        col: usize,
        row: usize,
    }

    impl Point {
        fn rect_area(&self, other: &Self) -> usize {
            (self.col.abs_diff(other.col) + 1) * (self.row.abs_diff(other.row) + 1)
        }
    }

    #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct Rect<'a> {
        area: usize,
        points: Vec<&'a Point>,
    }

    #[derive(Debug, Default)]
    pub struct Theater {
        points: Vec<Point>,
    }

    impl Theater {
        pub fn parse_input_file(&mut self, filename: &str) {
            self.points = io_utils::file_to_lines(filename)
                .map(|line| {
                    let mut coords = line.split(',')
                        .map(|coord| coord.parse::<usize>().unwrap());
                    Point {
                        col: coords.next().unwrap(),
                        row: coords.next().unwrap(),
                    }
                })
                .collect();
        }

        pub fn largest_area(&self) -> usize {
            let pairwise_areas: BinaryHeap<Rect> = BinaryHeap::from_iter(
                self.points.iter().combinations(2).map(|points| {
                    Rect {
                        area: points[0].rect_area(points[1]),
                        points,
                    }
                })
            );
            pairwise_areas.peek().unwrap().area
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};
    use super::utils::Theater;

    #[derive(Debug, Default)]
    pub struct Soln {
        theater: Theater,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.theater.parse_input_file(filename);
            Answer::Usize(self.theater.largest_area())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(50); "example_1")]
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
#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2025, day: 9 };

mod utils {
    use std::collections::{BinaryHeap, HashMap};
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

    // TODO: possibly consolidate this into Theater or inherit from it
    // if there's a lot of shared functionality
    struct RestrictedTheater {
        points: Vec<Point>, // TODO: need?
        // Need to know if clockwise or counter-clockwise
        // Need cardinal directions enum
        // Need horizontal segments.
        // Segment is start, end, direction.
        // Combine with overall direction of travel to know if up or down is "in".
        // 
        // To check validity:
        //    go from one edge sweeping down? or right? Maybe the shorter dimension? Either should flag it.
        //    if it intersects (not just on an edge) and is in the opposite direction, it's invalid unless there's
        //    an immediately below facing the same direction again that covers the whole thing.
        // Need an efficient way to check intersection
        // Maps row index to the horizontal segments on that row
        horizontal_directed_segments: HashMap<usize, Vec<DirectedSegment>>, // TODO: probably want the values sorted for easy lookups
        // Maps column index to the vertical segments on that col
        vertical_directed_segments: HashMap<usize, Vec<DirectedSegment>>, // TODO: probably want the values sorted
        orientation: Orientation,
    }

    enum Direction {
        North,
        East,
        South,
        West,
    }

    enum Orientation {
        Clockwise,
        Counterclockwise,
    }

    struct Segment {
        low: usize,
        high: usize,
    }

    impl Segment {
        fn overlap(&self, other: &Self) -> Option<Segment> {
            if other.high > self.low && other.low < self.high {
                Some(Segment {
                    low: std::cmp::max(self.low, other.low),
                    high: std::cmp::min(self.high, other.high),
                })
            } else {
                None
            }
        }
    }

    struct DirectedSegment {
        segment: Segment,
        direction: Direction,
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

pub mod part_two {
    use crate::utils::solution::{Answer, Solution};

    #[derive(Debug, Default)]
    pub struct Soln {}

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            unimplemented!();
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(24); "example_1")]
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
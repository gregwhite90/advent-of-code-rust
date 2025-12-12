#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2025, day: 9 };

mod utils {
    use std::{cmp::{self, Ordering}, collections::{BinaryHeap, HashMap}, net};
    use itertools::Itertools;

    use crate::utils::io_utils;

    #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    struct Point {
        col: usize,
        row: usize,
    }

    impl Point {
        fn rect_area(&self, other: &Self) -> usize {
            (self.col.abs_diff(other.col) + 1) * (self.row.abs_diff(other.row) + 1)
        }

        fn direction_to(&self, other: &Self) -> Direction {
            match (self.col.cmp(&other.col), self.row.cmp(&other.row)) {
                (Ordering::Equal, Ordering::Less) => Direction::South,
                (Ordering::Equal, Ordering::Greater) => Direction::North,
                (Ordering::Less, Ordering::Equal) => Direction::East,
                (Ordering::Greater, Ordering::Equal) => Direction::West,
                _ => panic!("Comparing points that don't share exactly one of row or column."),
            }
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
    #[derive(Debug, Default)]
    pub struct RestrictedTheater {
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
        orientation: Option<Orientation>,
    }

    impl RestrictedTheater {
        pub fn parse_input_file(&mut self, filename: &str) {
            // TODO: what else needs 
            let mut net_clockwise: isize = 0;
            let mut prev_point: Option<Point> = None;
            let mut prev_dir: Option<Direction> = None;
            self.points = io_utils::file_to_lines(filename)
                .map(|line| {
                    let mut coords = line.split(',')
                        .map(|coord| coord.parse::<usize>().unwrap());
                    let pt = Point {
                        col: coords.next().unwrap(),
                        row: coords.next().unwrap(),
                    };
                    let cur_dir = if let Some(point) = prev_point {
                        let dir = point.direction_to(&pt);
                        let directed_segment = DirectedSegment::new(
                            point,
                            pt,
                            dir,
                        );
                        match dir {
                            Direction::North | Direction::South => {
                                self.vertical_directed_segments.entry(pt.col)
                                    .and_modify(|v| {
                                        let idx = v.partition_point(|&seg| seg <= directed_segment);
                                        v.insert(idx, directed_segment);
                                    })
                                    .or_insert(vec![directed_segment]);
                            },
                            Direction::East | Direction::West => {
                                self.horizontal_directed_segments.entry(pt.row)
                                    .and_modify(|v| {
                                        let idx = v.partition_point(|&seg| seg <= directed_segment);
                                        v.insert(idx, directed_segment);
                                    })
                                    .or_insert(vec![directed_segment]);
                            }
                        }
                        Some(dir)
                    } else {
                        None
                    };
                    if let Some(p_dir) = prev_dir {
                        match p_dir.orientation_turning_to(
                            &cur_dir.expect("Have a previous direction, so should have a current direction")
                        ) {
                            Orientation::Clockwise        => net_clockwise += 1,
                            Orientation::Counterclockwise => net_clockwise -= 1,
                        }
                    }
                    prev_point = Some(pt.clone());
                    prev_dir = cur_dir;
                    pt
                })
                .collect();
            self.orientation = match net_clockwise.cmp(&0) {
                Ordering::Less    => Some(Orientation::Counterclockwise),
                Ordering::Greater => Some(Orientation::Clockwise),
                _ => panic!("Unexpected value for net clockwise turns"),
            }
        }

        pub fn largest_area(&self) -> usize {
            let pairwise_areas: BinaryHeap<Rect> = BinaryHeap::from_iter(
                self.points.iter().combinations(2).filter_map(|points| {
                    if self.rect_is_valid(&points) {
                        Some(
                            Rect {
                                area: points[0].rect_area(points[1]),
                                points,
                            }
                        )
                    } else {
                        None
                    }
                })
            );
            pairwise_areas.peek().unwrap().area
        }

        fn rect_is_valid(
            &self,
            points: &Vec<&Point>,
        ) -> bool {
            // TODO: implement
            true
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    #[repr(i8)]
    enum Direction {
        North = 0,
        East  = 1,
        South = 2,
        West  = 3,
    }
    
    impl Direction {
        fn orientation_turning_to(&self, other: &Self) -> Orientation {
            match (*self as i8 - *other as i8).rem_euclid(4) {
                3 => Orientation::Clockwise,
                1 => Orientation::Counterclockwise,
                _ => panic!("Comparign directions that are not exactly one 90 degree rotation apart"),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    enum Orientation {
        Clockwise,
        Counterclockwise,
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
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

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    struct DirectedSegment {
        segment: Segment,
        direction: Direction,
    }

    impl DirectedSegment {
        fn new(
            previous_point: Point,
            current_point: Point,
            direction: Direction,
        ) -> Self {
            let (low, high) = match direction {
                Direction::North | Direction::South => {
                    (
                        cmp::min(previous_point.row, current_point.row),
                        cmp::max(previous_point.row, current_point.row),
                    )
                },
                Direction::East | Direction::West => {
                    (
                        cmp::min(previous_point.col, current_point.col),
                        cmp::max(previous_point.col, current_point.col),
                    )
                },
            };
            Self {
                segment: Segment {
                    low,
                    high,
                },
                direction,
            }
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

pub mod part_two {
    use crate::utils::solution::{Answer, Solution};
    use super::utils::RestrictedTheater;

    #[derive(Debug, Default)]
    pub struct Soln {
        theater: RestrictedTheater,
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
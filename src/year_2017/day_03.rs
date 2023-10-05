#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 3 };

mod utils {
    use crate::utils::{io_utils, solution::Solution};

    pub trait Year2017Day03Solution {
        fn set_num(&mut self, num: u32);
    }

    pub fn parse_input_file<T>(soln: &mut T, filename: &str)
    where
        T: Solution + Year2017Day03Solution
    {
        soln.set_num(
            io_utils::file_to_string(filename)
                .parse::<u32>()
                .expect("File should be a single unsigned integer.")
        );
    }
    // TODO: test    
}

pub mod part_one {
    use crate::utils::solution::{Solution, Answer};
    use super::utils::{self, Year2017Day03Solution};

    #[derive(Default)]
    pub struct Soln {
        num: u32,
    }
 
    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            utils::parse_input_file(self, filename);
            let sqrt = (self.num as f64).sqrt().ceil() as u32;
            let shortest_distance_from_layer = sqrt / 2;
            let step_shortest_dist_multiplier = 2;
            let num_steps = 4;
            let dist_within_layer = if shortest_distance_from_layer > 0 {
                (shortest_distance_from_layer..(
                    shortest_distance_from_layer + 
                    shortest_distance_from_layer * step_shortest_dist_multiplier * (num_steps - 1) + 1
                ))
                    .step_by(
                        (shortest_distance_from_layer * step_shortest_dist_multiplier)
                            .try_into()
                            .unwrap()
                    )
                    .map(|cross_point| {
                        cross_point.abs_diff(
                            self.num - if sqrt % 2 == 0 { sqrt - 1 } else { sqrt - 2 }.pow(2)
                        )
                    })
                    .min()
                    .expect("Should be at least one cross point to find the distance to.")
                } else { 
                    0
                };
            Answer::U32(
                shortest_distance_from_layer + dist_within_layer
            )
        }
    }

    impl Year2017Day03Solution for Soln {
        fn set_num(&mut self, num: u32) {
            self.num = num;
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::test_utils;
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(0); "example_1")]
        #[test_case(2, Answer::U32(3); "example_2")]
        #[test_case(3, Answer::U32(2); "example_3")]
        #[test_case(4, Answer::U32(31); "example_4")]
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
    use std::collections::{HashSet, HashMap};
    use itertools::Itertools;
    use crate::utils::solution::{Solution, Answer};
    use super::utils::{self, Year2017Day03Solution};

    #[derive(PartialEq, Eq, Hash, Debug, Default, Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        pub fn neighbors(&self) -> HashSet<Point> {
            let mut points = HashSet::from_iter(
                ((self.x - 1)..(self.x + 2))
                    .cartesian_product((self.y - 1)..(self.y + 2))
                    .map(|(x, y)| Point { x, y })
            );
            assert!(points.remove(self));
            assert!(points.len() == 8);
            points
        }

        pub fn next_point(&self, direction: &Direction) -> Point {
            match *direction {
                Direction::North => Point { y: self.y + 1, ..*self },
                Direction::East => Point { x: self.x + 1, ..*self },
                Direction::South => Point { y: self.y - 1, ..*self },
                Direction::West => Point { x: self.x - 1, ..*self },
            }
        }
    }

    enum Direction {
        North,
        East,
        South,
        West,
    }

    impl Default for Direction {
        /// Default is East, the starting direction of the puzzle
        fn default() -> Self {
            Direction::East
        }        
    }

    impl Direction {        
        /// Returns the direction that is 90 degrees counterclockwise.
        fn rotate_90_degrees_cc(&self) -> Self {
            match *self {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            }
        }
    }

    #[derive(Default)]
    pub struct Soln {
        num: u32,
        point_values: HashMap<Point, u32>,
        point: Point,
        direction: Direction,
    }
 
    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            utils::parse_input_file(self, filename);
            self.point = Point { x: 0, y: 0};
            self.point_values.insert(self.point, 1);
            self.next_point();
            let mut next_num = 0;
            while next_num <= self.num {
                let neighbors = self.point.neighbors();
                next_num = 0;
                let mut neighbors_count = 0;
                for neighbor in &neighbors {
                    match self.point_values.get(neighbor) {
                        Some(&val) => {
                            next_num += val;
                            neighbors_count += 1;
                        },
                        None => (),
                    }
                }
                self.point_values.insert(self.point, next_num);
                self.advance_direction_and_point(neighbors_count);
            }
            Answer::U32(next_num)
        }
    }

    impl Year2017Day03Solution for Soln {
        fn set_num(&mut self, num: u32) {
            self.num = num;
        }
    }

    impl Soln {
        fn next_point(&mut self) {
            self.point = self.point.next_point(&self.direction);
        }

        fn advance_direction_and_point(&mut self, neighbors_count: u32) {
            // The sequence pattern has hit a corner when it has only 2 neighbors
            // already chosen: the one directly before it in the sequence and
            // the neighbor diagonally toward the origin. The very first
            // counter-clockwise turn must happen on the second added number when
            // there is only 1 neighbor (the only time there is only 1 neighbor).
            if neighbors_count <= 2 {
                self.direction = self.direction.rotate_90_degrees_cc();
            }
            self.next_point();
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test]
        fn neighbors_is_correct() {
            assert_eq!(
                Point { x: 0, y: 0 }.neighbors(),
                HashSet::from([
                    Point { x: 0, y: 1},
                    Point { x: 0, y: -1},
                    Point { x: 1, y: 0},
                    Point { x: -1, y: 0},
                    Point { x: -1, y: -1},
                    Point { x: -1, y: 1},
                    Point { x: 1, y: -1},
                    Point { x: 1, y: 1},
                ])
            );
        }

        #[test_case(1, Answer::U32(2); "example_1")]
        #[test_case(2, Answer::U32(23); "example_2")]
        #[test_case(3, Answer::U32(25); "example_3")]
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
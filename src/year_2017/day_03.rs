#[cfg(test)]
const YEAR: u32 = 2017;
#[cfg(test)]
const DAY: u8 = 3;

pub mod utils {
    use std::fs;

    pub fn parse_input_file(filename: &str) -> u32 {
        fs::read_to_string(filename)
            .expect("Should be able to read the file to a string.")
            .parse::<u32>()
            .expect("File should be a single unsigned integer.")
    }
    
    #[cfg(test)]
    mod tests {
        use std::collections::HashMap;
        use crate::utils::utils::{InputFileType, input_filename};
        use super::*;    
        use super::super::{YEAR, DAY};

        #[test]
        fn parse_input_file_is_correct() {
            let cases = HashMap::from([
                (1u8, 1u32),
                (2,   12),
                (3,   23),
                (4,   1024),
            ]);
            for (&example_key, &input_value) in &cases {
                assert_eq!(
                    parse_input_file(&input_filename(YEAR, DAY, InputFileType::Example(example_key))),
                    input_value
                );
            }
        }
    }
}

pub mod part_one {
    pub use either::*;
    use crate::utils::utils::Solution;
    use super::utils;

    #[derive(Default)]
    pub struct Soln {
        num: u32,
    }
 
    impl Solution for Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.num = utils::parse_input_file(filename);
        }

        fn solve(&mut self) -> Either<i32, &str> {
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
                        (cross_point as i32 - (
                            self.num - if sqrt % 2 == 0 { sqrt - 1 } else { sqrt - 2 }.pow(2)
                        ) as i32).abs() as u32
                    })
                    .min()
                    .expect("Should be at least one cross point to find the distance to.")
                } else { 
                    0
                };
            Left(
                (shortest_distance_from_layer + dist_within_layer) as i32
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use std::collections::HashMap;
        use crate::utils::utils::{InputFileType, input_filename};
        use super::*;
        use super::super::{YEAR, DAY};

        #[test]
        fn examples_are_correct() {
            let cases = HashMap::from([
                (1u8, 0),
                (2,   3),
                (3,   2),
                (4,   31),
            ]);
            for (&example_key, &answer) in &cases {
                let mut soln = Soln::default();
                soln.parse_input_file(&input_filename(YEAR, DAY, InputFileType::Example(example_key)));
                assert_eq!(
                    soln.solve().expect_left("Solution should be an integer."),
                    answer
                );
            }
        }
    }    
}

pub mod part_two {
    use std::collections::{HashSet, HashMap};
    use itertools::Itertools;
    pub use either::*;
    use crate::utils::utils::Solution;
    use super::utils;

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
        fn parse_input_file(&mut self, filename: &str) {
            self.num = utils::parse_input_file(filename);
        }

        fn solve(&mut self) -> Either<i32, &str> {
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
                            next_num = next_num + val;
                            neighbors_count = neighbors_count + 1;
                        },
                        None => (),
                    }
                }
                self.point_values.insert(self.point, next_num);
                self.advance_direction_and_point(neighbors_count);
            }
            Left(next_num as i32)
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
        use std::collections::HashMap;
        use crate::utils::utils::{InputFileType, input_filename};
        use super::*;
        use super::super::{YEAR, DAY};

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

        #[test]
        fn examples_are_correct() {
            let cases = HashMap::from([
                (1u8, 2),
                (2,   23),
                (3,   25),
            ]);
            for (&example_key, &answer) in &cases {
                let mut soln = Soln::default();
                soln.parse_input_file(&input_filename(YEAR, DAY, InputFileType::Example(example_key)));
                assert_eq!(
                    soln.solve().expect_left("Solution should be an integer."),
                    answer
                );
            }
        }
    }    
}
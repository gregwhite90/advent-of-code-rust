#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 10 };

mod utils {
    use std::{collections::HashSet, fmt::Display, ops::{Add, AddAssign, Sub}};

    use regex::Regex;

    use crate::utils::io_utils;

    pub struct BoundingBox {
        min: Vector,
        max: Vector,
    }

    impl BoundingBox {
        fn size(&self) -> Vector {
            self.max - self.min
        }

        pub fn height(&self) -> i64 {
            self.size().y
        }
    }

    #[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
    struct Vector {
        x: i64,
        y: i64,
    }

    impl AddAssign for Vector {
        fn add_assign(&mut self, other: Self) {
            *self = Self {
                x: self.x + other.x,
                y: self.y + other.y,
            };
        }
    }
    
    impl Add for Vector {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }    
        }
    }

    impl Sub for Vector {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }    
        }
    }

    #[derive(Debug, Default)]
    struct Point {
        position: Vector,
        velocity: Vector,
    }

    impl Point {
        fn step(&mut self) {
            self.position += self.velocity;
        }
    }

    #[derive(Debug)]
    pub struct Message {
        points: Vec<Point>,
        target_max_height: i64,
        seconds: usize,
    }

    impl Display for Message {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let bb = self.bounding_box();
            let points: HashSet<Vector> = self.points.iter().map(|point| point.position).collect();
            for y in bb.min.y..=bb.max.y {
                for x in bb.min.x..=bb.max.x {
                    if points.contains(&Vector { x, y }) {
                        write!(f, "#")?;
                    } else {
                        write!(f, ".")?;
                    }
                }
                write!(f, "\n")?;
            }
            Ok(())
        }
    }

    impl Message {
        pub fn with_target_max_height(target_max_height: i64) -> Self {
            Self {
                points: Vec::new(),
                target_max_height,
                seconds: 0,
            }
        }

        pub fn parse_input_file(&mut self, filename: &str) {
            let line_re = Regex::new(r"position=< *(?<pos_x>\-?\d+), *(?<pos_y>\-?\d+)> velocity=< *(?<vel_x>\-?\d+), *(?<vel_y>\-?\d+)>").unwrap();
            self.points = io_utils::file_to_lines(filename).map(|line| {
                let captures = line_re.captures(&line).unwrap();
                let position = Vector {
                    x: captures.name("pos_x").unwrap().as_str().parse().unwrap(),
                    y: captures.name("pos_y").unwrap().as_str().parse().unwrap(),
                };
                let velocity = Vector {
                    x: captures.name("vel_x").unwrap().as_str().parse().unwrap(),
                    y: captures.name("vel_y").unwrap().as_str().parse().unwrap(),
                };
                Point {
                    position,
                    velocity,
                }
            }).collect();
        }

        pub fn calculate_message(&mut self) {
            let mut bb = self.bounding_box();
            while bb.height() > self.target_max_height {
                self.step();
                bb = self.bounding_box();
            }
        }

        fn step(&mut self) {
            self.points.iter_mut().for_each(|point| point.step());
            self.seconds += 1;
        }

        fn bounding_box(&self) -> BoundingBox {
            let min_x = self.points.iter()
                .min_by_key(|point| point.position.x)
                .unwrap()
                .position.x;
            let max_x = self.points.iter()
                .max_by_key(|point| point.position.x)
                .unwrap()
                .position.x;
            let min_y = self.points.iter()
                .min_by_key(|point| point.position.y)
                .unwrap()
                .position.y;
            let max_y = self.points.iter()
                .max_by_key(|point| point.position.y)
                .unwrap()
                .position.y;
            BoundingBox {
                min: Vector { x: min_x, y: min_y },
                max: Vector { x: max_x, y: max_y },
            }
        }

        pub fn seconds(&self) -> usize {
            self.seconds
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::Message;

    #[derive(Debug)]
    pub struct Soln {
        message: Message,    
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.message.parse_input_file(filename);
            self.message.calculate_message();
            Answer::String(format!("{}", self.message))
        }
    }

    impl Default for Soln {
        fn default() -> Self {
            Self {
                // Height limits found with trial and error, not robust.
                message: Message::with_target_max_height(10),
            }
        }
    }
}

pub mod part_two {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::Message;

    #[derive(Debug)]
    pub struct Soln {
        message: Message,    
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.message.parse_input_file(filename);
            self.message.calculate_message();
            Answer::Usize(self.message.seconds())
        }
    }

    impl Soln {
        fn with_target_max_height(target_max_height: i64) -> Self {
            Self {
                message: Message::with_target_max_height(target_max_height),
            }
        }
    }

    impl Default for Soln {
        fn default() -> Self {
            Self::with_target_max_height(10)
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(3); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::with_target_max_height(8),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}
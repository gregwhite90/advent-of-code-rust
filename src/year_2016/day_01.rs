#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 1 };

pub mod part_one {
    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Turn {
        Left = -1,
        Right = 1,
    }

    impl Turn {
        fn from_str(input: &str) -> Self {
            match input {
                "L" => Self::Left,
                "R" => Self::Right,
                _ => panic!("Unrecognized Turn input string."),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Direction {
        North,
        East,
        South,
        West,
    }

    impl Direction {
        fn from_discriminant(discriminant: isize) -> Self {
            match discriminant {
                0 => Self::North,
                1 => Self::East,
                2 => Self::South,
                3 => Self::West,
                _ => panic!("Unrecognized Direction discriminant."),
            }
        }

        fn turn(&self, turn: Turn) -> Self {
            let offset = turn as isize;
            Self::from_discriminant((((*self as isize + offset) % 4) + 4) % 4)
        }
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        net_north: i32,
        net_east: i32,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::I32(self.net_north.abs() + self.net_east.abs())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let mut direction = Direction::North;
            let re = Regex::new(r"(?<turn>[LR])(?<steps>\d+)").unwrap();
            io_utils::file_to_string(filename).split(", ")
                .for_each(|instruction| {
                    let captures = re.captures(&instruction).unwrap();
                    let turn = Turn::from_str(captures.name("turn").unwrap().as_str());
                    direction = direction.turn(turn);
                    let steps: i32 = captures.name("steps").unwrap().as_str().parse().unwrap();
                    match direction {
                        Direction::North => self.net_north += steps,
                        Direction::East => self.net_east += steps,
                        Direction::South => self.net_north -= steps,
                        Direction::West => self.net_east -= steps,
                    }
            });
        }   
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::I32(5); "example_1")]
        #[test_case(2, Answer::I32(2); "example_2")]
        #[test_case(3, Answer::I32(12); "example_3")]
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
    use std::collections::HashSet;

    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Turn {
        Left = -1,
        Right = 1,
    }

    impl Turn {
        fn from_str(input: &str) -> Self {
            match input {
                "L" => Self::Left,
                "R" => Self::Right,
                _ => panic!("Unrecognized Turn input string."),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Direction {
        North,
        East,
        South,
        West,
    }

    impl Direction {
        fn from_discriminant(discriminant: isize) -> Self {
            match discriminant {
                0 => Self::North,
                1 => Self::East,
                2 => Self::South,
                3 => Self::West,
                _ => panic!("Unrecognized Direction discriminant."),
            }
        }

        fn turn(&self, turn: Turn) -> Self {
            let offset = turn as isize;
            Self::from_discriminant((((*self as isize + offset) % 4) + 4) % 4)
        }
    }

    #[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        fn manhattan_distance(&self, other: &Self) -> i32 {
            (self.x - other.x).abs() + (self.y - other.y).abs()
        }

        fn step(&self, direction: Direction) -> Self { 
            let x = self.x + match direction {
                Direction::East => 1,
                Direction::West => -1,
                _ => 0,
            };
            let y = self.y + match direction {
                Direction::North => 1,
                Direction::South => -1,
                _ => 0,
            };
            Point { x, y }
        }
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        first_location_visited_twice: Point,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::I32(self.first_location_visited_twice.manhattan_distance(&Point { x: 0, y: 0 }))
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let mut position = Point { x: 0, y: 0 };
            let mut visited: HashSet<Point> = HashSet::from([position]);
            let mut direction = Direction::North;
            let re = Regex::new(r"(?<turn>[LR])(?<steps>\d+)").unwrap();
            for instruction in io_utils::file_to_string(filename).split(", ") {
                let captures = re.captures(&instruction).unwrap();
                let turn = Turn::from_str(captures.name("turn").unwrap().as_str());
                direction = direction.turn(turn);
                let steps: i32 = captures.name("steps").unwrap().as_str().parse().unwrap();
                for _ in 0..steps {
                    position = position.step(direction);
                    if visited.contains(&position) {
                        self.first_location_visited_twice = position;
                        return;
                    }
                    visited.insert(position);
                }
            };
        }   
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(4, Answer::I32(4); "example_4")]
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
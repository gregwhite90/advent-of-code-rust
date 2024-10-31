#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 23 };

mod utils {
    use regex::Regex;

    use lazy_static::lazy_static;

    use crate::utils::io_utils;


    lazy_static! {
        static ref NANOBOT_RE: Regex = Regex::new(r"pos=<(?<x>\-?\d+),(?<y>\-?\d+),(?<z>\-?\d+)>, r=(?<r>\d+)").unwrap();
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    struct Position {
        x: isize,
        y: isize,
        z: isize,
    }

    impl Position {
        pub fn manhattan_distance(&self, other: &Self) -> usize {
            self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    struct Nanobot {
        position: Position,
        radius: usize,
    }

    impl Nanobot {
        pub fn from_str(input: &str) -> Self {
            let caps = NANOBOT_RE.captures(input).unwrap();
            let x = caps.name("x").unwrap().as_str().parse().unwrap();
            let y = caps.name("y").unwrap().as_str().parse().unwrap();
            let z = caps.name("z").unwrap().as_str().parse().unwrap();
            let radius = caps.name("r").unwrap().as_str().parse().unwrap();
            Self {
                position: Position { x, y, z },
                radius,
            }
        }

        pub fn manhattan_distance(&self, other: &Self) -> usize {
            self.position.manhattan_distance(&other.position)
        }

        pub fn other_is_in_range(&self, other: &Self) -> bool {
            self.manhattan_distance(other) <= self.radius
        }
    }

    #[derive(Debug, Default)]
    pub struct Cavern {
        nanobots: Vec<Nanobot>,
    }

    impl Cavern {
        pub fn parse_input_file(&mut self, filename: &str) {
            self.nanobots = io_utils::file_to_lines(filename).map(|line| Nanobot::from_str(&line)).collect();
        }

        pub fn nanobots_in_range_of_largest_signal_radius(&self) -> usize {
            let nanobot_with_largest_signal_radius = self.nanobots.iter().max_by_key(|n| n.radius).unwrap();
            self.nanobots.iter().filter(|n| nanobot_with_largest_signal_radius.other_is_in_range(n)).count()
        }

        pub fn distance_to_nearest_position_in_range_of_most_nanobots(&self) -> usize {
            // TODO
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::Cavern;

    #[derive(Debug, Default)]
    pub struct Soln {
        cavern: Cavern,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.cavern.parse_input_file(filename);
            Answer::Usize(self.cavern.nanobots_in_range_of_largest_signal_radius())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(7); "example_1")]
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

    use super::utils::Cavern;

    #[derive(Debug, Default)]
    pub struct Soln {
        cavern: Cavern,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.cavern.parse_input_file(filename);
            Answer::Usize(self.cavern.distance_to_nearest_position_in_range_of_most_nanobots())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(2, Answer::Usize(36); "example_2")]
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
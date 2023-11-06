#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 19 };

mod utils {
    use std::collections::HashMap;

    use crate::utils::io_utils;

    #[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Copy)]
    struct Point {
        row: usize,
        col: usize,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Direction {
        North,
        East,
        South,
        West,
    }

    impl Default for Direction {
        fn default() -> Self {
            Self::South
        }
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct PacketRouter {
        found: String,
        position: Point,
        direction: Direction,
        map: HashMap<Point, char>,
        steps: u32,
    }

    impl PacketRouter {
        pub fn parse_input_file(&mut self, filename: &str) {
            io_utils::file_to_lines(filename)
                .enumerate()
                .for_each(|(row, line)| {
                    line.chars()
                        .enumerate()
                        .for_each(|(col, c)| {
                            if c == ' ' { return; }
                            let pt = Point{ row, col };
                            self.map.insert(pt, c);                            
                            if row == 0 { self.position = pt};
                        })
                });
        }

        fn next_pos(&self, direction: Direction) -> Point {
            match direction {
                Direction::North => Point { row: self.position.row - 1, col: self.position.col },
                Direction::East => Point { row: self.position.row, col: self.position.col + 1 },
                Direction::South => Point { row: self.position.row + 1, col: self.position.col },
                Direction::West => Point { row: self.position.row, col: self.position.col - 1 },
            }            
        }

        /// Returns whether it's finished
        fn advance_pos(&mut self) -> bool {
            match self.map.get(&self.position) {
                None => return true,
                Some('+') => {
                    // turning. find the dir.
                    self.direction = match self.direction {
                        Direction::North | Direction::South => {
                            if let Some(_) = self.map.get(&self.next_pos(Direction::East)) { 
                                Direction::East
                            } else {
                                self.map.get(&self.next_pos(Direction::West)).unwrap();
                                Direction::West
                            }
                        },
                        Direction::East | Direction::West => {
                            if let Some(_) = self.map.get(&self.next_pos(Direction::North)) { 
                                Direction::North
                            } else {
                                self.map.get(&self.next_pos(Direction::South)).unwrap();
                                Direction::South
                            }
                        },
                    }
                },
                Some('|') | Some('-') => (),
                Some(c) => {
                    self.found.push(*c);
                }
            }
            self.steps += 1;
            self.position = self.next_pos(self.direction);
            false
        }

        pub fn run(&mut self) {
            while !self.advance_pos() {}
        }

        pub fn found(&self) -> String {
            self.found.clone()
        }

        pub fn steps(&self) -> u32 {
            self.steps
        }
    }

    // TODO: tests

}

pub mod part_one {
    use crate::utils::solution::{Solution, Answer};

    use super::utils::PacketRouter;


    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        packet_router: PacketRouter,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.packet_router.parse_input_file(filename);
            self.packet_router.run();
            Answer::String(self.packet_router.found())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::String(String::from("ABCDEF")); "example_1")]
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
    use crate::utils::solution::{Solution, Answer};

    use super::utils::PacketRouter;

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        packet_router: PacketRouter,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.packet_router.parse_input_file(filename);
            self.packet_router.run();
            Answer::U32(self.packet_router.steps())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(38); "example_1")]
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
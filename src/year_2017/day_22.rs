#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 22 };

mod utils {
    use std::collections::HashMap;

    use crate::utils::io_utils;

    #[derive(Debug, PartialEq, Eq)]
    pub enum Part {
        One,
        Two,
    }

    #[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        fn forward(&mut self, direction: &Direction) {
            match direction {
                Direction::Up => self.y -= 1,
                Direction::Down => self.y += 1,
                Direction::Left => self.x -= 1,
                Direction::Right => self.x += 1,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    #[repr(i8)]
    enum Direction {
        Up = 0,
        Right = 1,
        Down = 2,
        Left = 3,
    }

    impl Default for Direction {
        fn default() -> Self {
            Self::Up
        }
    }

    impl Direction {
        fn from_val(val: i8) -> Self {
            match val {
                0 => Self::Up,
                1 => Self::Right,
                2 => Self::Down,
                3 => Self::Left,
                _ => panic!("Unknown value for Direction."),
            }
        }

        fn turn(&self, turn_direction: TurnDirection) -> Self {
            let val = (*self as i8 + turn_direction as i8).rem_euclid(4);
            Self::from_val(val)
        }
    }

    #[derive(Debug, Clone, Copy)]
    #[repr(i8)]
    enum TurnDirection {
        Left = -1,
        Right = 1,
        Reverse = 2,
        None = 0,
    }


    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum NodeStatus {
        Weakened,
        Infected,
        Flagged,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct VirusCarrier {
        position: Point,
        direction: Direction,
        infected: HashMap<Point, NodeStatus>,
        bursts_causing_infection: u32,
        bursts: u32,
        part: Part,
    }

    impl VirusCarrier {
        pub fn new(bursts: u32, part: Part)  -> Self {
            Self {
                position: Point::default(),
                direction: Direction::default(),
                infected: HashMap::default(),
                bursts_causing_infection: 0,
                bursts,
                part,
            }
        }

        pub fn parse_input_file(&mut self, filename: &str) {
            let mut rows = 0;
            let mut cols = 0;
            io_utils::file_to_lines(filename)
                .enumerate()
                .for_each(|(row, line)| {
                    rows += 1;
                    line.chars()
                        .enumerate()
                        .for_each(|(col, c)| {
                            if row == 0 { cols += 1; } 
                            if c == '#' { 
                                self.infected.insert(
                                    Point { 
                                        x: col.try_into().unwrap(),
                                        y: row.try_into().unwrap(),
                                    },
                                    NodeStatus::Infected,
                                ); 
                            }
                        })
                });
            self.position = Point { 
                x: (cols - 1) / 2,
                y: (rows - 1) / 2,
            }
        }

        pub fn burst_all(&mut self) {
            for _ in 0..self.bursts {
                self.burst();
            }
        }
        
        fn burst(&mut self) {
            self.turn();
            match self.infected.get(&self.position) {
                None => {
                    match self.part {
                        Part::One => {
                            self.infected.insert(self.position, NodeStatus::Infected);
                            self.bursts_causing_infection += 1;                            
                        },
                        Part::Two => {
                            self.infected.insert(self.position, NodeStatus::Weakened);
                        },
                    }
                },
                Some(NodeStatus::Weakened) => {
                    self.infected.insert(self.position, NodeStatus::Infected);
                    self.bursts_causing_infection += 1;
                },
                Some(NodeStatus::Infected) => {
                    match self.part {
                        Part::One => {
                            self.infected.remove(&self.position);
                        },
                        Part::Two => {
                            self.infected.insert(self.position, NodeStatus::Flagged);
                        }
                    }
                },
                Some(NodeStatus::Flagged) => {
                    self.infected.remove(&self.position);
                },
            };
            self.forward();
        }

        pub fn bursts_causing_infection(&self) -> u32 {
            self.bursts_causing_infection
        }

        fn turn(&mut self) {
            let turn_direction = match self.infected.get(&self.position) {
                None => TurnDirection::Left,
                Some(NodeStatus::Weakened) => TurnDirection::None,
                Some(NodeStatus::Infected) => TurnDirection::Right,
                Some(NodeStatus::Flagged) => TurnDirection::Reverse,
            };
            self.direction = self.direction.turn(turn_direction);
        }

        fn forward(&mut self) {
            self.position.forward(&self.direction);
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Solution, Answer};

    use super::utils::{VirusCarrier, Part};

    #[derive(Debug, PartialEq, Eq)]
    pub struct Soln {
        virus_carrier: VirusCarrier,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self {
                virus_carrier: VirusCarrier::new(10_000, Part::One),
            }
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.virus_carrier.parse_input_file(filename);
            self.virus_carrier.burst_all();
            Answer::U32(self.virus_carrier.bursts_causing_infection())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(5587); "example_1")]
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

    use super::utils::{VirusCarrier, Part};

    #[derive(Debug, PartialEq, Eq)]
    pub struct Soln {
        virus_carrier: VirusCarrier,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self {
                virus_carrier: VirusCarrier::new(10_000_000, Part::Two),
            }
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.virus_carrier.parse_input_file(filename);
            self.virus_carrier.burst_all();
            Answer::U32(self.virus_carrier.bursts_causing_infection())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(2_511_944); "example_1")]
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
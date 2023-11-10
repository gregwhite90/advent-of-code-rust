#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 22 };

pub mod part_one {
    use std::collections::HashSet;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

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
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct Soln {
        position: Point,
        direction: Direction,
        infected: HashSet<Point>,
        bursts_causing_infection: u32,
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
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
                                    }
                                ); 
                            }
                        })
                });
            self.position = Point { 
                x: (cols - 1) / 2,
                y: (rows - 1) / 2,
            }
        }

        fn burst(&mut self) {
            self.turn();
            match self.infected.contains(&self.position) {
                true => {
                    self.infected.remove(&self.position);
                },
                false => {
                    self.infected.insert(self.position);
                    self.bursts_causing_infection += 1;
                },
            };
            self.forward();
        }

        fn bursts_causing_infection(&self) -> u32 {
            self.bursts_causing_infection
        }

        fn turn(&mut self) {
            let turn_direction = match self.infected.contains(&self.position) {
                true => TurnDirection::Right,
                false => TurnDirection::Left,
            };
            self.direction = self.direction.turn(turn_direction);
        }

        fn forward(&mut self) {
            self.position.forward(&self.direction);
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            for _ in 0..10000 {
                self.burst();
            }
            Answer::U32(self.bursts_causing_infection())
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
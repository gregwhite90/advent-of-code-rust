#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2024, day: 6 };

mod utils {
    use std::{cmp::max, collections::HashSet};

    use crate::utils::io_utils;

    #[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
    pub struct Point {
        row: usize,
        col: usize,
    }

    #[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
    enum Direction {
        #[default] Up,
        Right,
        Down,
        Left,
    }

    impl Direction {
        fn from_char(input: char) -> Self {
            match input {
                '^' => Self::Up,
                '>' => Self::Right,
                'v' => Self::Down,
                '<' => Self::Left,
                _ => panic!("Unrecognized character for direction."),
            }
        }

        fn turn(&mut self) {
            *self = match self {
                Self::Up => Self::Right,
                Self::Right => Self::Down,
                Self::Down => Self::Left,
                Self::Left => Self::Up,
            }
        }
    }

    #[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
    struct GuardStatus {
        pos: Point,
        dir: Direction,
    }

    impl GuardStatus {
        fn point_in_path(&self, point: &Point) -> bool {
            match self.dir {
                Direction::Up => self.pos.col == point.col && self.pos.row > point.row,
                Direction::Right => self.pos.row == point.row && self.pos.col < point.col,
                Direction::Down => self.pos.col == point.col && self.pos.row < point.row,
                Direction::Left => self.pos.row == point.row && self.pos.col > point.col,
            }
        }
    }

    #[derive(Debug, Default, Clone)]
    pub struct LabMap {
        rows: usize,
        cols: usize,
        guard_start: GuardStatus,
        guard_status: GuardStatus,
        obstacles: HashSet<Point>,
    }

    impl LabMap {
        pub fn parse_input_file(&mut self, filename: &str) {
            io_utils::file_to_lines(filename)
                .enumerate()
                .for_each(|(row, line)| {
                    self.rows = row + 1;
                    self.cols = max(line.len(), self.cols);
                    line.char_indices()
                        .for_each(|(col, ch)| {
                            match ch {
                                '.' => (),
                                '#' => { self.obstacles.insert(Point { row, col }); },
                                '^' | '>' | 'v' | '<' => {
                                    self.guard_start = GuardStatus { pos: Point { row, col }, dir: Direction::from_char(ch) };
                                    self.guard_status = self.guard_start.clone();
                                },
                                _ => panic!("Unrecognized character."),
                            }
                        });
                });
        }
        
        pub fn add_obstacle(&mut self, point: Point) {
            self.obstacles.insert(point);
        }

        fn next_obstacle(&self) -> Option<&Point> {
            let obstacles_in_path = self.obstacles.iter()
                .filter(|pt| {
                    self.guard_status.point_in_path(pt)
                });
            match self.guard_status.dir {
                Direction::Up => obstacles_in_path.max_by_key(|obstacle| obstacle.row),
                Direction::Right => obstacles_in_path.min_by_key(|obstacle| obstacle.col),
                Direction::Down => obstacles_in_path.min_by_key(|obstacle| obstacle.row),
                Direction::Left => obstacles_in_path.max_by_key(|obstacle| obstacle.col),
            }
        }

        /// Returns a tuple of whether the guard is stuck in a loop and the set of points visited
        pub fn positions_visited(&mut self) -> (bool, HashSet<Point>) {
            let mut visited: HashSet<Point> = HashSet::new();
            let mut guard_statuses: HashSet<GuardStatus> = HashSet::new();
            loop {
                if guard_statuses.contains(&self.guard_status) {
                    return (true, visited);
                }
                guard_statuses.insert(self.guard_status.clone());
                if let Some(&next_obstacle) = self.next_obstacle() {
                    // Move to next obstacle, adding all points along way to visited
                    match self.guard_status.dir {
                        Direction::Up => {
                            for row in next_obstacle.row + 1..=self.guard_status.pos.row {
                                visited.insert(Point { row, col: self.guard_status.pos.col });
                            }
                            self.guard_status.pos = Point { row: next_obstacle.row + 1, col: self.guard_status.pos.col };
                            self.guard_status.dir.turn();
                        },
                        Direction::Right => {
                            for col in self.guard_status.pos.col..next_obstacle.col {
                                visited.insert(Point { row: self.guard_status.pos.row, col });
                            }
                            self.guard_status.pos = Point { row: self.guard_status.pos.row, col: next_obstacle.col - 1 };
                            self.guard_status.dir.turn();
                        },
                        Direction::Down => {
                            for row in self.guard_status.pos.row..next_obstacle.row {
                                visited.insert(Point { row, col: self.guard_status.pos.col });
                            }
                            self.guard_status.pos = Point { row: next_obstacle.row - 1, col: self.guard_status.pos.col };
                            self.guard_status.dir.turn();
                        },
                        Direction::Left => {
                            for col in next_obstacle.col + 1..=self.guard_status.pos.col {
                                visited.insert(Point { row: self.guard_status.pos.row, col });
                            }
                            self.guard_status.pos = Point { row: self.guard_status.pos.row, col: next_obstacle.col + 1 };
                            self.guard_status.dir.turn();
                        },
                    }
                } else {
                    // Going off the map
                    match self.guard_status.dir {
                        Direction::Up => {
                            for row in 0..=self.guard_status.pos.row {
                                visited.insert(Point { row, col: self.guard_status.pos.col });
                            }
                        },
                        Direction::Right => {
                            for col in self.guard_status.pos.col..self.cols {
                                visited.insert(Point { row: self.guard_status.pos.row, col });
                            }
                        },
                        Direction::Down => {
                            for row in self.guard_status.pos.row..self.rows {
                                visited.insert(Point { row, col: self.guard_status.pos.col });
                            }
                        },
                        Direction::Left => {
                            for col in 0..=self.guard_status.pos.col {
                                visited.insert(Point { row: self.guard_status.pos.row, col });
                            }
                        },
                    }
                    return (false, visited);
                }
            }
        }

        pub fn num_positions_visited(&mut self) -> usize {
            self.positions_visited().1.len()
        }

        pub fn guard_start_pos(&self) -> Point {
            self.guard_start.pos.clone()
        }

        pub fn is_looping(&mut self) -> bool {
            self.positions_visited().0
        }
        
        pub fn reset_guard_status(&mut self) {
            self.guard_status = self.guard_start.clone();
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::LabMap;

    #[derive(Debug, Default)]
    pub struct Soln {
        lab_map: LabMap,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.lab_map.parse_input_file(filename);
            Answer::Usize(self.lab_map.num_positions_visited())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(41); "example_1")]
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

    use super::utils::LabMap;

    #[derive(Debug, Default)]
    pub struct Soln {}

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            let mut base_lab_map = LabMap::default();
            base_lab_map.parse_input_file(filename);
            let mut candidate_obstacle_additions = base_lab_map.positions_visited().1;
            base_lab_map.reset_guard_status();
            candidate_obstacle_additions.remove(&base_lab_map.guard_start_pos());
            Answer::Usize(
                candidate_obstacle_additions.into_iter()
                    .filter(|obstacle| {
                        let mut lab_map = base_lab_map.clone();
                        lab_map.add_obstacle(obstacle.to_owned());
                        lab_map.is_looping()
                    })
                    .count()
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(6); "example_1")]
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
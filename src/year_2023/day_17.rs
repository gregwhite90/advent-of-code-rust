#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 17 };

mod utils { 
    use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}};

    use crate::utils::io_utils;

    #[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
    struct Point {
        row: usize,
        col: usize,
    }

    impl Point {
        fn manhattan_dist_from_origin(&self) -> usize {
            self.row + self.col
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    enum Direction {
        North,
        East,
        South,
        West,
    }

    impl Direction {
        fn opposite(&self) -> Self {
            match self {
                Direction::North => Direction::South,
                Direction::South => Direction::North,
                Direction::East => Direction::West,
                Direction::West => Direction::East,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Path {
        heat_loss: u32,
        position: Point,
        direction: Direction,
        straight_steps: u8,
    }

    /// Because our priority queue is a max heap, this orders by priority to be popped from the queue.
    impl Ord for Path {
        fn cmp(&self, other: &Self) -> Ordering {
            match other.heat_loss.cmp(&self.heat_loss) {
                Ordering::Equal => {
                    match self.position.manhattan_dist_from_origin().cmp(&other.position.manhattan_dist_from_origin()) {
                        Ordering::Equal => other.straight_steps.cmp(&self.straight_steps),
                        comparison => comparison,
                    }
                },
                comparison => comparison,
            }
        }
    }

    impl PartialOrd for Path {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Path {
        fn next_straight_steps(&self, direction: &Direction, min_steps: u8) -> u8 {
            if self.direction == *direction {
                self.straight_steps + 1
            } else {
                min_steps
            }
        }

        fn orientation(&self) -> Orientation {
            Orientation {
                position: self.position,
                direction: self.direction,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    struct Orientation {
        position: Point,
        direction: Direction,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct PathFinder {
        min_steps: u8,
        max_steps: u8,
        heat_loss_map: Vec<Vec<u32>>,
    }

    impl PathFinder {
        pub fn new(min_steps: u8, max_steps: u8) -> Self {
            Self {
                min_steps,
                max_steps,
                heat_loss_map: Vec::new(),
            }
        }

        pub fn parse_input_file(&mut self, filename: &str) {
            self.heat_loss_map = io_utils::file_to_lines(filename)
                .map(|line| {
                    line.chars().map(|ch| ch.to_digit(10).expect("Input must be a number.")).collect()
                })
                .collect();
        }

        fn rows(&self) -> usize {
            self.heat_loss_map.len()
        }

        fn cols(&self) -> usize {
            self.heat_loss_map[0].len()
        }

        fn finished(&self, path: &Path) -> bool {
            path.position == Point { row: self.rows() - 1, col: self.cols() - 1 }
        }

        fn heat_loss(&self, position: &Point) -> u32 {
            self.heat_loss_map[position.row][position.col]
        }

        pub fn minimum_heat_loss(&self) -> u32 {
            let mut pq = BinaryHeap::new();
            let mut min_straight_steps_seen = HashMap::new();
            let path = Path {
                heat_loss: (1..=self.min_steps as usize).map(|step| self.heat_loss(&Point { row: 0, col: step })).sum(),
                position: Point { row: 0, col: self.min_steps as usize },
                direction: Direction::East,
                straight_steps: self.min_steps,
            };
            pq.push(path);
            let path = Path {
                heat_loss: (1..=self.min_steps as usize).map(|step| self.heat_loss(&Point { row: step, col: 0 })).sum(),
                position: Point { row: self.min_steps as usize, col: 0 },
                direction: Direction::South,
                straight_steps: self.min_steps,
            };
            pq.push(path);
            loop {
                let path = pq.pop().expect("Priority queue should be populated unless we've reached the finish.");
                if self.finished(&path) { return path.heat_loss; }
                if let Some(min_straight_steps) = min_straight_steps_seen.get(&path.orientation()) {
                    if path.straight_steps >= *min_straight_steps {
                        continue;
                    }
                }
                min_straight_steps_seen.insert(path.orientation(), path.straight_steps);
                pq.append(&mut self.path_options(path));
            }
        }

        fn next_position(&self, path: &Path, direction: Direction) -> Option<(Point, u32)> {
            if path.direction == direction.opposite() || path.direction == direction && path.straight_steps == self.max_steps {
                return None;
            } else {
                let steps = if path.direction == direction { 1 } else { self.min_steps as usize };                
                match direction {
                    Direction::North => {
                        if path.position.row < steps {
                            None
                        } else {
                            let pt = Point { row: path.position.row - steps, col: path.position.col };
                            let addl_heat_loss = (1..=steps)
                                .map(|step| {
                                    let point = Point { row: path.position.row - step, col: path.position.col };
                                    self.heat_loss(&point)
                                })
                                .sum();
                            Some((pt, addl_heat_loss))
                        }
                    },
                    Direction::South => {
                        if path.position.row > self.rows() - 1 - steps { // TODO: figure out how to make work with steps=1
                            None
                        } else {
                            let pt = Point { row: path.position.row + steps, col: path.position.col };
                            let addl_heat_loss = (1..=steps)
                                .map(|step| {
                                    let point = Point { row: path.position.row + step, col: path.position.col };
                                    self.heat_loss(&point)
                                })
                                .sum();
                            Some((pt, addl_heat_loss))
                        }
                    },
                    Direction::East => {
                        if path.position.col > self.cols() - 1 - steps { // TODO: figure out how to make work with steps=1
                            None
                        } else {
                            let pt = Point { row: path.position.row, col: path.position.col + steps };
                            let addl_heat_loss = (1..=steps)
                                .map(|step| {
                                    let point = Point { row: path.position.row, col: path.position.col + step };
                                    self.heat_loss(&point)
                                })
                                .sum();
                            Some((pt, addl_heat_loss))
                        }
                    },
                    Direction::West => {
                        if path.position.col < steps {
                            None
                        } else {
                            let pt = Point { row: path.position.row, col: path.position.col - steps };
                            let addl_heat_loss = (1..=steps)
                                .map(|step| {
                                    let point = Point { row: path.position.row, col: path.position.col - step };
                                    self.heat_loss(&point)
                                })
                                .sum();
                            Some((pt, addl_heat_loss))
                        }
                    }
                }
            }
        }

        fn path_option(&self, path: &Path, direction: Direction) -> Option<Path> {
            if let Some((position, addl_heat_loss)) = self.next_position(path, direction) {
                return Some(Path {
                    heat_loss: path.heat_loss + addl_heat_loss,
                    position,
                    direction,
                    straight_steps: path.next_straight_steps(&direction, self.min_steps),
                })
            }
            None
        }

        fn path_options(&self, path: Path) -> BinaryHeap<Path> {
            let mut options = BinaryHeap::new();
            // Could iterate over elements of enum, but would need a third party crate `strum`
            if let Some(path_option) = self.path_option(&path, Direction::North) {
                options.push(path_option);
            }
            if let Some(path_option) = self.path_option(&path, Direction::South) {
                options.push(path_option);
            }
            if let Some(path_option) = self.path_option(&path, Direction::East) {
                options.push(path_option);
            }
            if let Some(path_option) = self.path_option(&path, Direction::West) {
                options.push(path_option);
            }
            options
        }
    }
}

pub mod part_one {

    use crate::utils::solution::{Answer, Solution};

    use super::utils::PathFinder;

    #[derive(Debug, PartialEq, Eq)]
    pub struct Soln {
        path_finder: PathFinder,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self {
                path_finder: PathFinder::new(1, 3),
            }
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.minimum_heat_loss())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.path_finder.parse_input_file(filename);
        }

        fn minimum_heat_loss(&self) -> u32 {
            self.path_finder.minimum_heat_loss()
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(102); "example_1")]
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

    use super::utils::PathFinder;

    #[derive(Debug, PartialEq, Eq)]
    pub struct Soln {
        path_finder: PathFinder,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self {
                path_finder: PathFinder::new(4, 10),
            }
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.minimum_heat_loss())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.path_finder.parse_input_file(filename);
        }

        fn minimum_heat_loss(&self) -> u32 {
            self.path_finder.minimum_heat_loss()
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(94); "example_1")]
        #[test_case(2, Answer::U32(71); "example_2")]
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
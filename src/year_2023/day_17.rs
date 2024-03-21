#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 17 };

pub mod part_one {

    use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}};

    use crate::utils::{io_utils, solution::{Answer, Solution}};

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
        fn next_straight_steps(&self, direction: &Direction) -> u8 {
            if self.direction == *direction {
                self.straight_steps + 1
            } else {
                1
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

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct Soln {
        heat_loss_map: Vec<Vec<u32>>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.minimum_heat_loss())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
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

        fn minimum_heat_loss(&self) -> u32 {
            let mut pq = BinaryHeap::new();
            let mut min_straight_steps_seen = HashMap::new();
            let path = Path {
                heat_loss: 0,
                position: Point { row: 0, col: 0 },
                direction: Direction::East,
                straight_steps: 0,
            };
            pq.push(path);
            let path = Path {
                heat_loss: 0,
                position: Point { row: 0, col: 0 },
                direction: Direction::South,
                straight_steps: 0,
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

        fn next_position(&self, position: Point, direction: Direction) -> Option<Point> {
            match direction {
                Direction::North => {
                    if position.row == 0 {
                        None
                    } else {
                        Some(Point { row: position.row - 1, col: position.col })
                    }
                },
                Direction::South => {
                    if position.row == self.rows() - 1 {
                        None
                    } else {
                        Some(Point { row: position.row + 1, col: position.col })
                    }
                },
                Direction::East => {
                    if position.col == self.cols() - 1 {
                        None
                    } else {
                        Some(Point { row: position.row, col: position.col + 1 })
                    }
                },
                Direction::West => {
                    if position.col == 0 {
                        None
                    } else {
                        Some(Point { row: position.row, col: position.col - 1 })
                    }
                }
            }
        }

        fn path_option(&self, path: &Path, direction: Direction) -> Option<Path> {
            if path.direction != direction.opposite()
                && !(path.direction == direction && path.straight_steps == 3) {
                    if let Some(position) = self.next_position(path.position, direction) {
                        return Some(Path {
                            heat_loss: path.heat_loss + self.heat_loss(&position),
                            position,
                            direction,
                            straight_steps: path.next_straight_steps(&direction),
                        })
                    }
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
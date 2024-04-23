#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 21 };

pub mod part_one {

    use std::collections::{HashMap, HashSet, VecDeque};

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
    struct Point {
        row: usize,
        col: usize,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct Path {
        point: Point,
        steps: usize,
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        rows: usize,
        cols: usize,
        start: Point,
        rocks: HashSet<Point>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            let mut shortest_paths: HashMap<Point, usize> = HashMap::new();
            let mut queue = VecDeque::from([
                Path {
                    point: self.start,
                    steps: 0,
                },
            ]);
            while !queue.is_empty() {
                let path = queue.pop_front().unwrap();
                if shortest_paths.contains_key(&path.point) { continue; }
                shortest_paths.insert(path.point, path.steps);
                queue.append(&mut self.next_paths(&path));
            }
            Answer::Usize(
                shortest_paths.values()
                    .filter(|steps| **steps % 2 == 0 && **steps <= 64)
                    .count()
            )
        }
    }

    impl Soln {
        fn next_paths(&self, path: &Path) -> VecDeque<Path> {
            let mut result = VecDeque::new();
            if path.point.row != 0 {
                let point = Point { 
                    row: path.point.row - 1,
                    col: path.point.col,
                };
                if !self.rocks.contains(&point) {
                    result.push_back(Path { point, steps: path.steps + 1 });
                }
            }
            if path.point.col != 0 {
                let point = Point { 
                    row: path.point.row,
                    col: path.point.col - 1, 
                };
                if !self.rocks.contains(&point) {
                    result.push_back(Path { point, steps: path.steps + 1 });
                }
            }
            if path.point.row != self.rows - 1 {
                let point = Point { 
                    row: path.point.row + 1,
                    col: path.point.col, 
                };
                if !self.rocks.contains(&point) {
                    result.push_back(Path { point, steps: path.steps + 1 });
                }
            }
            if path.point.col != self.cols - 1 {
                let point = Point { 
                    row: path.point.row,
                    col: path.point.col + 1, 
                };
                if !self.rocks.contains(&point) {
                    result.push_back(Path { point, steps: path.steps + 1 });
                }
            }
            result
        }

        fn parse_input_file(&mut self, filename: &str) {
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    self.cols = line.len();
                    self.rocks.extend(
                        line.char_indices()
                            .filter(|(_col, ch)| *ch == '#')
                            .map(|(col, _ch)| Point { row: self.rows, col })
                    );
                    if let Some(col) = line.find('S') {
                        self.start = Point { row: self.rows, col };
                    }
                    self.rows += 1;
                });
        }

    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(2, Answer::Usize(3_639); "full_input")]
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

/// This solution is based on [this explanation](https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21).
pub mod part_two {

    use std::collections::{HashMap, HashSet, VecDeque};

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
    struct Point {
        row: usize,
        col: usize,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct Path {
        point: Point,
        steps: usize,
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        rows: usize,
        cols: usize,
        start: Point,
        rocks: HashSet<Point>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            let mut shortest_paths: HashMap<Point, usize> = HashMap::new();
            let mut queue = VecDeque::from([
                Path {
                    point: self.start,
                    steps: 0,
                },
            ]);
            while !queue.is_empty() {
                let path = queue.pop_front().unwrap();
                if shortest_paths.contains_key(&path.point) { continue; }
                shortest_paths.insert(path.point, path.steps);
                queue.append(&mut self.next_paths(&path));
            }
            /*  
            This solution relies on the input having these special case
            properties as outlined in the explanation linked at the
            beginning of this mod.
            */
            // Input is a square
            assert_eq!(self.rows, self.cols);
            // Input dimensions are odd
            assert_eq!(self.rows % 2, 1);
            let steps_to_edge = (self.rows - 1) / 2;
            // Start point is in the middle of the input
            assert_eq!(self.start, Point { row: steps_to_edge, col: steps_to_edge });
            // Start row and col are unobstructed
            assert_eq!(0, self.rocks.iter().filter(|pt| pt.row == self.start.row).count());
            assert_eq!(0, self.rocks.iter().filter(|pt| pt.row == self.start.row).count());
            // With the required number of steps, we can reach the very edge of an input tile
            // at the furthest point.
            let steps_required: usize = 26_501_365;
            assert_eq!(0, (steps_required - steps_to_edge) % self.rows);
            let diamond_radius = (steps_required - steps_to_edge) / self.rows;
            // Diamond radius is even
            assert_eq!(0, diamond_radius % 2);

            let even_parity_all_spaces = shortest_paths.values()
                .filter(|steps| **steps % 2 == 0)
                .count();
            let odd_parity_all_spaces = shortest_paths.values()
                .filter(|steps| **steps % 2 == 1)
                .count();
            let even_parity_corner_spaces = shortest_paths.values()
                .filter(|steps| **steps % 2 == 0 && **steps > steps_to_edge)
                .count();
            let odd_parity_corner_spaces = shortest_paths.values()
                .filter(|steps| **steps % 2 == 1 && **steps > steps_to_edge)
                .count();

            Answer::Usize(
                diamond_radius.pow(2) * even_parity_all_spaces
                + (diamond_radius + 1).pow(2) * odd_parity_all_spaces
                + diamond_radius * even_parity_corner_spaces
                - (diamond_radius + 1) * odd_parity_corner_spaces
            )
        }
    }

    impl Soln {
        fn next_paths(&self, path: &Path) -> VecDeque<Path> {
            let mut result = VecDeque::new();
            if path.point.row != 0 {
                let point = Point { 
                    row: path.point.row - 1,
                    col: path.point.col,
                };
                if !self.rocks.contains(&point) {
                    result.push_back(Path { point, steps: path.steps + 1 });
                }
            }
            if path.point.col != 0 {
                let point = Point { 
                    row: path.point.row,
                    col: path.point.col - 1, 
                };
                if !self.rocks.contains(&point) {
                    result.push_back(Path { point, steps: path.steps + 1 });
                }
            }
            if path.point.row != self.rows - 1 {
                let point = Point { 
                    row: path.point.row + 1,
                    col: path.point.col, 
                };
                if !self.rocks.contains(&point) {
                    result.push_back(Path { point, steps: path.steps + 1 });
                }
            }
            if path.point.col != self.cols - 1 {
                let point = Point { 
                    row: path.point.row,
                    col: path.point.col + 1, 
                };
                if !self.rocks.contains(&point) {
                    result.push_back(Path { point, steps: path.steps + 1 });
                }
            }
            result
        }

        fn parse_input_file(&mut self, filename: &str) {
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    self.cols = line.len();
                    self.rocks.extend(
                        line.char_indices()
                            .filter(|(_col, ch)| *ch == '#')
                            .map(|(col, _ch)| Point { row: self.rows, col })
                    );
                    if let Some(col) = line.find('S') {
                        self.start = Point { row: self.rows, col };
                    }
                    self.rows += 1;
                });
        }

    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        // TODO: update for correct answer
        #[test_case(2, Answer::Usize(0); "full_input")]
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
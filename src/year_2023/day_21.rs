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
#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 21 };

pub mod part_one {

    use std::collections::{HashSet, VecDeque};

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
    struct Point {
        row: usize,
        col: usize,
    }

    pub struct Soln {
        steps: u32,
        start: Point,
        rocks: HashSet<Point>,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self::with_steps(64)
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            let mut explored: HashSet<Point> = HashSet::new();
            let mut queue: VecDeque<Point> = VecDeque::from([self.start]);
            // Assumes an even number of steps
            for _ in 0..(self.steps / 2) {
                let mut new_queue = VecDeque::new();
                while !queue.is_empty() {
                    let pt = queue.pop_front().unwrap();
                    if !explored.contains(&pt) {
                        new_queue.extend(self.two_steps_away(&pt));
                        explored.insert(pt);
                    }
                }
                queue = new_queue;
            }
            for pt in queue.drain(0..) {
                explored.insert(pt);
            }
            Answer::Usize(explored.len())
        }
    }

    impl Soln {
        fn with_steps(steps: u32) -> Self {
            Self {
                steps,
                start: Point { row: 0, col: 0 },
                rocks: HashSet::new(),
            }
        }

        fn parse_input_file(&mut self, filename: &str) {
            let mut row: usize = 0;
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    self.rocks.extend(
                        line.char_indices()
                            .filter(|(_col, ch)| *ch == '#')
                            .map(|(col, _ch)| Point { row, col })
                    );
                    if let Some(col) = line.find('S') {
                        self.start = Point { row, col };
                    }
                    row += 1;
                });
        }

        fn two_steps_away(&self, pt: &Point) -> VecDeque<Point> {
            // 8 directions to check.
            // TODO: DRY
            let mut result = VecDeque::new();
            if !self.rocks.contains(&Point { row: pt.row, col: pt.col - 2 })
                && !self.rocks.contains(&Point { row: pt.row, col: pt.col - 1 }) {
                    result.push_back(Point { row: pt.row, col: pt.col - 2 });
            }
            if !self.rocks.contains(&Point { row: pt.row, col: pt.col + 2 })
                && !self.rocks.contains(&Point { row: pt.row, col: pt.col + 1 }) {
                    result.push_back(Point { row: pt.row, col: pt.col + 2 });
            }
            if !self.rocks.contains(&Point { row: pt.row - 2, col: pt.col })
                && !self.rocks.contains(&Point { row: pt.row - 1, col: pt.col }) {
                    result.push_back(Point { row: pt.row - 2, col: pt.col });
            }
            if !self.rocks.contains(&Point { row: pt.row + 2, col: pt.col })
                && !self.rocks.contains(&Point { row: pt.row + 1, col: pt.col }) {
                    result.push_back(Point { row: pt.row + 2, col: pt.col });
            }
            if !self.rocks.contains(&Point { row: pt.row + 1, col: pt.col + 1 })
                && (!self.rocks.contains(&Point { row: pt.row + 1, col: pt.col })
                    || !self.rocks.contains(&Point { row: pt.row, col: pt.col + 1 })) {
                        result.push_back(Point { row: pt.row + 1, col: pt.col + 1});
            }
            if !self.rocks.contains(&Point { row: pt.row + 1, col: pt.col - 1 })
                && (!self.rocks.contains(&Point { row: pt.row + 1, col: pt.col })
                    || !self.rocks.contains(&Point { row: pt.row, col: pt.col - 1 })) {
                        result.push_back(Point { row: pt.row + 1, col: pt.col - 1});
            }
            if !self.rocks.contains(&Point { row: pt.row - 1, col: pt.col - 1 })
                && (!self.rocks.contains(&Point { row: pt.row - 1, col: pt.col })
                    || !self.rocks.contains(&Point { row: pt.row, col: pt.col - 1 })) {
                        result.push_back(Point { row: pt.row - 1, col: pt.col - 1});
            }
            if !self.rocks.contains(&Point { row: pt.row - 1, col: pt.col + 1 })
                && (!self.rocks.contains(&Point { row: pt.row - 1, col: pt.col })
                    || !self.rocks.contains(&Point { row: pt.row, col: pt.col + 1 })) {
                        result.push_back(Point { row: pt.row - 1, col: pt.col + 1});
            }
            if !result.is_empty() {
                result.push_back(pt.clone());
            }
            result
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(16); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::with_steps(6),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}
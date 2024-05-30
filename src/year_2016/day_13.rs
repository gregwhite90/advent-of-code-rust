#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 13 };

mod utils {
    use std::collections::{BinaryHeap, HashMap, HashSet};

    use crate::utils::io_utils;

    #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    pub struct Point {
        x: u64,
        y: u64,
    }

    impl Point {
        pub fn new(x: u64, y: u64) -> Self {
            Self {
                x,
                y,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum MazeMaterial {
        Wall,
        Open,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct State {
        steps: u64,
        point: Point,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.steps.cmp(&self.steps).then_with(|| self.point.cmp(&other.point))
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    #[derive(Debug)]
    pub struct MazeSearcher {
        num: u64,
        destination: Point,
        maze: HashMap<Point, MazeMaterial>,
    }

    impl Default for MazeSearcher {
        fn default() -> Self {
            Self::with_destination(Point { x: 31, y: 39 })
        }
    }

    impl MazeSearcher {
        pub fn with_destination(destination: Point) -> Self {
            Self {
                num: 0,
                destination,
                maze: HashMap::new(),
            }
        }

        pub fn parse_input_file(&mut self, filename: &str) {
            self.num = io_utils::file_to_string(filename).parse().unwrap();
        }

        pub fn min_steps_to_destination(&mut self) -> u64 {
            let mut visited: HashSet<Point> = HashSet::new();
            let mut pq = BinaryHeap::from([
                State { steps: 0, point: Point { x: 1, y: 1 }}
            ]);
            while !pq.is_empty() {
                let state = pq.pop().unwrap();
                if state.point == self.destination { return state.steps; }
                if visited.contains(&state.point) { continue; }
                visited.insert(state.point);
                for next_state in self.next_states(state) {
                    if !visited.contains(&next_state.point) { pq.push(next_state); }                   
                }
            }   
            panic!("explored all states without reaching destination.")
        }

        pub fn visited_in_steps(&mut self, steps: u64) -> usize {
            let mut visited: HashSet<Point> = HashSet::new();
            let mut pq = BinaryHeap::from([
                State { steps: 0, point: Point { x: 1, y: 1 }}
            ]);
            while !pq.is_empty() {
                let state = pq.pop().unwrap();
                if state.steps > steps { break; }
                if visited.contains(&state.point) { continue; }
                visited.insert(state.point);
                for next_state in self.next_states(state) {
                    if !visited.contains(&next_state.point) { pq.push(next_state); }                   
                }
            }   
            visited.len()
        }

        fn next_states(&mut self, state: State) -> Vec<State> {
            let mut next_states = Vec::new();
            let mut next_points = Vec::new();
            if state.point.x != 0 {
                next_points.push(Point { x: state.point.x - 1, y: state.point.y });
            }
            if state.point.y != 0 {
                next_points.push(Point { x: state.point.x, y: state.point.y - 1 });
            }
            next_points.append(&mut vec![
                Point { x: state.point.x, y: state.point.y + 1 },
                Point { x: state.point.x + 1, y: state.point.y },
            ]);
            for next_point in next_points {
                if self.maze_material(&next_point) == MazeMaterial::Open {
                    next_states.push(State { steps: state.steps + 1, point: next_point });
                }
            }
            next_states
        }

        fn maze_material(&mut self, point: &Point) -> MazeMaterial {
            *self.maze.entry(*point).or_insert_with(|| {
                let sum = (point.x + point.y) * (point.x + point.y) + 3 * point.x + point.y + self.num;
                let ones = sum.count_ones();
                match ones % 2 {
                    0 => MazeMaterial::Open,
                    1 => MazeMaterial::Wall,
                    _ => unreachable!(),
                }
            })
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::{MazeSearcher, Point};

    #[derive(Debug)]
    pub struct Soln {
        maze_searcher: MazeSearcher,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self::with_destination(Point::new(31, 39))
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer{
            self.maze_searcher.parse_input_file(filename);
            Answer::U64(self.maze_searcher.min_steps_to_destination())
        }
    }

    impl Soln {
        fn with_destination(destination: Point) -> Self {
            Self {
                maze_searcher: MazeSearcher::with_destination(destination),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U64(11); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::with_destination(Point::new(7, 4)),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}


pub mod part_two {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::MazeSearcher;

    #[derive(Debug, Default)]
    pub struct Soln {
        maze_searcher: MazeSearcher,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer{
            self.maze_searcher.parse_input_file(filename);
            Answer::Usize(self.maze_searcher.visited_in_steps(50))
        }
    }
}
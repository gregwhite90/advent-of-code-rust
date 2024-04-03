#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 23 };

pub mod part_one {

    use std::collections::{HashMap, HashSet, VecDeque};

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
    struct Point {
        row: usize,
        col: usize,
    }

    impl Point {
        fn next_position(&self, direction: Direction) -> Point {
            match direction {
                Direction::North => Point { row: self.row - 1, col: self.col },
                Direction::East => Point { row: self.row, col: self.col + 1},
                Direction::South => Point { row: self.row + 1, col: self.col },
                Direction::West => Point { row: self.row, col: self.col - 1},
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Direction {
        North,
        East,
        South,
        West,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Tile {
        Forest,
        Path,
        Slope(Direction),
    }

    impl Tile {
        fn from_char(input: char) -> Self {
            match input {
                '#' => Self::Forest,
                '.' => Self::Path,
                '^' => Self::Slope(Direction::North),
                '>' => Self::Slope(Direction::East),
                '<' => Self::Slope(Direction::West),
                'v' => Self::Slope(Direction::South),
                _ => panic!("Unrecognized tile."),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Path {
        position: Point,
        visited: HashSet<Point>,
    }

    #[derive(Default)]
    pub struct Soln {
        rows: usize,
        cols: usize,
        start: Point,
        end: Point,
        map: HashMap<Point, Tile>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::Usize(self.longest_path())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    self.cols = line.len();
                    line.char_indices().for_each(|(idx, ch)| {
                        self.map.insert(Point { row: self.rows, col: idx }, Tile::from_char(ch));
                        if self.rows == 0 && ch == '.' { self.start = Point { row: self.rows, col: idx }}
                    });
                    self.rows += 1;
                });
            let end_points: Vec<Point> = self.map.iter()
                .filter(|(point, tile)| point.row == self.rows - 1 && **tile == Tile::Path)
                .map(|(point, _tile)| *point)
                .collect();
            assert_eq!(1, end_points.len());
            self.end = *end_points.get(0).unwrap();
        }

        fn longest_path(&mut self) -> usize {
            let mut paths: VecDeque<Path> = VecDeque::from([Path {
                position: self.start,
                visited: HashSet::from([self.start]),
            }]);
            let mut completed_path_lengths: HashSet<usize> = HashSet::new();
            while !paths.is_empty() {
                let path = paths.pop_front().unwrap();
                if path.position == self.end { 
                    completed_path_lengths.insert(path.visited.len() - 1); // -1 for the start point which should not be counted in the path len 
                    continue;
                }
                paths.append(&mut self.valid_paths(path));
            }
            completed_path_lengths.into_iter().max().expect("Should be at least one completed path.")
        }

        fn valid_paths(&self, path: Path) -> VecDeque<Path> {
            let mut valid_paths = VecDeque::new();
            if path.position.row != 0 {
                // check north
                let next_position = path.position.next_position(Direction::North);
                match self.map.get(&next_position).unwrap() {
                    Tile::Forest => {},
                    Tile::Path => {
                        if !path.visited.contains(&next_position) {
                            let mut new_visited = path.visited.clone();
                            new_visited.insert(next_position);
                            valid_paths.push_back(Path { position: next_position, visited: new_visited});
                        }
                    },
                    Tile::Slope(Direction::North) => {
                        // TODO: this should really be a loop. and a function.
                        if !path.visited.contains(&next_position) {
                            let second_position = next_position.next_position(Direction::North);
                            if !path.visited.contains(&second_position) {
                                let mut new_visited = path.visited.clone();
                                new_visited.insert(next_position);
                                new_visited.insert(second_position);
                                valid_paths.push_back(Path { position: second_position, visited: new_visited});
                            }
                        }
                    },
                    Tile::Slope(Direction::East) => {
                        if !path.visited.contains(&next_position) {
                            let second_position = next_position.next_position(Direction::East);
                            if !path.visited.contains(&second_position) {
                                let mut new_visited = path.visited.clone();
                                new_visited.insert(next_position);
                                new_visited.insert(second_position);
                                valid_paths.push_back(Path { position: second_position, visited: new_visited});
                            }
                        }
                    },
                    Tile::Slope(Direction::South) => {},
                    Tile::Slope(Direction::West) => {
                        if !path.visited.contains(&next_position) {
                            let second_position = next_position.next_position(Direction::West);
                            if !path.visited.contains(&second_position) {
                                let mut new_visited = path.visited.clone();
                                new_visited.insert(next_position);
                                new_visited.insert(second_position);
                                valid_paths.push_back(Path { position: second_position, visited: new_visited});
                            }
                        }
                    },
                }
            } if path.position.col != 0 {
                // check west
                let next_position = path.position.next_position(Direction::West);
                match self.map.get(&next_position).unwrap() {
                    Tile::Forest => {},
                    Tile::Path => {
                        if !path.visited.contains(&next_position) {
                            let mut new_visited = path.visited.clone();
                            new_visited.insert(next_position);
                            valid_paths.push_back(Path { position: next_position, visited: new_visited});
                        }
                    },
                    Tile::Slope(Direction::North) => {
                        // TODO: this should really be a loop. and a function.
                        if !path.visited.contains(&next_position) {
                            let second_position = next_position.next_position(Direction::North);
                            if !path.visited.contains(&second_position) {
                                let mut new_visited = path.visited.clone();
                                new_visited.insert(next_position);
                                new_visited.insert(second_position);
                                valid_paths.push_back(Path { position: second_position, visited: new_visited});
                            }
                        }
                    },
                    Tile::Slope(Direction::East) => {},
                    Tile::Slope(Direction::South) => {
                        if !path.visited.contains(&next_position) {
                            let second_position = next_position.next_position(Direction::South);
                            if !path.visited.contains(&second_position) {
                                let mut new_visited = path.visited.clone();
                                new_visited.insert(next_position);
                                new_visited.insert(second_position);
                                valid_paths.push_back(Path { position: second_position, visited: new_visited});
                            }
                        }
                    },
                    Tile::Slope(Direction::West) => {
                        if !path.visited.contains(&next_position) {
                            let second_position = next_position.next_position(Direction::West);
                            if !path.visited.contains(&second_position) {
                                let mut new_visited = path.visited.clone();
                                new_visited.insert(next_position);
                                new_visited.insert(second_position);
                                valid_paths.push_back(Path { position: second_position, visited: new_visited});
                            }
                        }
                    },
                }
            }
            if path.position.row != self.rows - 1 {
                // check south
                let next_position = path.position.next_position(Direction::South);
                match self.map.get(&next_position).unwrap() {
                    Tile::Forest => {},
                    Tile::Path => {
                        if !path.visited.contains(&next_position) {
                            let mut new_visited = path.visited.clone();
                            new_visited.insert(next_position);
                            valid_paths.push_back(Path { position: next_position, visited: new_visited});
                        }
                    },
                    Tile::Slope(Direction::North) => {
                        // TODO: this should really be a loop. and a function.
                    },
                    Tile::Slope(Direction::East) => {
                        if !path.visited.contains(&next_position) {
                            let second_position = next_position.next_position(Direction::East);
                            if !path.visited.contains(&second_position) {
                                let mut new_visited = path.visited.clone();
                                new_visited.insert(next_position);
                                new_visited.insert(second_position);
                                valid_paths.push_back(Path { position: second_position, visited: new_visited});
                            }
                        }
                    },
                    Tile::Slope(Direction::South) => {
                        if !path.visited.contains(&next_position) {
                            let second_position = next_position.next_position(Direction::South);
                            if !path.visited.contains(&second_position) {
                                let mut new_visited = path.visited.clone();
                                new_visited.insert(next_position);
                                new_visited.insert(second_position);
                                valid_paths.push_back(Path { position: second_position, visited: new_visited});
                            }
                        }
                    },
                    Tile::Slope(Direction::West) => {
                        if !path.visited.contains(&next_position) {
                            let second_position = next_position.next_position(Direction::West);
                            if !path.visited.contains(&second_position) {
                                let mut new_visited = path.visited.clone();
                                new_visited.insert(next_position);
                                new_visited.insert(second_position);
                                valid_paths.push_back(Path { position: second_position, visited: new_visited});
                            }
                        }
                    },
                }
            }
            if path.position.row != self.cols - 1 {
                // check east
                let next_position = path.position.next_position(Direction::East);
                match self.map.get(&next_position).unwrap() {
                    Tile::Forest => {},
                    Tile::Path => {
                        if !path.visited.contains(&next_position) {
                            let mut new_visited = path.visited.clone();
                            new_visited.insert(next_position);
                            valid_paths.push_back(Path { position: next_position, visited: new_visited});
                        }
                    },
                    Tile::Slope(Direction::North) => {
                        // TODO: this should really be a loop. and a function.
                        if !path.visited.contains(&next_position) {
                            let second_position = next_position.next_position(Direction::North);
                            if !path.visited.contains(&second_position) {
                                let mut new_visited = path.visited.clone();
                                new_visited.insert(next_position);
                                new_visited.insert(second_position);
                                valid_paths.push_back(Path { position: second_position, visited: new_visited});
                            }
                        }
                    },
                    Tile::Slope(Direction::East) => {
                        if !path.visited.contains(&next_position) {
                            let second_position = next_position.next_position(Direction::East);
                            if !path.visited.contains(&second_position) {
                                let mut new_visited = path.visited.clone();
                                new_visited.insert(next_position);
                                new_visited.insert(second_position);
                                valid_paths.push_back(Path { position: second_position, visited: new_visited});
                            }
                        }
                    },
                    Tile::Slope(Direction::South) => {
                        if !path.visited.contains(&next_position) {
                            let second_position = next_position.next_position(Direction::South);
                            if !path.visited.contains(&second_position) {
                                let mut new_visited = path.visited.clone();
                                new_visited.insert(next_position);
                                new_visited.insert(second_position);
                                valid_paths.push_back(Path { position: second_position, visited: new_visited});
                            }
                        }
                    },
                    Tile::Slope(Direction::West) => {
                    },
                }
            }
            valid_paths
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(94); "example_1")]
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

mod utils {
    use std::{cmp, collections::{HashMap, HashSet, VecDeque}};

    use crate::utils::io_utils;

    #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    pub struct Point {
        row: usize,
        col: usize,
    }

    impl Point {
        fn next_position(&self, direction: Direction) -> Point {
            match direction {
                Direction::North => Point { row: self.row - 1, col: self.col },
                Direction::East => Point { row: self.row, col: self.col + 1},
                Direction::South => Point { row: self.row + 1, col: self.col },
                Direction::West => Point { row: self.row, col: self.col - 1},
            }
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
        fn opposite(&self) -> Direction {
            match self {
                Direction::North => Direction::South,
                Direction::East => Direction::West,
                Direction::South => Direction::North,
                Direction::West => Direction::East,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    struct Status {
        position: Point,
        direction: Direction,
    }

    impl Status {
        fn opposite_direction(&self) -> Status {
            Status {
                position: self.position,
                direction: self.direction.opposite(),
            }
        }
    }

    // tODO: to consolidate, could use part_one's version of this
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Tile {
        Forest,
        Path,
        Slope,
    }

    impl Tile {
        fn from_char(input: char) -> Self {
            match input {
                '#' => Self::Forest,
                '.' => Self::Path,
                '^' | '>' | 'v' | '<' => Self::Slope,
                _ => panic!("Unrecognized tile."),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    pub struct EdgeOut {
        dest: Point,
        length: u32,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Path {
        position: Point,
        visited: HashSet<Point>,
        length: u32,
    }

    impl Path {
        pub fn new(start: Point) -> Self {
            Self {
                position: start,
                visited: HashSet::from([start]),
                length: 0,
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct PathBuilder {
        rows: usize,
        cols: usize,
        start: Point,
        end: Point,
        map: HashMap<Point, Tile>,
        graph: HashMap<Point, HashSet<EdgeOut>>, // maps intersection point to edges out from it
    }

    impl PathBuilder {
        pub fn parse_input_file(&mut self, filename: &str) {
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    self.cols = line.len();
                    line.char_indices().for_each(|(idx, ch)| {
                        self.map.insert(Point { row: self.rows, col: idx }, Tile::from_char(ch));
                        if self.rows == 0 && ch == '.' { self.start = Point { row: self.rows, col: idx }}
                    });
                    self.rows += 1;
                });
            let end_points: Vec<Point> = self.map.iter()
                .filter(|(point, tile)| point.row == self.rows - 1 && **tile == Tile::Path)
                .map(|(point, _tile)| *point)
                .collect();
            assert_eq!(1, end_points.len());
            self.end = *end_points.get(0).unwrap();
            self.create_graph();
        }

        fn create_graph(&mut self) {
            let mut unexplored = VecDeque::new();
            let mut explored: HashSet<Status> = HashSet::new();
            unexplored.push_back(Status { position: self.start, direction: Direction::South });
            while !unexplored.is_empty() {
                let status = unexplored.pop_front().unwrap();
                if explored.contains(&status) { continue; }
                let position = status.position;
                explored.insert(status);
                let (edge_out, status) = self.find_edge_out(status);
                self.graph.entry(position)
                    .and_modify(|edges_out| { edges_out.insert(edge_out); })
                    .or_insert(HashSet::from([edge_out]));
                let backwards_edge_out = EdgeOut { dest: position, length: edge_out.length };
                self.graph.entry(edge_out.dest)
                    .and_modify(|edges_out| { edges_out.insert(backwards_edge_out); })
                    .or_insert(HashSet::from([backwards_edge_out]));
                explored.insert(status.opposite_direction());
                if status.position != self.end && !explored.contains(&status) {
                    unexplored.append(&mut self.valid_statuses(status).iter().map(|s| Status { position: status.position, direction: s.direction}).collect());
                }
            }
        }

        // Returns both the edge and the ending status (so it is known how the intersection/end was reached.)
        fn find_edge_out(&self, mut status: Status) -> (EdgeOut, Status) {
            let mut length = 0;
            status = Status { position: status.position.next_position(status.direction), direction: status.direction };
            loop {
                // TODO: This may be incorrect if the final segment is only length 1.
                let mut valid_statuses = self.valid_statuses(status);
                length += 1;
                if status.position == self.end || valid_statuses.len() > 1 { break; }
                status = valid_statuses.pop_front().unwrap();
            }
            (
                EdgeOut {
                    dest: status.position,
                    length,
                },
                status
            )
        }

        fn valid_statuses(&self, status: Status) -> VecDeque<Status> {
            let mut valid_statuses = VecDeque::new();
            for dir in [Direction::North, Direction::East, Direction::South, Direction::West] {
                if let Some(new_status) = self.check_direction(status, dir) {
                    valid_statuses.push_back(new_status);
                }    
            }
            valid_statuses
        }

        fn check_direction(&self, status: Status, direction: Direction) -> Option<Status> {
            match direction {
                Direction::North => if status.position.row == 0 { return None; },
                Direction::South => if status.position.row == self.rows - 1 { return None; },
                Direction::East => if status.position.col == self.cols - 1 { return None; },
                Direction::West => if status.position.col == 0 { return None; },
            }
            if status.direction != direction.opposite() {
                let next_position = status.position.next_position(direction);
                match self.map.get(&next_position).unwrap() {
                    Tile::Forest => None,
                    Tile::Path | Tile::Slope => {
                        Some(Status { position: next_position, direction })
                    },
                }
            } else {
                None
            }
        }

        pub fn start(&self) -> Point {
            self.start
        }

        pub fn end(&self) -> Point {
            self.end
        }

        // Brute force approach, traverses every possible path through the graph. Works, but slowly.
        pub fn max_path(&self) -> u32 {
            let mut max_path = 0;
            let mut queue: VecDeque<Path> = VecDeque::from([Path::new(self.start())]);
            while !queue.is_empty() {
                let path = queue.pop_front().unwrap();
                if path.position == self.end() {
                    max_path = cmp::max(max_path, path.length);
                } else {
                    for edge_out in self.graph.get(&path.position).expect("Intersection point should be in graph.") {
                        if path.visited.contains(&edge_out.dest) { continue; }
                        let mut visited = path.visited.clone();
                        visited.insert(edge_out.dest);
                        queue.push_back(Path {
                            position: edge_out.dest, 
                            visited, 
                            length: path.length + edge_out.length,
                        });
                    }
                }
            }
            max_path
        }
    }
}


/// This solution relies on noticing that the inputs (example and full problem input)
/// are effectively graphs: there are paths that lead to intersection points.
/// The "nodes" of the graph are the start point, the end point,
/// and all of the intersection points where there is actually a choice to make
/// when traversing the graph. Intersection points has 3-4 path or slope
/// tiles surrounding it. This uses a brute force approach to finding the longest
/// path through the graph. It is slow, but it works.
pub mod part_two {
    use crate::utils::solution::{Solution, Answer};

    use super::utils::PathBuilder;

    #[derive(Debug, Default)]
    pub struct Soln {
        path_builder: PathBuilder,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.path_builder.parse_input_file(filename);
            Answer::U32(self.path_builder.max_path())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(154); "example_1")]
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
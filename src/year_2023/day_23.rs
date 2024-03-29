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

/* TODO: need to figure out a way to be more efficient here, this should work (it works on smaller input)
but it has an infeasible runtime on the full input. Probably need to figure out a way to short circuit
and ignore certain paths. 

This probably requires a whole new approach. The input is pretty narrow, so we could split it into segments and
intersections. An intersection tile has 3 - 4 path or slope tiles surrounding it. segments connect intersections
and have lengths. Segment is start_intersection point, end_intersection point, and path length.

First segment all have to go. some segments could go in reverse order. Can't traverse the same segment twice.
Will this approach allow us to short circuit? I don't think so.
*/
pub mod part_two {

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
                if !path.visited.contains(&next_position) && *self.map.get(&next_position).unwrap() != Tile::Forest {
                    let mut new_visited = path.visited.clone();
                    new_visited.insert(next_position);
                    valid_paths.push_back(Path { position: next_position, visited: new_visited});

                }
            } if path.position.col != 0 {
                // check west
                let next_position = path.position.next_position(Direction::West);
                if !path.visited.contains(&next_position) && *self.map.get(&next_position).unwrap() != Tile::Forest {
                    let mut new_visited = path.visited.clone();
                    new_visited.insert(next_position);
                    valid_paths.push_back(Path { position: next_position, visited: new_visited});

                }
            }
            if path.position.row != self.rows - 1 {
                // check south
                let next_position = path.position.next_position(Direction::South);
                if !path.visited.contains(&next_position) && *self.map.get(&next_position).unwrap() != Tile::Forest {
                    let mut new_visited = path.visited.clone();
                    new_visited.insert(next_position);
                    valid_paths.push_back(Path { position: next_position, visited: new_visited});

                }
            }
            if path.position.row != self.cols - 1 {
                // check east
                let next_position = path.position.next_position(Direction::East);
                if !path.visited.contains(&next_position) && *self.map.get(&next_position).unwrap() != Tile::Forest {
                    let mut new_visited = path.visited.clone();
                    new_visited.insert(next_position);
                    valid_paths.push_back(Path { position: next_position, visited: new_visited});
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

        #[test_case(1, Answer::Usize(154); "example_1")]
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
#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 10 };

pub mod part_one {
    use std::collections::HashMap;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    struct Point {
        row: i32,
        col: i32,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Direction {
        N,
        S,
        E,
        W,
    }

    impl Direction {
        fn opposite(&self) -> Self {
            match *self {
                Self::N => Self::S,
                Self::S => Self::N,
                Self::E => Self::W,
                Self::W => Self::E,
            }           
        }
    }

    impl Point {
        fn step(&self, dir: &Direction) -> Self {
            match *dir {
                Direction::N => Self { row: self.row - 1, col: self.col },                
                Direction::S => Self { row: self.row + 1, col: self.col },                
                Direction::E => Self { row: self.row, col: self.col + 1 },                
                Direction::W => Self { row: self.row, col: self.col - 1 },                
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Pipe {
        NS,
        EW,
        NE,
        NW,
        SW,
        SE,
    }

    impl Pipe {
        fn from_char(ch: char) -> Self {
            match ch {
                '|' => Self::NS,
                '-' => Self::EW,
                'L' => Self::NE,
                'J' => Self::NW,
                '7' => Self::SW,
                'F' => Self::SE,
                _ => panic!("Unrecognized pipe character."),
            }
        }

        fn connects(&self, dir: &Direction) -> bool {
            match dir {
                Direction::N => *self == Self::NS || *self == Self::NE || *self == Self::NW,
                Direction::S => *self == Self::NS || *self == Self::SE || *self == Self::SW,
                Direction::E => *self == Self::EW || *self == Self::SE || *self == Self::NE,
                Direction::W => *self == Self::EW || *self == Self::SW || *self == Self::NW,
            }
        }

        fn exit_dir(&self, entry_dir: &Direction) -> Direction {
            match *self {
                Self::NS | Self::EW => entry_dir.clone(),
                Self::NW => {
                    match *entry_dir {
                        Direction::S => Direction::W,
                        Direction::E => Direction::N,
                        _ => panic!("Unrecognized entry direction."),
                    }
                },
                Self::NE => {
                    match *entry_dir {
                        Direction::S => Direction::E,
                        Direction::W => Direction::N,
                        _ => panic!("Unrecognized entry direction."),
                    }
                },
                Self::SW => {
                    match *entry_dir {
                        Direction::N => Direction::W,
                        Direction::E => Direction::S,
                        _ => panic!("Unrecognized entry direction."),
                    }
                },
                Self::SE => {
                    match *entry_dir {
                        Direction::N => Direction::E,
                        Direction::W => Direction::S,
                        _ => panic!("Unrecognized entry direction."),
                    }
                },
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Path {
        steps: u32,
        point: Point,
        dir: Direction,
    }

    impl Path {
        fn same_point_as(&self, other: &Self) -> bool {
            self.point == other.point
        }
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        start: Option<Point>,
        pipes: HashMap<Point, Pipe>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.steps_to_furthest())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let mut row: i32 = 0;
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    for (col, ch) in line.chars().enumerate() {
                        let col = col as i32;
                        match ch {
                            'S' => self.start = Some(Point { row, col }),
                            '|' | '-' | 'L' | 'J' | '7' | 'F' => {
                                self.pipes.insert(Point { row, col }, Pipe::from_char(ch));
                            },
                            '.' => continue,
                            _ => panic!("Unrecognized character.")
                        }
                    }
                    row += 1;
                });
        }

        fn starting_paths(&self) -> Vec<Path> {
            let mut paths = vec![];
            self.add_path(&mut paths, &Direction::N);
            self.add_path(&mut paths, &Direction::S);
            self.add_path(&mut paths, &Direction::E);
            self.add_path(&mut paths, &Direction::W);
            assert!(paths.len() == 2);
            paths
        }

        fn add_path(&self, paths: &mut Vec<Path>, dir: &Direction) {
            let start = self.start.unwrap();
            let (row_offset, col_offset) = match *dir {
                Direction::N => (-1,  0),
                Direction::S => ( 1,  0),
                Direction::W => ( 0, -1),
                Direction::E => ( 0,  1),
            };
            let pt = Point { row: start.row + row_offset, col: start.col + col_offset };
            if let Some(pipe) = self.pipes.get(&pt) {
                if pipe.connects(&dir.opposite()) {
                    paths.push(
                        Path {
                            steps: 1,
                            point: start.step(dir),
                            dir: *dir,
                        }
                    );
                }
            }
        }

        fn steps_to_furthest(&self) -> u32 {
            let mut paths = self.starting_paths();
            while !paths[0].same_point_as(&paths[1]) {
                paths[0] = self.step(&paths[0]);
                paths[1] = self.step(&paths[1]);
            }
            assert_eq!(paths[0].steps, paths[1].steps);
            paths[0].steps
        }

        fn step(&self, path: &Path) -> Path {
            let next_dir = self.pipes.get(&path.point)
                .unwrap()
                .exit_dir(&path.dir);
            Path {
                steps: path.steps + 1,
                point: path.point.step(&next_dir),
                dir: next_dir,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(4); "example_1")]
        #[test_case(2, Answer::U32(8); "example_2")]
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
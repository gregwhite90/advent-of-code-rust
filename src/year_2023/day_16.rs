#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 16 };

mod utils {
    use std::collections::{HashMap, HashSet, VecDeque};

    use crate::utils::io_utils;

    #[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
    pub struct Point {
        row: usize,
        col: usize,
    }

    impl Point {
        pub fn new(row: usize, col: usize) -> Self {
            Self {
                row,
                col,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    pub enum Direction {
        North,
        East,
        South,
        West,
    }

    impl Default for Direction {
        fn default() -> Self {
            Self::East
        }
    }

    #[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
    pub struct Beam {
        position: Point,
        direction: Direction,
    }

    impl Beam {
        pub fn new(position: Point, direction: Direction) -> Self {
            Self {
                position,
                direction,
            }
        }

        fn next_position(&self) -> Point {
            match self.direction {
                Direction::North => Point { row: self.position.row - 1, col: self.position.col },
                Direction::East => Point { row: self.position.row, col: self.position.col + 1 },
                Direction::South => Point { row: self.position.row + 1, col: self.position.col },
                Direction::West => Point { row: self.position.row, col: self.position.col - 1 },
            }
        }

        fn advance_position(&mut self) {
            self.position = self.next_position();
        }
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct Grid {
        rows: usize,
        cols: usize,
        layout: HashMap<Point, char>,
    }

    impl Grid {
        pub fn parse_input_file(&mut self, filename: &str) {
            io_utils::file_to_lines(filename).for_each(|line| {
                self.cols = line.len();
                line.char_indices()
                    .filter(|(_idx, ch)| *ch != '.')
                    .for_each(|(idx, ch)| {
                        self.layout.insert(Point { row: self.rows, col: idx}, ch);
                    });
                self.rows += 1;
            });
        }

        pub fn cols(&self) -> usize {
            self.cols
        }

        pub fn rows(&self) -> usize {
            self.rows
        }

        fn going_off_map(&self, beam: &Beam) -> bool {
            beam.direction == Direction::East && beam.position.col == self.cols - 1
                || beam.direction == Direction::South && beam.position.row == self.rows - 1
                || beam.direction == Direction::West && beam.position.col == 0
                || beam.direction == Direction::North && beam.position.row == 0
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct GridEnergizer<'a> {
        grid: &'a Grid,
        energized: HashMap<Point, HashSet<Direction>>,
    }

    impl<'a> GridEnergizer<'a> {
        pub fn new(grid: &'a Grid) -> GridEnergizer<'a> {
            Self {
                grid,
                energized: HashMap::new(),
            }
        }

        pub fn energize(&mut self, start_beam: Beam) {
            let mut beams: VecDeque<Beam> = VecDeque::from([start_beam]);
            while !beams.is_empty() {
                let mut beam = beams.pop_back().unwrap();
                loop {
                    // Prevent infinite loops
                    if let Some(directions) = self.energized.get(&beam.position) {
                        if directions.contains(&beam.direction) {
                            break;
                        }
                    }
                    self.energized.entry(beam.position)
                        .and_modify(|directions| { directions.insert(beam.direction); })
                        .or_insert(HashSet::from([beam.direction]));
                    match self.grid.layout.get(&beam.position) {
                        None => {},
                        Some('/') => {
                            match beam.direction {
                                Direction::North => beam.direction = Direction::East,
                                Direction::East => beam.direction = Direction::North,
                                Direction::South => beam.direction = Direction::West,
                                Direction::West => beam.direction = Direction::South,
                            }
                        },
                        Some('\\') => {
                            match beam.direction {
                                Direction::North => beam.direction = Direction::West,
                                Direction::East => beam.direction = Direction::South,
                                Direction::South => beam.direction = Direction::East,
                                Direction::West => beam.direction = Direction::North,
                            }                            
                        },
                        Some('|') => {
                            match beam.direction {
                                Direction::East | Direction::West => {
                                    beam.direction = Direction::North;
                                    let mut new_beam = Beam { position: beam.position, direction: Direction::South };
                                    if !self.grid.going_off_map(&new_beam) {
                                        new_beam.advance_position();
                                        beams.push_back(new_beam);
                                    }
                                },
                                Direction::North | Direction::South => {},
                            }                            
                        },
                        Some('-') => {
                            match beam.direction {
                                Direction::North | Direction::South => {
                                    beam.direction = Direction::East;
                                    let mut new_beam = Beam { position: beam.position, direction: Direction::West };
                                    if !self.grid.going_off_map(&new_beam) {
                                        new_beam.advance_position();
                                        beams.push_back(new_beam);
                                    }
                                },
                                Direction::East | Direction::West => {},
                            }                            
                        },
                        Some(_) => panic!("Unrecognized layout character."),
                    }
                    if self.grid.going_off_map(&beam) {
                        break;
                    }
                    beam.advance_position();
                }

            }
        }

        pub fn energized(&self) -> usize {
            self.energized.len()
        }
    }
}

pub mod part_one {

    use crate::utils::solution::{Solution, Answer};

    use super::utils::{Beam, Grid, GridEnergizer};

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct Soln {
        grid: Grid,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::Usize(self.energized())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.grid = Grid::default();
            self.grid.parse_input_file(filename);
        }

        fn energized(&self) -> usize {
            let mut ge = GridEnergizer::new(&self.grid);
            ge.energize(Beam::default());
            ge.energized()
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(46); "example_1")]
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

    use std::cmp;

    use crate::utils::solution::{Solution, Answer};

    use super::utils::{Beam, Direction, Grid, GridEnergizer, Point};

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct Soln {
        grid: Grid,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::Usize(self.max_energized())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.grid = Grid::default();
            self.grid.parse_input_file(filename);
        }

        fn energize(&self, start_beam: Beam) -> usize {
            let mut ge = GridEnergizer::new(&self.grid);
            ge.energize(start_beam);
            ge.energized()
        }

        fn max_energized(&self) -> usize {
            let mut max_energized = 0;
            for col in 0..self.grid.cols() {
                let beam = Beam::new(Point::new(0, col), Direction::South);
                max_energized = cmp::max(self.energize(beam), max_energized);
                let beam = Beam::new(Point::new(self.grid.rows() - 1, col), Direction::North);
                max_energized = cmp::max(self.energize(beam), max_energized);
            }
            for row in 0..self.grid.rows() {
                let beam = Beam::new(Point::new(row, 0), Direction::East);
                max_energized = cmp::max(self.energize(beam), max_energized);
                let beam = Beam::new(Point::new(row, self.grid.cols() - 1), Direction::West);
                max_energized = cmp::max(self.energize(beam), max_energized);
            }
            max_energized
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(51); "example_1")]
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
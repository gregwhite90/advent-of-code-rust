#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 20 };

mod utils {
    use std::{cmp::min, collections::{HashMap, HashSet, VecDeque}};

    use crate::utils::io_utils;

    #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    struct Point {
        x: isize,
        y: isize,
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    impl Direction {
        fn from_char(input: char) -> Self {
            match input {
                'N' => Self::North,
                'S' => Self::South,
                'E' => Self::East,
                'W' => Self::West,
                _ => panic!("Unrecognized character"),
            }
        }
    }

    impl Point {
        fn move_direction(&mut self, direction: &Direction) {
            match direction {
                Direction::North => self.y += 1,
                Direction::South => self.y -= 1,
                Direction::East  => self.x += 1,
                Direction::West  => self.x -= 1,
            }
        }
    }

    #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    struct ConstructionMapStatus {
        doors_crossed: usize,
        point: Point,
    }

    impl ConstructionMapStatus {
        fn move_direction(&mut self, direction: &Direction) {
            self.doors_crossed += 1;
            self.point.move_direction(direction);
        }
    }

    #[derive(Debug, Default)]
    pub struct ConstructionMap {
        distances: HashMap<Point, usize>,
    }

    impl ConstructionMap {
        pub fn parse_input_file(&mut self, filename: &str) {
            let start = Point::default();
            let mut current: Vec<ConstructionMapStatus> = Vec::new();
            let mut branch_origins: VecDeque<HashSet<ConstructionMapStatus>> = VecDeque::new();
            let mut branch_ends: VecDeque<HashSet<ConstructionMapStatus>> = VecDeque::new();
            io_utils::file_to_string(filename)
                .chars()
                .for_each(|ch| {
                    match ch {
                        '^' => current.push(ConstructionMapStatus { point: start, doors_crossed: 0 }),
                        '$' => current.iter().for_each(|status| {
                            self.distances
                                .entry(status.point)
                                .and_modify(|dist| *dist = min(*dist, status.doors_crossed))
                                .or_insert(status.doors_crossed);
                        }),
                        'N' | 'S' | 'E' | 'W' => {
                            // Move all the current paths along this direction.
                            for status in current.iter_mut() {
                                self.distances
                                    .entry(status.point)
                                    .and_modify(|dist| *dist = min(*dist, status.doors_crossed))
                                    .or_insert(status.doors_crossed);
                                status.move_direction(&Direction::from_char(ch));
                            }
                        }, 
                        '(' => {
                            // Save all current paths as starting points for this branch
                            branch_origins.push_back(HashSet::from_iter(current.clone().into_iter()));
                            branch_ends.push_back(HashSet::new());
                        },
                        '|' => {
                            // Add all current paths to the branch ends.
                            branch_ends.get_mut(branch_ends.len() - 1)
                                .unwrap()
                                .extend(current.clone().into_iter());
                            // Restart with all the top starting points as current.
                            current = Vec::from_iter(branch_origins.get(branch_origins.len() - 1).unwrap().clone().into_iter());
                        },
                        ')' => {
                            branch_ends.get_mut(branch_ends.len() - 1)
                                .unwrap()
                                .extend(current.clone().into_iter());
                            // Pop off the top starting points and continue processing
                            current = Vec::from_iter(branch_ends.pop_back().unwrap().into_iter());
                            branch_origins.pop_back();
                        },
                        _ => panic!("Unrecognized character."),
                    }
                });
        }

        pub fn largest_number_of_doors(&self) -> usize {
            *self.distances.values().max().unwrap()
        }

        pub fn rooms_at_least_n_doors_away(&self, n: usize) -> usize {
            self.distances.values().filter(|doors| **doors >= n).count()
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::ConstructionMap;

    #[derive(Debug, Default)]
    pub struct Soln {
        construction_map: ConstructionMap,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.construction_map.parse_input_file(filename);
            Answer::Usize(self.construction_map.largest_number_of_doors())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(3); "example_1")]
        #[test_case(2, Answer::Usize(10); "example_2")]
        #[test_case(3, Answer::Usize(18); "example_3")]
        #[test_case(4, Answer::Usize(23); "example_4")]
        #[test_case(5, Answer::Usize(31); "example_5")]
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

    use super::utils::ConstructionMap;

    #[derive(Debug, Default)]
    pub struct Soln {
        construction_map: ConstructionMap,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.construction_map.parse_input_file(filename);
            Answer::Usize(self.construction_map.rooms_at_least_n_doors_away(1_000))
        }
    }
}
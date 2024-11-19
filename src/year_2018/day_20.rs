#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 20 };

pub mod part_one {
    use std::{cmp::min, collections::{HashMap, HashSet, VecDeque}};

    /** TODO: can i build it up from the start?
     * keep a priority queue of the doors tracked, location.
     * but also need to know the depth in the regex maybe
    */
    use crate::utils::{io_utils, solution::{Answer, Solution}};

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
    pub struct Soln {}

    // TODO: Track shortest path to each position.
    // Track current positions

    impl Soln {
        fn largest_number_of_doors(&mut self, filename: &str) -> usize {
            let start = Point::default();
            let mut current: Vec<ConstructionMapStatus> = Vec::new();
            // TODO: what data structure here?
            let mut branch_origins: VecDeque<HashSet<ConstructionMapStatus>> = VecDeque::new();
            // TODO: what data structure here?
            let mut branch_ends: VecDeque<HashSet<ConstructionMapStatus>> = VecDeque::new();
            let mut distances: HashMap<Point, usize> = HashMap::new();
            io_utils::file_to_string(filename)
                .chars()
                .for_each(|ch| {
                    match ch {
                        '^' => current.push(ConstructionMapStatus { point: start, doors_crossed: 0 }),
                        '$' => current.iter().for_each(|status| {
                            distances
                                .entry(status.point)
                                .and_modify(|dist| *dist = min(*dist, status.doors_crossed))
                                .or_insert(status.doors_crossed);
                        }),
                        'N' | 'S' | 'E' | 'W' => {
                            // Move all the current paths along this direction.
                            for status in current.iter_mut() {
                                distances
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
            *distances.values().max().unwrap()
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            Answer::Usize(self.largest_number_of_doors(filename))
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
#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 22 };

mod utils {
    use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}};
    
    use itertools::iproduct;
    use regex::Regex;

    use crate::utils::io_utils;

    #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    pub struct Coordinates {
        x: usize,
        y: usize,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct Region {
        erosion_level: usize,
    }

    impl Region {
        pub fn erosion_level(&self) -> usize {
            self.erosion_level
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    enum RegionType {
        ROCKY,
        WET,
        NARROW,
    }

    impl RegionType {
        pub fn from_erosion_level(erosion_level: usize) -> Self {
            match erosion_level % 3 {
                0 => Self::ROCKY,
                1 => Self::WET,
                2 => Self::NARROW,
                _ => unreachable!(),
            }
        }


        pub fn valid_gear(&self) -> HashSet<Gear> {
            match self {
                Self::ROCKY => HashSet::from([Gear::TORCH, Gear::CLIMBING]),
                Self::WET => HashSet::from([Gear::CLIMBING, Gear::NEITHER]),
                Self::NARROW => HashSet::from([Gear::TORCH, Gear::NEITHER]),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    enum Gear {
        TORCH,
        CLIMBING,
        NEITHER,
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
    struct Status {
        minutes: usize,
        visited_status: VisitedStatus,
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
    struct VisitedStatus {
        coordinates: Coordinates,
        gear: Gear,
    }

    #[derive(Debug, Default)]
    pub struct Maze {
        regions: HashMap<Coordinates, Region>,
        depth: usize,
        mouth: Coordinates,
        target: Coordinates,
    }

    impl Maze {
        pub fn parse_input_file(&mut self, filename: &str) {
            let depth_re = Regex::new(r"depth: (?<depth>\d+)").unwrap();
            let target_re = Regex::new(r"target: (?<x>\d+)\,(?<y>\d+)").unwrap();
            io_utils::file_to_lines(filename).for_each(|line| {
                if let Some(caps) = depth_re.captures(&line) {
                    self.depth = caps.name("depth").unwrap().as_str().parse().unwrap();
                } else if let Some(caps) = target_re.captures(&line) {
                    let x = caps.name("x").unwrap().as_str().parse().unwrap();
                    let y = caps.name("y").unwrap().as_str().parse().unwrap();
                    self.target = Coordinates { x, y };
                }
            });
        }

        fn region_erosion_level(&mut self, coords: Coordinates) -> usize {
            if let Some(r) = self.regions.get(&coords) {
                return r.erosion_level()
            }
            let geologic_index = if coords == self.mouth || coords == self.target {
                0
            } else if coords.y == 0 {
                coords.x * 16_807
            } else if coords.x == 0 {
                coords.y * 48_271
            } else {
                self.region_erosion_level(
                    Coordinates{ x: coords.x - 1, y: coords.y }
                ) * self.region_erosion_level(
                    Coordinates{ x: coords.x, y: coords.y - 1 }
                )
            };
            let erosion_level = (geologic_index + self.depth) % 20_183;
            self.regions.insert(coords, Region { erosion_level });
            return erosion_level;
        }

        pub fn rectangle_risk_area(&mut self) -> usize {
            iproduct!(0..=self.target.x, 0..=self.target.y)
                .map(|(x, y)| {
                    self.region_erosion_level(Coordinates { x, y }) % 3
                })
                .sum()
        }

        pub fn fewest_minutes(&mut self) -> usize {
            let mut visited: HashSet<VisitedStatus> = HashSet::new();
            let mut pq = BinaryHeap::from([
                Reverse(
                    Status {
                        minutes: 0,
                        visited_status: VisitedStatus {
                            coordinates: Coordinates::default(),
                            gear: Gear::TORCH,
                        },
                    }
                )
            ]);
            while !pq.is_empty() {
                let Reverse(status) = pq.pop().unwrap();
                if self.is_end(&status) {
                    return status.minutes
                }
                if visited.contains(&status.visited_status) {
                    continue;
                }
                visited.insert(status.visited_status.clone());
                // move to adjacent
                if status.visited_status.coordinates.x != 0 {
                    // go left
                    self.step_to_new_coords(
                        &status, 
                        Coordinates { x: status.visited_status.coordinates.x - 1, y: status.visited_status.coordinates.y }, 
                        &mut pq,
                    );
                }
                if status.visited_status.coordinates.y != 0 {
                    // go up
                    self.step_to_new_coords(
                        &status, 
                        Coordinates { x: status.visited_status.coordinates.x, y: status.visited_status.coordinates.y - 1 }, 
                        &mut pq,
                    );
                }
                // go right
                self.step_to_new_coords(
                    &status, 
                    Coordinates { x: status.visited_status.coordinates.x + 1, y: status.visited_status.coordinates.y }, 
                    &mut pq,
                );
                // go down
                self.step_to_new_coords(
                    &status, 
                    Coordinates { x: status.visited_status.coordinates.x, y: status.visited_status.coordinates.y + 1 }, 
                    &mut pq,
                );
                // switch gears
                let erosion_level = self.region_erosion_level(status.visited_status.coordinates);
                let region_type = RegionType::from_erosion_level(erosion_level);
                let valid_gears = region_type.valid_gear();
                for gear in valid_gears.iter().filter(|gear| **gear != status.visited_status.gear) {
                    pq.push(Reverse(
                        Status {
                            minutes: status.minutes + 7,
                            visited_status: VisitedStatus {
                                coordinates: status.visited_status.coordinates,
                                gear: *gear,
                            }
                        }
                    ));
                }
            }
            panic!("Empty priority queue without reaching target.");
        }

        fn step_to_new_coords(&mut self, status: &Status, coords: Coordinates, pq: &mut BinaryHeap<Reverse<Status>>) {
            let erosion_level = self.region_erosion_level(coords);
            let region_type = RegionType::from_erosion_level(erosion_level);
            let valid_gears = region_type.valid_gear();
            if valid_gears.contains(&status.visited_status.gear) {
                pq.push(Reverse(
                    Status {
                        minutes: status.minutes + 1,
                        visited_status: VisitedStatus {
                            coordinates: coords,
                            gear: status.visited_status.gear,
                        }
                    }    
                ));
            }
        }

        fn is_end(&self, status: &Status) -> bool {
            status.visited_status.coordinates == self.target
                && status.visited_status.gear == Gear::TORCH
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::Maze;

    #[derive(Debug, Default)]
    pub struct Soln {
        maze: Maze,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.maze.parse_input_file(filename);
            Answer::Usize(self.maze.rectangle_risk_area())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(114); "example_1")]
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

    use super::utils::Maze;

    #[derive(Debug, Default)]
    pub struct Soln {
        maze: Maze,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.maze.parse_input_file(filename);
            Answer::Usize(self.maze.fewest_minutes())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(45); "example_1")]
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
#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 22 };

mod utils {
    use std::collections::HashMap;
    
    use itertools::iproduct;
    use regex::Regex;

    use crate::utils::io_utils;

    #[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
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
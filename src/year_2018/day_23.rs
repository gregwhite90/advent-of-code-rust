#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 23 };

mod utils {
    use std::{cmp::{max, min, Ordering}, collections::BinaryHeap, isize};

    use itertools::iproduct;
    use regex::Regex;

    use lazy_static::lazy_static;

    use crate::utils::io_utils;

    lazy_static! {
        static ref NANOBOT_RE: Regex = Regex::new(r"pos=<(?<x>\-?\d+),(?<y>\-?\d+),(?<z>\-?\d+)>, r=(?<r>\d+)").unwrap();
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    struct Position {
        x: isize,
        y: isize,
        z: isize,
    }

    impl Position {
        pub fn manhattan_distance(&self, other: &Self) -> usize {
            self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    struct Nanobot {
        position: Position,
        radius: usize,
    }

    impl Nanobot {
        pub fn from_str(input: &str) -> Self {
            let caps = NANOBOT_RE.captures(input).unwrap();
            let x = caps.name("x").unwrap().as_str().parse().unwrap();
            let y = caps.name("y").unwrap().as_str().parse().unwrap();
            let z = caps.name("z").unwrap().as_str().parse().unwrap();
            let radius = caps.name("r").unwrap().as_str().parse().unwrap();
            Self {
                position: Position { x, y, z },
                radius,
            }
        }

        pub fn manhattan_distance(&self, other: &Self) -> usize {
            self.position.manhattan_distance(&other.position)
        }

        fn position_is_in_range(&self, position: &Position) -> bool {
            self.position.manhattan_distance(position) <= self.radius
        }

        pub fn other_is_in_range(&self, other: &Self) -> bool {
            self.position_is_in_range(&other.position)
        }

        fn min_x_boundary(&self) -> Position {
            Position {
                x: self.position.x - self.radius as isize,
                y: self.position.y,
                z: self.position.z,
            }
        }

        fn max_x_boundary(&self) -> Position {
            Position {
                x: self.position.x + self.radius as isize,
                y: self.position.y,
                z: self.position.z,
            }
        }

        fn min_y_boundary(&self) -> Position {
            Position {
                x: self.position.x,
                y: self.position.y - self.radius as isize,
                z: self.position.z,
            }
        }

        fn max_y_boundary(&self) -> Position {
            Position {
                x: self.position.x,
                y: self.position.y + self.radius as isize,
                z: self.position.z,
            }
        }

        fn max_z_boundary(&self) -> Position {
            Position {
                x: self.position.x,
                y: self.position.y,
                z: self.position.z + self.radius as isize,
            }
        }

        fn min_z_boundary(&self) -> Position {
            Position {
                x: self.position.x,
                y: self.position.y,
                z: self.position.z - self.radius as isize,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct SearchGrid {
        min_x: isize,
        max_x: isize,
        min_y: isize,
        max_y: isize,
        min_z: isize,
        max_z: isize,
    }

    impl Default for SearchGrid {
        fn default() -> Self {
            Self {
                min_x: isize::MAX,
                max_x: isize::MIN,
                min_y: isize::MAX,
                max_y: isize::MIN,
                min_z: isize::MAX,
                max_z: isize::MIN,
            }
        }
    }

    impl SearchGrid {
        fn volume(&self) -> Option<usize> {
            let x_dim = (self.max_x - self.min_x + 1) as usize;
            let y_dim = (self.max_y - self.min_y + 1) as usize;
            let z_dim = (self.max_z - self.min_z + 1) as usize;
            x_dim.checked_mul(y_dim)?.checked_mul(z_dim)
        }

        fn position_in_grid(&self, position: &Position) -> bool {
            self.min_x <= position.x && self.max_x >= position.x
                && self.min_y <= position.y && self.max_y >= position.y
                && self.min_z <= position.z && self.max_z >= position.z
        }

        fn nanobot_in_range(&self, nanobot: &Nanobot) -> bool {
            // Any of the 6 extremes of the nanobot radius is in the search grid,
            // or any of the 8 corners of the grid are in the nanobot's radius
            self.position_in_grid(&nanobot.min_x_boundary())
                || self.position_in_grid(&nanobot.max_x_boundary())
                || self.position_in_grid(&nanobot.min_y_boundary())
                || self.position_in_grid(&nanobot.max_y_boundary())
                || self.position_in_grid(&nanobot.min_z_boundary())
                || self.position_in_grid(&nanobot.max_z_boundary())
                || iproduct!([self.min_x, self.max_x], [self.min_y, self.max_y], [self.min_z, self.max_z])
                    .any(|(x, y, z)| {
                        nanobot.position_is_in_range(&Position { x, y, z })     
                    })
        }

        fn update_boundaries(&mut self, position: &Position) {
            self.min_x = min(self.min_x, position.x);
            self.max_x = max(self.max_x, position.x);
            self.min_y = min(self.min_y, position.y);
            self.max_y = max(self.max_y, position.y);
            self.min_z = min(self.min_z, position.z);
            self.max_z = max(self.max_z, position.z);
        }

        fn midpoint(&self) -> Position {
            let x = self.min_x + (self.max_x - self.min_x) / 2;
            let y = self.min_y + (self.max_y - self.min_y) / 2;
            let z = self.min_z + (self.max_z - self.min_z) / 2;
            Position { x, y, z }
        }

        fn divide(&self) -> impl Iterator<Item = Self> {
            let mid = self.midpoint();
            iproduct!(
                [(self.min_x, mid.x), (mid.x + 1, self.max_x)], 
                [(self.min_y, mid.y), (mid.y + 1, self.max_y)], 
                [(self.min_z, mid.z), (mid.z + 1, self.max_z)]
            ).map(|((min_x, max_x), (min_y, max_y), (min_z, max_z))| Self {
                    min_x,
                    max_x,
                    min_y,
                    max_y,
                    min_z,
                    max_z,
            })
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Subdivision {
        num_nanobots: usize,
        search_grid: SearchGrid,
    }

    impl Subdivision {
        fn new(search_grid: SearchGrid, nanobots: &Vec<Nanobot>) -> Self {
            Self {
                num_nanobots: nanobots.iter().filter(|nb| search_grid.nanobot_in_range(nb)).count(),
                search_grid,
            }
        }

        fn volume(&self) -> Option<usize> {
            self.search_grid.volume()
        }

        fn midpoint_distance_from_position(&self, position: &Position) -> usize {
            position.manhattan_distance(&self.search_grid.midpoint())
        }
    }

    impl Ord for Subdivision {
        fn cmp(&self, other: &Self) -> Ordering {
            self.num_nanobots.cmp(&other.num_nanobots)
                .then(other.search_grid.volume().cmp(&self.search_grid.volume()))
                .then(Ordering::Equal)
        }
    }

    impl PartialOrd for Subdivision {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    #[derive(Debug, Default)]
    pub struct Cavern {
        nanobots: Vec<Nanobot>,
        search_grid: SearchGrid,
    }

    impl Cavern {
        fn update_search_grid(&mut self, nanobot: &Nanobot) {
            self.search_grid.update_boundaries(&nanobot.position);
        }

        fn overall_search_grid(&self) -> SearchGrid {
            let max_diameter = *[
                self.search_grid.max_x - self.search_grid.min_x,
                self.search_grid.max_y - self.search_grid.min_y,
                self.search_grid.max_z - self.search_grid.min_z,
            ].iter().max().unwrap() as usize;
            let radius = (max_diameter.next_power_of_two() / 2) as isize;
            let mid = self.search_grid.midpoint();
            SearchGrid {
                min_x: mid.x - radius,
                max_x: mid.x + radius,
                min_y: mid.y - radius,
                max_y: mid.y + radius,
                min_z: mid.z - radius,
                max_z: mid.z + radius,          
            }
        }

        pub fn parse_input_file(&mut self, filename: &str) {
            self.nanobots = io_utils::file_to_lines(filename).map(|line| {
                let nb = Nanobot::from_str(&line);
                self.update_search_grid(&nb);
                nb
            }).collect();
        }

        pub fn nanobots_in_range_of_largest_signal_radius(&self) -> usize {
            let nanobot_with_largest_signal_radius = self.nanobots.iter().max_by_key(|n| n.radius).unwrap();
            self.nanobots.iter().filter(|n| nanobot_with_largest_signal_radius.other_is_in_range(n)).count()
        }

        pub fn distance_to_nearest_position_in_range_of_most_nanobots(&self) -> usize {
            let mut pq = BinaryHeap::from([
                Subdivision::new(self.overall_search_grid(), &self.nanobots),
            ]);
            while !pq.is_empty() {
                let subdivision = pq.pop().unwrap();
                if let Some(1) = subdivision.volume() {
                    return subdivision.midpoint_distance_from_position(&Position { x: 0, y: 0, z: 0 });
                }
                pq.extend(subdivision.search_grid.divide().map(|sg| Subdivision::new(sg, &self.nanobots)))
            }
            panic!("Searched all subdivisions without finding volume equal to 1.");
        }
        
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::Cavern;

    #[derive(Debug, Default)]
    pub struct Soln {
        cavern: Cavern,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.cavern.parse_input_file(filename);
            Answer::Usize(self.cavern.nanobots_in_range_of_largest_signal_radius())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(7); "example_1")]
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

/// Implements an octenary search, idea based on puzzle creator's
/// reference to an octree scan here: https://www.reddit.com/r/adventofcode/comments/aa9uvg/comment/ecrftas/
pub mod part_two {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::Cavern;

    #[derive(Debug, Default)]
    pub struct Soln {
        cavern: Cavern,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.cavern.parse_input_file(filename);
            Answer::Usize(self.cavern.distance_to_nearest_position_in_range_of_most_nanobots())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(2, Answer::Usize(36); "example_2")]
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
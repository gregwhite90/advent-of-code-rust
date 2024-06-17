#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 6 };

mod utils {
    use std::{cmp::{max, min}, collections::{HashMap, HashSet}};

    use itertools::iproduct;
    use regex::Regex;

    use crate::utils::io_utils;

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    struct Point {
        x: usize,
        y: usize,
    }

    impl Point {
        fn manhattan_distance(&self, other: &Self) -> usize {
            self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
        }
    }

    #[derive(Debug)]
    struct Range {
        min: usize,
        max: usize,
    }

    impl Default for Range {
        fn default() -> Self {
            Self {
                min: usize::MAX,
                max: usize::MIN,
            }
        }
    }

    impl Range {
        fn update(&mut self, val: usize) {
            self.min = min(self.min, val);
            self.max = max(self.max, val);
        }
    }

    #[derive(Debug, Default)]
    struct BoundingBox {
        x: Range,
        y: Range,
    }

    impl BoundingBox {
        fn update(&mut self, point: &Point) {
            self.x.update(point.x);
            self.y.update(point.y);
        }

        fn is_on_boundary(&self, point: &Point) -> bool {
            point.x == self.x.min || point.x == self.x.max || point.y == self.y.min || point.y == self.y.max
        }
    }

    #[derive(Debug, Default)]
    pub struct Grid {
        bounding_box: BoundingBox,
        regions: HashMap<Point, HashSet<Point>>,
        safe_region_max_dist: usize,
    }

    impl Grid {
        pub fn with_safe_region_max_dist(safe_region_max_dist: usize) -> Self {
            Self {
                bounding_box: BoundingBox::default(),
                regions: HashMap::default(),
                safe_region_max_dist,
            }
        }

        pub fn parse_input_file(&mut self, filename: &str) {
            let line_re = Regex::new(r"(?<x>\d+), (?<y>\d+)").unwrap();
            io_utils::file_to_lines(filename).for_each(|line| {
                let captures = line_re.captures(&line).unwrap();
                let point = Point {
                    x: captures.name("x").unwrap().as_str().parse().unwrap(),
                    y: captures.name("y").unwrap().as_str().parse().unwrap(),
                };
                self.bounding_box.update(&point);
                self.regions.insert(point, HashSet::new());
            })
        }

        pub fn calculate_regions(&mut self) {
            for (x, y) in iproduct!(
                self.bounding_box.x.min..=self.bounding_box.x.max,
                self.bounding_box.y.min..=self.bounding_box.y.max
            ) {
                let point = Point { x, y };
                let min = self.regions.keys().map(|pt| {
                    point.manhattan_distance(&pt)
                }).min().unwrap();
                let mut min_points_iter = self.regions.keys().filter(|pt| {
                    point.manhattan_distance(&pt) == min
                });
                let min_point = min_points_iter.next().unwrap();
                if min_points_iter.count() == 0 {
                    self.regions.entry(*min_point).and_modify(|region| { region.insert(point); });
                }
            }
        }

        pub fn max_finite_area(&self) -> usize {
            self.regions.values().filter(|region|{
                region.iter().all(|point| !self.bounding_box.is_on_boundary(point))
            })
                .map(|region| region.len())
                .max()
                .unwrap()
        }

        /// This assumes the safe region is contained within the bounding box, which is not necessarily true
        pub fn safe_region_area(&mut self) -> usize {
            iproduct!(
                self.bounding_box.x.min..=self.bounding_box.x.max,
                self.bounding_box.y.min..=self.bounding_box.y.max
            ).filter(|(x, y)| {
                let point = Point { x: *x, y: *y };
                self.regions.keys().map(|pt| {
                    point.manhattan_distance(&pt)
                }).sum::<usize>() < self.safe_region_max_dist
            }).count()
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};
    use super::utils::Grid;

    #[derive(Debug, Default)]
    pub struct Soln {
        grid: Grid,    
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.grid.parse_input_file(filename);
            self.grid.calculate_regions();
            Answer::Usize(self.grid.max_finite_area())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(17); "example_1")]
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
    use super::utils::Grid;

    #[derive(Debug)]
    pub struct Soln {
        grid: Grid,    
    }

    impl Default for Soln {
        fn default() -> Self {
            Self::with_safe_region_max_dist(10_000)
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.grid.parse_input_file(filename);
            Answer::Usize(self.grid.safe_region_area())
        }
    }

    impl Soln {
        fn with_safe_region_max_dist(safe_region_max_dist: usize) -> Self {
            Self {
                grid: Grid::with_safe_region_max_dist(safe_region_max_dist),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(16); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::with_safe_region_max_dist(32),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}
#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2015, day: 6 };

mod utils {
    use itertools::iproduct;
    use regex::Regex;

    use crate::utils::io_utils;

    pub const DIMENSIONS: usize = 1_000;

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    struct Point {
        x: usize,
        y: usize,
    }

    pub struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    impl Rectangle {
        pub fn iter_over_cells(&self) -> impl Iterator<Item = (usize, usize)> { 
            iproduct!(
                self.top_left.x..=self.bottom_right.x,
                self.top_left.y..=self.bottom_right.y
            )
        }
    }

    pub trait InteractiveLightGrid {
        fn turn_on(&mut self, rectangle: &Rectangle);
        fn toggle(&mut self, rectangle: &Rectangle);
        fn turn_off(&mut self, rectangle: &Rectangle);

        fn parse_input_file(&mut self, filename: &str) {
            let re = Regex::new(
                r"(?<op>(turn on)|(toggle)|(turn off)) (?<x_min>\d+)\,(?<y_min>\d+) through (?<x_max>\d+)\,(?<y_max>\d+)"
            ).unwrap();
            for line in io_utils::file_to_lines(filename) {
                let caps = re.captures(&line).unwrap();
                let op = caps.name("op").unwrap().as_str();
                let x_min = caps.name("x_min").unwrap().as_str().parse().unwrap();
                let y_min = caps.name("y_min").unwrap().as_str().parse().unwrap();
                let top_left = Point { x: x_min, y: y_min };
                let x_max = caps.name("x_max").unwrap().as_str().parse().unwrap();
                let y_max = caps.name("y_max").unwrap().as_str().parse().unwrap();
                let bottom_right = Point { x: x_max, y: y_max };
                let rectangle = Rectangle { top_left, bottom_right };
                match op {
                    "turn on" => self.turn_on(&rectangle),
                    "toggle" => self.toggle(&rectangle),
                    "turn off" => self.turn_off(&rectangle),
                    _ => panic!("Unrecognized operation."),
                }
            }
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::{InteractiveLightGrid, Rectangle, DIMENSIONS};

    #[derive(Debug)]
    struct LightGrid {
        on: Vec<Vec<bool>>,
    }

    impl LightGrid {
        fn new(dimensions: usize) -> Self {
            Self {
                on: vec![vec![false; dimensions]; dimensions]
            }
        }

        pub fn num_on(&self) -> usize {
            self.on.iter().map(|row|{
                row.iter().filter(|on| **on).count()
            }).sum()
        }
    }

    impl InteractiveLightGrid for LightGrid { 
        fn turn_on(&mut self, rectangle: &Rectangle) {
            for (x, y) in rectangle.iter_over_cells() {
                self.on[x][y] = true;
            }
        }

        fn toggle(&mut self, rectangle: &Rectangle) {
            for (x, y) in rectangle.iter_over_cells() {
                self.on[x][y] = !self.on[x][y];
            }
        }

        fn turn_off(&mut self, rectangle: &Rectangle) {
            for (x, y) in rectangle.iter_over_cells() {
                self.on[x][y] = false;
            }
        }
    }

    impl Default for LightGrid {
        fn default() -> Self {
            Self::new(DIMENSIONS)
        }
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        light_grid: LightGrid,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.light_grid.parse_input_file(filename);
            Answer::Usize(self.light_grid.num_on())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(1_000_000 - 1_000 - 4); "example_1")]
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

    use super::utils::{InteractiveLightGrid, Rectangle, DIMENSIONS};

    #[derive(Debug)]
    struct LightGrid {
        on: Vec<Vec<usize>>,
    }

    impl LightGrid {
        fn new(dimensions: usize) -> Self {
            Self {
                on: vec![vec![0; dimensions]; dimensions]
            }
        }

        pub fn num_on(&self) -> usize {
            self.on.iter().map(|row|{
                row.iter().sum::<usize>()
            }).sum()
        }
    }

    impl InteractiveLightGrid for LightGrid { 
        fn turn_on(&mut self, rectangle: &Rectangle) {
            for (x, y) in rectangle.iter_over_cells() {
                self.on[x][y] += 1;
            }
        }

        fn toggle(&mut self, rectangle: &Rectangle) {
            for (x, y) in rectangle.iter_over_cells() {
                self.on[x][y] += 2;
            }
        }

        fn turn_off(&mut self, rectangle: &Rectangle) {
            for (x, y) in rectangle.iter_over_cells() {
                self.on[x][y] = if let Some(res) = self.on[x][y].checked_sub(1) {
                    res
                } else {
                    0
                };
            }
        }
    }

    impl Default for LightGrid {
        fn default() -> Self {
            Self::new(DIMENSIONS)
        }
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        light_grid: LightGrid,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.light_grid.parse_input_file(filename);
            Answer::Usize(self.light_grid.num_on())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(1_000_000 + 2 * 1_000 - 4); "example_1")]
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
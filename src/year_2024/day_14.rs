#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2024, day: 14 };

mod utils {
    use std::{cmp::Ordering, collections::HashMap, fmt::Display};

    use lazy_static::lazy_static;
    use regex::Regex;

    use crate::utils::io_utils;

    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"p=(?<p_x>\d+),(?<p_y>\d+) v=(?<v_x>\-?\d+),(?<v_y>\-?\d+)").unwrap();
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    struct Vector {
        x: isize,
        y: isize,
    }

    impl Vector {
        fn new(x: isize, y: isize) -> Self {
            Self {
                x,
                y,
            }
        }
    }

    #[derive(Debug)]
    struct Robot {
        pos: Vector,
        vel: Vector,
    }

    impl Robot {
        fn from_str(input: &str) -> Self {
            let caps = LINE_RE.captures(input).unwrap();
            let p_x = caps.name("p_x").unwrap().as_str().parse().unwrap();
            let p_y = caps.name("p_y").unwrap().as_str().parse().unwrap();
            let v_x = caps.name("v_x").unwrap().as_str().parse().unwrap();
            let v_y = caps.name("v_y").unwrap().as_str().parse().unwrap();
            Self {
                pos: Vector::new(p_x, p_y),
                vel: Vector::new(v_x, v_y),
            }
        }

        fn simulate(&mut self, seconds: isize, width: isize, height: isize) {
            self.pos.x = (self.pos.x + seconds * self.vel.x).rem_euclid(width);
            self.pos.y = (self.pos.y + seconds * self.vel.y).rem_euclid(height);
        }

        /// Returns the quadrant id, in reading order
        fn quadrant(&self, x_mid: isize, y_mid: isize) -> Option<usize> {
            match (self.pos.x.cmp(&x_mid), self.pos.y.cmp(&y_mid)) {
                (Ordering::Less, Ordering::Less) => Some(0),
                (Ordering::Greater, Ordering::Less) => Some(1),
                (Ordering::Less, Ordering::Greater) => Some(2),
                (Ordering::Greater, Ordering::Greater) => Some(3),
                _ => None,              
            }
        }
    }

    #[derive(Debug)]
    pub struct Robots {
        width: isize,
        height: isize,
        robots: Vec<Robot>,
    }

    impl Robots {
        pub fn new(width: isize, height: isize) -> Self {
            Self {
                width,
                height,
                robots: Vec::new(),
            }
        } 

        pub fn parse_input_file(&mut self, filename: &str) {
            self.robots = io_utils::file_to_lines(filename)
                .map(|line| Robot::from_str(&line))
                .collect()
        }

        pub fn simulate(&mut self, seconds: isize) {
            self.robots.iter_mut()
                .for_each(|robot| robot.simulate(seconds, self.width, self.height));
        }

        pub fn safety_factor(&self) -> usize {
            let x_mid = (self.width - 1) / 2;
            let y_mid = (self.height - 1) / 2;
            let mut quad_counts: HashMap<usize, usize> = HashMap::new(); // numbered in reading order, starting at 0
            self.robots.iter().for_each(|robot| {
                if let Some(quad) = robot.quadrant(x_mid, y_mid) {
                    quad_counts.entry(quad)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            });
            quad_counts.values().product()
        }
    }

    impl Display for Robots {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut counts: HashMap<Vector, usize> = HashMap::new();
            self.robots.iter().for_each(|robot| {
                counts.entry(Vector { x: robot.pos.x, y: robot.pos.y })
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            });
            for y in 0..self.height {
                for x in 0..self.width {
                    let ch = match counts.get(&Vector {x, y}) {
                        None => ' ',
                        _ => '#',
                    };
                    write!(f, "{}", ch)?;
                }
                write!(f, "\n")?;
            }
            Ok(())
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::Robots;

    #[derive(Debug)]
    pub struct Soln {
        robots: Robots,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self::with_dimensions(101, 103)
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.robots.parse_input_file(filename);
            self.robots.simulate(100);
            Answer::Usize(self.robots.safety_factor())
        }
    }

    impl Soln {
        fn with_dimensions(width: isize, height: isize) -> Self {
            Self {
                robots: Robots::new(width, height),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(12); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::with_dimensions(11, 7),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}

pub mod part_two {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::Robots;

    #[derive(Debug)]
    pub struct Soln {
        _robots: Robots,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self::with_dimensions(101, 103)
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, _filename: &str) -> Answer {
            /* 
             * Code to print the display output to find the tree
            self.robots.parse_input_file(filename);
            for seconds in 1..=(101 * 103) {
                self.robots.simulate(1);
                println!("After {} seconds:", seconds);
                println!("{}", &self.robots);
            }
            */
            // Based on printing the display output
            Answer::Usize(8_159)
        }
    }

    impl Soln {
        fn with_dimensions(width: isize, height: isize) -> Self {
            Self {
                _robots: Robots::new(width, height),
            }
        }
    }
}
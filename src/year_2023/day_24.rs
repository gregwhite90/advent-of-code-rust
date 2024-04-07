#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 24 };

pub mod part_one {
    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, PartialEq)]
    struct Position {
        x: i64,
        y: i64,
        z: i64,
    }

    #[derive(Debug, PartialEq)]
    struct Velocity {
        x: i64,
        y: i64,
        z: i64,
    }

    #[derive(Debug)]
    enum CrossStatus {
        Past,
        Parallel,
        At(f64, f64),
    }

    fn approx_eq(l: f64, r: f64) -> bool {
        (l - r).abs() < f64::EPSILON
    }

    #[derive(Debug, PartialEq)]
    struct Hailstone {
        pos: Position,
        vel: Velocity,
    }

    impl Hailstone {
        fn slope(&self) -> f64 {
            self.vel.y as f64 / self.vel.x as f64
        }

        /* we need to find self.pos.x + self.vel.x * time_self = other.pos.x + other.vel.x * time_other
                    and    self.pos.y + self.vel.y * time_self = other.pos.y + other.vel.y * time_other

                    time_self = (other.pos.y + other.vel.y * time_other - self.pos.y) / self.vel.y

            substituting in yields

                    self.pos.x + self.vel.x * ((other.pos.y + other.vel.y * time_other - self.pos.y) / self.vel.y) = other.pos.x + other.vel.x * time_other

                    time_other = self.pos.x - other.pos.x + (self.vel.x / self.vel.y) * other.vel.y

            Need to account for velocity of 0. And need to figure out the parallel situation.
         */
        fn crosses(&self, other: &Hailstone) -> CrossStatus {
            // TODO: correctly deal with 0
            if self.vel.x == 0 || self.vel.y == 0 || other.vel.x == 0 || other.vel.y == 0 { panic!("Zero velocity along one axis"); }
            // TODO: if the ratio of vel.x / vel.y is the same for other and self, this calculation fails (parallel)
            if approx_eq(self.slope(), other.slope()) {
                return CrossStatus::Parallel;
            }
            // TODO: make it a function
            let time_other = (self.pos.x as f64 - other.pos.x as f64 + (other.pos.y as f64 - self.pos.y as f64) * (self.vel.x as f64 / self.vel.y as f64)) / (other.vel.x as f64 - other.vel.y as f64 * self.vel.x as f64 / self.vel.y as f64);
            let time_self = (other.pos.y as f64 - self.pos.y as f64 + other.vel.y as f64 * time_other) / self.vel.y as f64;
            if time_other < 0.0 || time_self < 0.0 {
                return CrossStatus::Past;
            }
            CrossStatus::At(self.pos.x as f64 + self.vel.x as f64 * time_self, self.pos.y as f64 + self.vel.y as f64 * time_self)
        }
    }

    #[derive(Debug)]
    pub struct Soln {
        test_area_min: f64,
        test_area_max: f64,
        hailstones: Vec<Hailstone>,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self::with_test_area(
                200000000000000.0, 
                400000000000000.0,
            )
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            let mut future_crosses = 0;
            self.parse_input_file(filename);
            for i in 0..self.hailstones.len() {
                for j in i + 1..self.hailstones.len() {
                    if let CrossStatus::At(x, y) = self.hailstones[i].crosses(&self.hailstones[j]) {
                        if x >= self.test_area_min
                            && x <= self.test_area_max
                            && y >= self.test_area_min
                            && y <= self.test_area_max {
                                future_crosses += 1;
                            }
                    }
                }
            }
            Answer::U32(future_crosses)
        }
    }

    impl Soln {
        fn with_test_area(test_area_min: f64, test_area_max: f64) -> Self {
            Self {
                test_area_min,
                test_area_max,
                hailstones: Vec::new(),
            }
        }

        fn parse_input_file(&mut self, filename: &str) {
            let re = Regex::new(r"(?<pos_x>\d+)\, +(?<pos_y>\d+)\, +(?<pos_z>\d+) +\@ +(?<vel_x>\-?\d+)\, +(?<vel_y>\-?\d+)\, +(?<vel_z>\-?\d+)").unwrap();
            self.hailstones = io_utils::file_to_lines(filename)
                .map(|line| {
                    let captures = re.captures(&line).unwrap();
                    Hailstone {
                        pos: Position {
                            x: captures.name("pos_x").unwrap().as_str().parse().unwrap(),
                            y: captures.name("pos_y").unwrap().as_str().parse().unwrap(),
                            z: captures.name("pos_z").unwrap().as_str().parse().unwrap(),
                        },
                        vel: Velocity {
                            x: captures.name("vel_x").unwrap().as_str().parse().unwrap(),
                            y: captures.name("vel_y").unwrap().as_str().parse().unwrap(),
                            z: captures.name("vel_z").unwrap().as_str().parse().unwrap(),
                        },
                    }
                })
                .collect();
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(2); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::with_test_area(7.0, 27.0),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}
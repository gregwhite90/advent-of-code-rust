#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 25 };

mod utils {
    use std::collections::{HashMap, HashSet};

    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        static ref POINT_RE: Regex = Regex::new(r"(?<x>\-?\d+),(?<y>\-?\d+),(?<z>\-?\d+),(?<t>\-?\d+)").unwrap();
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
    pub struct Point {
        x: isize,
        y: isize,
        z: isize,
        t: isize,
    }

    impl Point {
        pub fn from_str(line: &str) -> Self {
            let caps = POINT_RE.captures(line).unwrap();
            let x = caps.name("x").unwrap().as_str().parse().unwrap();
            let y = caps.name("y").unwrap().as_str().parse().unwrap();
            let z = caps.name("z").unwrap().as_str().parse().unwrap();
            let t = caps.name("t").unwrap().as_str().parse().unwrap();
            Self { x, y, z, t }
        }

        pub fn manhattan_distance(&self, other: &Self) -> usize {
            self.x.abs_diff(other.x)
                + self.y.abs_diff(other.y)
                + self.z.abs_diff(other.z)
                + self.t.abs_diff(other.t)
        }
    }

    #[derive(Debug)]
    pub struct Constellation {
        points: HashSet<Point>,
    }

    impl Constellation {
        pub fn add_point(&mut self, point: Point) {
            self.points.insert(point);
        }

        pub fn merge(&mut self, other: Constellation) {
            self.points.extend(other.points.into_iter());
        }
    }

    #[derive(Debug, Default)]
    pub struct SolarSystem {
        constellations: HashMap<usize, Constellation>,
        next_id: usize,
    }

    impl SolarSystem {
        pub fn add_point(&mut self, line: &str) {
            let point = Point::from_str(line);
            let mut constellation_ids = HashSet::new();
            for (id, constellation) in self.constellations.iter() {
                for pt in constellation.points.iter() {
                    if point.manhattan_distance(pt) <= 3 {
                        constellation_ids.insert(*id);
                        break;
                    }
                }
            }
            if constellation_ids.is_empty() {
                self.constellations.insert(
                    self.next_id,
                    Constellation { points: HashSet::from([point]) },
                );
                self.next_id += 1;
            } else if constellation_ids.len() == 1 {
                let constellation_id = constellation_ids.iter().next().unwrap();
                self.constellations.get_mut(constellation_id).unwrap().add_point(point);
            } else {
                // merge
                let mut constellation_ids_iter = constellation_ids.into_iter();
                let mergee_id = constellation_ids_iter.next().unwrap();
                let mut mergee = self.constellations.remove(&mergee_id).unwrap();
                mergee.add_point(point);
                for merger_id in constellation_ids_iter {
                    let merger = self.constellations.remove(&merger_id).unwrap();
                    mergee.merge(merger);
                }
                self.constellations.insert(mergee_id, mergee);
            }
        }

        pub fn num_constellations(&self) -> usize {
            self.constellations.len()
        }
    }
}

pub mod part_one {
    use crate::utils::{io_utils, solution::{Answer, Solution}};

    use super::utils::SolarSystem;

    #[derive(Debug, Default)]
    pub struct Soln {
        solar_system: SolarSystem,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            io_utils::file_to_lines(filename).for_each(|line| {
                self.solar_system.add_point(&line);
            });
            Answer::Usize(self.solar_system.num_constellations())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(2); "example_1")]
        #[test_case(2, Answer::Usize(4); "example_2")]
        #[test_case(3, Answer::Usize(3); "example_3")]
        #[test_case(4, Answer::Usize(8); "example_4")]
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
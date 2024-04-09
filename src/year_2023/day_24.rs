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

/// Assumes the intersections occur at integer time intervals.
pub mod part_two {
    use std::collections::{HashMap, HashSet};

    use itertools::Itertools;
    use prime_factorization::Factorization;
    use regex::Regex;
    use ndarray::prelude::*;
    use ndarray_linalg::Solve;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    const NUM_AXES: usize = 3;

    // Treating as an enum rather than named fields so we can use to index into arrays
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[repr(usize)]
    enum Axis {
        X = 0,
        Y,
        Z,
    }

    /*
    #[derive(Debug, PartialEq, Eq)]
    struct Vector {
        x: i64,
        y: i64,
        z: i64,

    }
    */

    #[derive(Debug, PartialEq, Eq)]
    struct Projectile {
        pos: [i64; NUM_AXES],
        vel: [i64; NUM_AXES],
    }

    #[derive(Debug, PartialEq, Eq)]
    struct TimeDelta {
        i: usize,
        j: usize,
        delta: i64,
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    struct Possibilities {
        possible: HashMap<i64, Vec<TimeDelta>>,
        impossible: HashSet<i64>,
    }

    impl Possibilities {
        fn determined(&self) -> bool {
            self.possible.len() == 1
        }

        fn determined_value(&self) -> Option<i64> {
            if !self.determined() {
                None
            } else {
                Some(*self.possible.iter().next().unwrap().0)
            }
        }

        fn determined_time_deltas(&self) -> impl Iterator<Item = &TimeDelta> {
            self.possible.get(&self.determined_value().unwrap()).unwrap().iter()
        }
    }

    /*
    #[derive(Debug, Default, PartialEq, Eq)]
    struct PossibilitiesVector {
        x: Possibilities,
        y: Possibilities,
        z: Possibilities,
    }
    */

    #[derive(Debug, Default, PartialEq, Eq)]
    struct PossibleProjectile {
        pos: [Possibilities; NUM_AXES],
        vel: [Possibilities; NUM_AXES],
    }

    impl PossibleProjectile {
        fn sum_of_pos_coords(&self) -> Option<i64> {
            if let (Some(x), Some(y), Some(z)) = (
                self.pos[Axis::X as usize].determined_value(), 
                self.pos[Axis::Y as usize].determined_value(), 
                self.pos[Axis::Z as usize].determined_value()
            ) {
                Some(x + y + z)
            } else {
                None
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        hailstones: Vec<Projectile>,
        rock: PossibleProjectile,
    }

    // Todo: The smaller example in the problem is actually less constrained than the full input.
    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            // Find the possible velocities
            for i in 0..self.hailstones.len() {
                for j in i + 1..self.hailstones.len() {
                    for axis_idx in [Axis::X, Axis::Y, Axis::Z].map(|a| a as usize) {
                        if self.hailstones[i].vel[axis_idx] == self.hailstones[j].vel[axis_idx] {
                            if self.hailstones[i].pos[axis_idx] != self.hailstones[j].pos[axis_idx] {
                                self.mark_vel_impossible(axis_idx, self.hailstones[i].vel[axis_idx]);
                                let max_offset = (self.hailstones[i].pos[axis_idx] - self.hailstones[j].pos[axis_idx]).abs() as u64;
                                let prime_factors = Factorization::run(max_offset);
                                let mut possible_offsets: HashSet<u64> = HashSet::new();
                                for len in 0..=prime_factors.factors.len() {
                                    for combo in prime_factors.factors.iter().combinations(len) {
                                        let prod = combo.into_iter().fold(1u64, |prod, factor| prod * *factor);
                                        possible_offsets.insert(prod);
                                    }
                                }
                                let mut possible_vels: HashMap<i64, Vec<TimeDelta>> = HashMap::new();
                                possible_offsets.into_iter()
                                    .for_each(|offset| {
                                        let offset: i64 = offset.try_into().unwrap();
                                        possible_vels.insert(
                                            self.hailstones[i].vel[axis_idx] - offset,
                                            vec![TimeDelta {
                                                i,
                                                j,
                                                delta: (self.hailstones[i].pos[axis_idx] - self.hailstones[j].pos[axis_idx]) / - offset,
                                            }],
                                        );
                                        possible_vels.insert(
                                            self.hailstones[i].vel[axis_idx] + offset,
                                            vec![TimeDelta {
                                                i,
                                                j,
                                                delta: (self.hailstones[i].pos[axis_idx] - self.hailstones[j].pos[axis_idx]) / offset,
                                            }],
                                        );
                                    }); 
                                self.mark_vels_possible(axis_idx, possible_vels);
                            }
                        }    
                    }
                }
            }
            // This solution assumes. It works on the full input but will fail on the test case.
            // There is a way to determine the velocities in the test case as well.
            // TODO: implement determining velocities for the test case.
            for axis_idx in [Axis::X as usize, Axis::Y as usize, Axis::Z as usize] {
                assert!(self.rock.vel[axis_idx].determined())
            }
            /* TODO: track and then use the known time differences corresponding to the same-velocity
            pairs. can learn the y and z position for any pair where the x velocity is the same, etc.

            Ideal is if we track for each axis: velocity: i, j, delta triple. where delta is such that
            t_i - t_j = delta and i < j.

            Then once we know the velocity for the other axes, the matrix A is:

            1       rock_vel - hailstone_i_vel      0                                               hailstone_i_pos
            1       0                               rock_vel - hailstone_j_vel                      hailstone_j_pos
            0       1                               -1                                              delta

            And the vector b is:

            hailstone_i_pos
            hailstone_j_pos
            delta

            Solving yields:

            rock_pos
            t_i
            t_j

            */
            // Determine the positions
            // TODO: this is just an example to prove that this works.
            // TODO: decide if possible positions should be different data structure from
            // possible velocities
            for time_delta in self.rock.vel[Axis::X as usize].determined_time_deltas() {
                if self.rock.pos[Axis::Y as usize].determined() && self.rock.pos[Axis::Z as usize].determined() {
                    break;
                }
                for axis_idx in [Axis::Y as usize, Axis::Z as usize] {
                    if let Some(pos) = self.determine_position(time_delta, axis_idx) {
                        self.rock.pos[axis_idx].possible.insert(pos, vec![]);
                    }
                }
            }
            for time_delta in self.rock.vel[Axis::Y as usize].determined_time_deltas() {
                if let Some(pos) = self.determine_position(time_delta, Axis::X as usize) {
                    self.rock.pos[Axis::X as usize].possible.insert(pos, vec![]);
                    break;
                }
            }
            Answer::I64(self.rock.sum_of_pos_coords().expect("Should know the answer by now"))
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let re = Regex::new(r"(?<pos_x>\d+)\, +(?<pos_y>\d+)\, +(?<pos_z>\d+) +\@ +(?<vel_x>\-?\d+)\, +(?<vel_y>\-?\d+)\, +(?<vel_z>\-?\d+)").unwrap();
            self.hailstones = io_utils::file_to_lines(filename)
                .map(|line| {
                    let captures = re.captures(&line).unwrap();
                    Projectile {
                        pos: [
                            captures.name("pos_x").unwrap().as_str().parse().unwrap(),
                            captures.name("pos_y").unwrap().as_str().parse().unwrap(),
                            captures.name("pos_z").unwrap().as_str().parse().unwrap(),
                        ],
                        vel: [
                            captures.name("vel_x").unwrap().as_str().parse().unwrap(),
                            captures.name("vel_y").unwrap().as_str().parse().unwrap(),
                            captures.name("vel_z").unwrap().as_str().parse().unwrap(),
                        ],
                    }
                })
                .collect();
        }

        fn mark_vel_impossible(&mut self, axis_idx: usize, vel: i64) {
            self.rock.vel[axis_idx].possible.remove(&vel);
            self.rock.vel[axis_idx].impossible.insert(vel);
        }

        fn mark_vels_possible(&mut self, axis_idx: usize, vels: HashMap<i64, Vec<TimeDelta>>) {
            let vels: HashMap<i64, Vec<TimeDelta>> = vels.into_iter().filter(|(vel, _tds)| !self.rock.vel[axis_idx].impossible.contains(&vel)).collect();
            if self.rock.vel[axis_idx].possible.is_empty() {
                self.rock.vel[axis_idx].possible = vels;
            } else {
                self.rock.vel[axis_idx].possible.retain(|vel, _tds| vels.contains_key(&vel));
                if self.rock.vel[axis_idx].possible.is_empty() { panic!("Emptied possible set"); }
                for (vel, mut new_tds) in vels.into_iter() {
                    self.rock.vel[axis_idx].possible.entry(vel)
                        .and_modify(|tds| tds.append(&mut new_tds));
                }
            }
        }

        fn determine_position(&self, time_delta: &TimeDelta, axis_idx: usize) -> Option<i64> {
            if !self.rock.pos[axis_idx].determined() 
                && self.hailstones[time_delta.i].vel[axis_idx] != self.hailstones[time_delta.j].vel[axis_idx] {
                    let rock_vel = self.rock.vel[axis_idx].determined_value().unwrap() as f64;
                    let a: Array2<f64> = array![
                        [1.0, rock_vel - self.hailstones[time_delta.i].vel[axis_idx] as f64, 0.0], 
                        [1.0, 0.0, rock_vel - self.hailstones[time_delta.j].vel[axis_idx] as f64],
                        [0.0, 1.0, -1.0], // t_i - t_j = delta
                    ];            
                    let b: Array1<f64> = array![
                        self.hailstones[time_delta.i].pos[axis_idx] as f64, 
                        self.hailstones[time_delta.j].pos[axis_idx] as f64, 
                        time_delta.delta as f64,
                    ];
                    let x = a.solve_into(b).unwrap();
                    if x[0] == (x[0] as i64) as f64 {
                        Some(x[0].round() as i64)
                    } else {
                        None
                    }
            } else {
                None
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::I64(47); "example_1")]
        #[test_case(2, Answer::I64(646_810_057_104_753); "example_2")]
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
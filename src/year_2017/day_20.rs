#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 20 };

mod utils {
    //! Utilities shared by both parts of the solution.

    use std::{cmp::Ordering, ops::Index};

    /// Representation of a vector in 3D coordinates.
    #[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
    pub struct Vector {
        pub x: i32,
        pub y: i32,
        pub z: i32,
    }

    impl Vector {
        /// Returns the sum of the absolute value of the 3 dimensions.
        pub fn abs_sum(&self) -> u32 {
            (self.x.abs() + self.y.abs() + self.z.abs()).try_into().unwrap()
        }

        /// Returns a `Vector` created from an input string.
        pub fn from_str(input: &str) -> Self {
            let magnitudes: Vec<i32> = input
                .split(",")
                .map(|magnitude| magnitude.parse().unwrap())
                .collect();
            Vector {
                x: magnitudes[0],
                y: magnitudes[1],
                z: magnitudes[2],
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Axis {
        X,
        Y,
        Z,
    }

    impl Index<Axis> for Vector {
        type Output = i32;

        fn index(&self, axis: Axis) -> &Self::Output {
            match axis {
                Axis::X => &self.x,
                Axis::Y => &self.y,
                Axis::Z => &self.z,
            }
        }
    }

    /// Returns whether a given dimension is 0 or in the positive (1) or negative (-1) direction.
    pub fn dir(dim: i32) -> i32 {
        match dim.cmp(&0) {
            Ordering::Equal => 0,
            _ => dim / dim.abs(),
        }
    }
}

pub mod part_one {
    use std::cmp::Ordering;
    use regex::Regex;
    use crate::utils::{solution::{Solution, Answer}, io_utils};
    use super::utils::{self, Vector};

    #[derive(Debug, Clone, Copy)]
    struct Particle {
        id: usize,
        pos: Vector,
        vel: Vector,
        acc: Vector,
    }

    enum HeadStartType {
        Vel,
        Pos,
    }

    impl Particle {
        fn abs_acc(&self) -> u32 {
            self.acc.abs_sum()
        }

        fn head_start(&self, head_start_type: HeadStartType) -> i32 {
            let vector = match head_start_type {
                HeadStartType::Vel => self.vel,
                HeadStartType::Pos => self.pos,
            };
            vector.x * utils::dir(self.acc.x) +
            vector.y * utils::dir(self.acc.y) +
            vector.z * utils::dir(self.acc.z)
        }
    }

    impl Ord for Particle {
        fn cmp(&self, other: &Self) -> Ordering {
            match self.abs_acc().cmp(&other.abs_acc()) {
                Ordering::Equal => {
                    match self.head_start(HeadStartType::Vel).cmp(&other.head_start(HeadStartType::Vel)) {
                        Ordering::Equal => {
                            self.head_start(HeadStartType::Pos).cmp(&other.head_start(HeadStartType::Pos))
                        },
                        comparison => comparison,
                    }
                },
                comparison => comparison,
            }
        }
    }

    impl PartialOrd for Particle {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for Particle {
        fn eq(&self, other: &Self) -> bool {
            (self.pos, self.vel, self.acc) == (other.pos, other.vel, other.acc)
        }
    }

    impl Eq for Particle {}

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct Soln {
        min_particle: Option<Particle>,
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let re = Regex::new(r"p=<(?<pos>[\-,\d]+)>, v=<(?<vel>[\-,\d]+)>, a=<(?<acc>[\-,\d]+)>").unwrap();
            io_utils::file_to_lines(filename)
                .enumerate()
                .for_each(|(id, particle)| {
                    // parse particle string
                    let captures = re.captures(&particle)
                        .expect("Line should match the regex.");
                    let pos = Vector::from_str(
                        captures.name("pos")
                            .unwrap()
                            .as_str()
                    );
                    let vel = Vector::from_str(
                        captures.name("vel")
                            .unwrap()
                            .as_str()
                    );
                    let acc = Vector::from_str(
                        captures.name("acc")
                            .unwrap()
                            .as_str()
                    );
                    let particle = Particle {
                        id,
                        pos,
                        vel,
                        acc,
                    };
                    match self.min_particle {
                        None => self.min_particle = Some(particle),
                        Some(min_particle) => {
                            if particle == min_particle { 
                                panic!("Found equal particles.");
                            } else if particle < min_particle {
                                self.min_particle = Some(particle);
                            }        
                        },                        
                    }
                });
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.min_particle.unwrap().id.try_into().unwrap())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(0); "example_1")]
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
    use std::collections::{HashMap, HashSet};

    use itertools::Itertools;
    use regex::Regex;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    use super::utils::{Vector, Axis};

    enum AxisCollides {
        Never,
        Always,
        Once(u32),
        Twice(u32, u32),
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct Particle {
        id: usize,
        pos: Vector,
        vel: Vector,
        acc: Vector,
    }

    impl Particle {
        /// Returns the earliest time at which the two particles collide, if any.
        /// We'll denote initial position for a given axis as $p$, 
        /// initial velocity as $v$, and initial acceleration as $a$. 
        /// 
        /// A particle's position along each axis follows the series:
        ///     At $t = 0$: $p$
        ///     At $t = 1$: $p + v + a$
        ///     At $t = 2$: $p + v + a + (v + 2a) = p + 2v + 3a$
        ///     At $t = 3$: $p + 2v + 3a + (v + 4a) = p + 3v + 6a$
        ///     ...
        /// 
        /// At any given $t$, this equates to $p + vt + {at(t+1) \over 2}$.
        /// 
        /// Two particles $l$ and $r$ collide at time $t$ if and only if, for each axis:
        /// $$ a_lt^2 + (2v_l + a_l)t + 2p_l = a_rt^2 + (2v_r + a_r)t + 2p_r$$
        /// 
        /// This uses the quadratic formula to find the positive integer roots, if any, 
        /// for each axis and returns the minimum positive shared integer root across axes.
        // TODO: fix where the comment is.
        fn collision_time(&self, other: &Particle) -> Option<u32> {
            let mut positive_integer_roots = HashSet::new();
            for axis in [Axis::X, Axis::Y, Axis::Z] {
                match self.axis_collides(other, axis) {
                    AxisCollides::Never => return None,
                    AxisCollides::Always => (),
                    AxisCollides::Once(t) => {
                        if positive_integer_roots.is_empty() || positive_integer_roots.contains(&t) {
                            positive_integer_roots = HashSet::from([t]);
                        } else {
                            return None;
                        }
                    },
                    AxisCollides::Twice(t1, t2) => {
                        let axis_times = HashSet::from([t1, t2]);
                        if positive_integer_roots.is_empty() { 
                            positive_integer_roots = axis_times; 
                        } else {
                            positive_integer_roots.retain(|&t| axis_times.contains(&t));
                        } 
                    },
                }
            }
            positive_integer_roots.iter().min().copied()
        }

        fn axis_collides(&self, other: &Particle, axis: Axis) -> AxisCollides {
                let a = self.acc[axis] - other.acc[axis];
                let b = 2 * (self.vel[axis] - other.vel[axis]) + self.acc[axis] - other.acc[axis];
                let c = 2 * (self.pos[axis] - other.pos[axis]);
                if a == 0 {
                    // solve the bt + c linear equation. t = - c / b. 
                    if b == 0 {
                        if c == 0 { return AxisCollides::Always; } else { return AxisCollides::Never; }
                    } else {
                        if c % b != 0 || - c / b < 0 { return AxisCollides::Never; }
                        return AxisCollides::Once((- c / b).try_into().unwrap());
                    }
                } else {
                    let sqrt = ((b.pow(2) - 4 * a * c) as f64).sqrt();
                    if sqrt.is_nan() { return AxisCollides::Never; }
                    if sqrt != (sqrt as u32) as f64 { 
                        // sqrt is not an integer
                        return AxisCollides::Never;
                    }
                    let sqrt = sqrt as i32;
                    // check that the entire expression is an int and positive
                    let mut non_negative_integer_roots: HashSet<u32> = HashSet::new();
                    if (-b + sqrt) % (2 * a) == 0 && (-b + sqrt) / (2 * a) >= 0 {
                        non_negative_integer_roots.insert(((-b + sqrt) / (2 * a)).try_into().unwrap());
                    }
                    if (-b - sqrt) % (2 * a) == 0 && (-b - sqrt) / (2 * a) >= 0 {
                        non_negative_integer_roots.insert(((-b - sqrt) / (2 * a)).try_into().unwrap());
                    }
                    match non_negative_integer_roots.len() {
                        0 => AxisCollides::Never,
                        1 => AxisCollides::Once(
                            *non_negative_integer_roots.iter().next().unwrap(),
                        ),
                        2 => AxisCollides::Twice(
                            *non_negative_integer_roots.iter().next().unwrap(),
                            *non_negative_integer_roots.iter().next().unwrap(),
                        ),
                        _ => panic!("More than 2 roots found."),
                    }
                }
        }
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct Soln {
        /// All particles.
        particles: Vec<Particle>,
        /// Maps a pair of `Particle` IDs to the earliest tick at which they collide.
        particle_id_pair_collisions: HashMap<[usize; 2], u32>,
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let re = Regex::new(r"p=<(?<pos>[\-,\d]+)>, v=<(?<vel>[\-,\d]+)>, a=<(?<acc>[\-,\d]+)>").unwrap();
            io_utils::file_to_lines(filename)
                .enumerate()
                .for_each(|(id, particle)| {
                    // parse particle string
                    let captures = re.captures(&particle)
                        .expect("Line should match the regex.");
                    let pos = Vector::from_str(
                        captures.name("pos")
                            .unwrap()
                            .as_str()
                    );
                    let vel = Vector::from_str(
                        captures.name("vel")
                            .unwrap()
                            .as_str()
                    );
                    let acc = Vector::from_str(
                        captures.name("acc")
                            .unwrap()
                            .as_str()
                    );
                    let particle = Particle {
                        id,
                        pos,
                        vel,
                        acc,
                    };
                    for p in self.particles.iter() {
                        if let Some(t) = p.collision_time(&particle) {
                            self.particle_id_pair_collisions.insert([p.id, particle.id], t);
                        }
                    }
                    self.particles.push(particle);
                });
        }

        fn surviving_particles(&self) -> usize {
            let mut first_collisions: HashMap<usize, u32> = HashMap::new();
            self.particle_id_pair_collisions
                .iter()
                .sorted_by_key(|&(_particle_ids, t)| t)
                .for_each(|(particle_ids, t)| {
                    if let Some(first_collision_0) = first_collisions.get(&particle_ids[0]) {
                        if first_collision_0 < t { return; }
                    }
                    if let Some(first_collision_1) = first_collisions.get(&particle_ids[1]) {
                        if first_collision_1 < t { return; }
                    }
                    for particle_id in particle_ids {
                        first_collisions.insert(*particle_id, *t);                        
                    }
                });
            self.particles.len() - first_collisions.len()
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.surviving_particles().try_into().unwrap())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(2, Answer::U32(1); "example_2")]
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
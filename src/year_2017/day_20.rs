#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 20 };

pub mod part_one {
    use std::cmp::Ordering;

    use regex::Regex;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct Vector {
        x: i32,
        y: i32,
        z: i32,
    }

    impl Vector {
        fn abs_sum(&self) -> u32 {
            (self.x.abs() + self.y.abs() + self.z.abs()).try_into().unwrap()
        }

        fn from_str(input: &str) -> Self {
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

    fn dir(dim: i32) -> i32 {
        match dim.cmp(&0) {
            Ordering::Equal => 0,
            Ordering::Less => -1,
            Ordering::Greater => 1,
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct Particle {
        id: usize,
        pos: Vector,
        vel: Vector,
        acc: Vector,
    }

    impl Particle {
        fn abs_acc(&self) -> u32 {
            self.acc.abs_sum()
        }

        fn vel_head_start(&self) -> i32 {
            let x = self.vel.x * dir(self.acc.x);
            let y = self.vel.y * dir(self.acc.y);
            let z = self.vel.z * dir(self.acc.z);
            x + y + z
        }

        fn pos_head_start(&self) -> i32 {
            let x = self.pos.x * dir(self.acc.x);
            let y = self.pos.y * dir(self.acc.y);
            let z = self.pos.z * dir(self.acc.z);
            x + y + z
        }
    }

    impl Ord for Particle {
        fn cmp(&self, other: &Self) -> Ordering {
            match self.abs_acc().cmp(&other.abs_acc()) {
                Ordering::Equal => {
                    match self.vel_head_start().cmp(&other.vel_head_start()) {
                        Ordering::Equal => {
                            self.pos_head_start().cmp(&other.pos_head_start())
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
                    if id == 0 {
                        self.min_particle = Some(particle)
                    } else {
                        let min_particle = self.min_particle.unwrap();
                        if particle == min_particle { 
                            panic!("Found equal particles.");
                        } else if particle < min_particle {
                            self.min_particle = Some(particle);
                        }    
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
#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 10 };

pub mod part_one {
    use crate::utils::{io_utils, solution::{Solution, Answer}};

    #[derive(PartialEq, Eq, Debug)]
    pub struct Soln {
        nums: Vec<u8>,
        lengths: Vec<usize>,
        position: usize,
        skip_size: usize,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self::with_max(u8::MAX)
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            for length_idx in 0..self.lengths.len() {
                self.step(*self.lengths.get(length_idx).unwrap())
            }
            Answer::U16(self.check())
        }
    }

    impl Soln {
        fn with_max(max: u8) -> Self {
            Soln {
                nums: (0..=max).collect(),
                lengths: vec![],
                position: 0,
                skip_size: 0,
            }
        }    

        fn parse_input_file(&mut self, filename: &str) {
            self.lengths = io_utils::file_to_string(filename)
                .split(",")
                .map(|num| {
                    num.parse::<usize>().expect("Should be able to parse input to an unsigned integer.")
                })
                .collect();
        }
    
        fn step(&mut self, length: usize) {
            if self.position + length < self.nums.len() {
                self.nums[self.position..self.position + length].reverse();
            } else {
                // Circularity applies and requires a special case
                let mut full_reversal_region: Vec<u8> = self.nums[self.position..self.nums.len()]
                    .to_vec();
                full_reversal_region.append(
                        &mut self.nums[0..(self.position + length - self.nums.len())].to_vec()
                    );
                full_reversal_region.reverse();
                self.nums.splice(self.position..self.nums.len(), full_reversal_region[..self.nums.len() - self.position].to_vec());
                self.nums.splice(0..self.position + length - self.nums.len(), full_reversal_region[self.nums.len() - self.position..].to_vec());
            }
            self.position = (self.position + length + self.skip_size) % self.nums.len();
            self.skip_size += 1;
        }

        fn check(&self) -> u16 {
            self.nums[..2].iter().fold(1, |acc, num| acc * *num as u16)
        }
    }
 
    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test]
        fn step_is_correct() {
            let mut soln = Soln::with_max(4);
            soln.step(0);
            assert_eq!(
                soln,
                Soln {
                    nums: vec![0, 1, 2, 3, 4],
                    lengths: vec![],
                    position: 0,
                    skip_size: 1,
                },
            );
            soln.step(1);
            assert_eq!(
                soln,
                Soln {
                    nums: vec![0, 1, 2, 3, 4],
                    lengths: vec![],
                    position: 2,
                    skip_size: 2,
                },
            );
            soln.step(5);
            assert_eq!(
                soln,
                Soln {
                    nums: vec![3, 2, 1, 0, 4],
                    lengths: vec![],
                    position: 4,
                    skip_size: 3,
                },
            );
            soln.step(4);
            assert_eq!(
                soln,
                Soln {
                    nums: vec![2, 3, 4, 0, 1],
                    lengths: vec![],
                    position: 1,
                    skip_size: 4,
                },
            );
        }

        #[test_case(1, Answer::U16(12); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::with_max(4),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}


pub mod part_two {
    use itertools::Itertools;

    use crate::utils::{io_utils, solution::{Solution, Answer}};

    const ROUNDS: u8 = 64;
    const DENSE_HASH_CHUNK_SIZE: usize = 16;

    #[derive(PartialEq, Eq, Debug)]
    pub struct Soln {
        nums: Vec<u8>,
        lengths: Vec<usize>,
        position: usize,
        skip_size: usize,
    }

    // TODO: refactor to shared functionality?
    impl Default for Soln {
        fn default() -> Self {
            Self::with_max(u8::MAX)
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            for _round in 0..ROUNDS {
                for length_idx in 0..self.lengths.len() {
                    self.step(*self.lengths.get(length_idx).unwrap())
                }
            }
            Answer::String(self.knot_hash())
        }
    }

    impl Soln {
        // TODO: refactor shared functionality?
        fn with_max(max: u8) -> Self {
            Soln {
                nums: (0..=max).collect(),
                lengths: vec![],
                position: 0,
                skip_size: 0,
            }
        }    

        fn parse_input_file(&mut self, filename: &str) {
            self.lengths = io_utils::file_to_string(filename)
                .chars()
                .map(|ch| ch as usize)
                .collect();
            self.lengths.append(&mut vec![17usize, 31, 73, 47, 23]);
        }
    
        // TODO: split into shared functionality?
        fn step(&mut self, length: usize) {
            if self.position + length < self.nums.len() {
                self.nums[self.position..self.position + length].reverse();
            } else {
                // Circularity applies and requires a special case
                let mut full_reversal_region: Vec<u8> = self.nums[self.position..self.nums.len()]
                    .to_vec();
                full_reversal_region.append(
                        &mut self.nums[0..(self.position + length - self.nums.len())].to_vec()
                    );
                full_reversal_region.reverse();
                self.nums.splice(self.position..self.nums.len(), full_reversal_region[..self.nums.len() - self.position].to_vec());
                self.nums.splice(0..self.position + length - self.nums.len(), full_reversal_region[self.nums.len() - self.position..].to_vec());
            }
            self.position = (self.position + length + self.skip_size) % self.nums.len();
            self.skip_size += 1;
        }

        fn knot_hash(&mut self) -> String {
            let nums: [u8; 256] = self.nums.clone().try_into().expect("Should be exactly 256 numbers.");
            nums
                .into_iter()
                .chunks(DENSE_HASH_CHUNK_SIZE)
                .into_iter()
                .map(|chunk| -> String {
                    format!("{:02x}", chunk
                    .reduce(|acc, num| acc ^ num)
                    .expect("No chunk should be empty."))
                })                
                .join("")
        }
    }
 
    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test]
        fn step_is_correct() {
            let mut soln = Soln::with_max(4);
            soln.step(0);
            assert_eq!(
                soln,
                Soln {
                    nums: vec![0, 1, 2, 3, 4],
                    lengths: vec![],
                    position: 0,
                    skip_size: 1,
                },
            );
            soln.step(1);
            assert_eq!(
                soln,
                Soln {
                    nums: vec![0, 1, 2, 3, 4],
                    lengths: vec![],
                    position: 2,
                    skip_size: 2,
                },
            );
            soln.step(5);
            assert_eq!(
                soln,
                Soln {
                    nums: vec![3, 2, 1, 0, 4],
                    lengths: vec![],
                    position: 4,
                    skip_size: 3,
                },
            );
            soln.step(4);
            assert_eq!(
                soln,
                Soln {
                    nums: vec![2, 3, 4, 0, 1],
                    lengths: vec![],
                    position: 1,
                    skip_size: 4,
                },
            );
        }

        #[test_case(2, Answer::String(String::from("a2582a3a0e66e6e86e3812dcb672a272")); "example_2")]
        #[test_case(3, Answer::String(String::from("33efeb34ea91902bb2f59c9920caa6cd")); "example_3")]
        #[test_case(4, Answer::String(String::from("3efbe78a8d82f29979031a4aa0b16a9d")); "example_4")]
        #[test_case(5, Answer::String(String::from("63960835bcdc130f0b66d7ff4f6a5a8e")); "example_5")]
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
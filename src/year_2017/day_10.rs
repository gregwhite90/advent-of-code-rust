#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 10 };

mod utils {
    use crate::utils::solution::Solution;

    pub fn range_vec_max(max: u8) -> Vec<u8> {
        (0..=max).collect()
    }

    pub trait Year2017Day10Solution {
        fn set_nums(&mut self, nums: Vec<u8>);
        fn get_nums(&self) -> &Vec<u8>;
        fn get_mut_nums(&mut self) -> &mut Vec<u8>;
        fn get_length(&self) -> usize;
        fn increment_length_idx(&mut self);
        fn get_position(&self) -> usize;
        fn set_position(&mut self, position: usize);
        fn get_skip_size(&self) -> usize;
        fn increment_skip_size(&mut self);
    }

    pub fn step<T>(soln: &mut T) 
    where
        T: Solution + Year2017Day10Solution 
    {
        let position = soln.get_position();
        let nums_len = soln.get_nums().len();
        let length = soln.get_length();
        if position + length < nums_len {
            soln.get_mut_nums()[position..position + length].reverse();
        } else {
            // Circularity applies and requires a special case
            let mut full_reversal_region: Vec<u8> = soln.get_nums()[position..nums_len]
                .to_vec();
            full_reversal_region.append(
                    &mut soln.get_nums()[0..(position + length - nums_len)].to_vec()
                );
            full_reversal_region.reverse();
            soln.get_mut_nums().splice(position..nums_len, full_reversal_region[..nums_len - position].to_vec());
            soln.get_mut_nums().splice(0..position + length - nums_len, full_reversal_region[nums_len - position..].to_vec());
        }
        soln.set_position((position + length + soln.get_skip_size()) % nums_len);
        soln.increment_skip_size();
        soln.increment_length_idx();
    }

}

pub mod part_one {
    use crate::utils::{io_utils, solution::{Solution, Answer}};

    use super::utils::{self, Year2017Day10Solution};

    #[derive(PartialEq, Eq, Debug)]
    pub struct Soln {
        nums: Vec<u8>,
        lengths: Vec<usize>,
        length_idx: usize,
        position: usize,
        skip_size: usize,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self::with_max(u8::MAX)
        }
    }

    impl Year2017Day10Solution for Soln {
        fn set_nums(&mut self, nums: Vec<u8>) {
            self.nums = nums;
        }

        fn get_nums(&self) -> &Vec<u8> {
            &self.nums
        }
        fn get_mut_nums(&mut self) -> &mut Vec<u8> {
            &mut self.nums
        }

        fn get_length(&self) -> usize {
            *self.lengths.get(self.length_idx).expect("Should be able to get the length at the current index.")
        }

        fn increment_length_idx(&mut self) {
            self.length_idx = (self.length_idx + 1) % self.lengths.len();
        }

        fn get_position(&self) -> usize {
            self.position
        }

        fn set_position(&mut self, position: usize) {
            self.position = position;
        }

        fn get_skip_size(&self) -> usize {
            self.skip_size
        }

        fn increment_skip_size(&mut self) {
            self.skip_size += 1;
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            for _length_idx in 0..self.lengths.len() {
                utils::step(self);
            }
            Answer::U16(self.check())
        }
    }

    impl Soln {
        fn with_max(max: u8) -> Self {
            Soln {
                nums: utils::range_vec_max(max),
                lengths: vec![],
                length_idx: 0,
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
            let mut soln = Soln {
                nums: utils::range_vec_max(4),
                lengths: vec![0, 1, 5, 4],
                ..Soln::default()
            };
            utils::step(&mut soln);
            assert_eq!(
                soln,
                Soln {
                    nums: vec![0, 1, 2, 3, 4],
                    lengths: vec![0, 1, 5, 4],
                    length_idx: 1,
                    position: 0,
                    skip_size: 1,
                },
            );
            utils::step(&mut soln);
            assert_eq!(
                soln,
                Soln {
                    nums: vec![0, 1, 2, 3, 4],
                    lengths: vec![0, 1, 5, 4],
                    length_idx: 2,
                    position: 2,
                    skip_size: 2,
                },
            );
            utils::step(&mut soln);
            assert_eq!(
                soln,
                Soln {
                    nums: vec![3, 2, 1, 0, 4],
                    lengths: vec![0, 1, 5, 4],
                    length_idx: 3,
                    position: 4,
                    skip_size: 3,
                },
            );
            utils::step(&mut soln);
            assert_eq!(
                soln,
                Soln {
                    nums: vec![2, 3, 4, 0, 1],
                    lengths: vec![0, 1, 5, 4],
                    length_idx: 0,
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

    use super::utils::{self, Year2017Day10Solution};

    const ROUNDS: u8 = 64;
    const DENSE_HASH_CHUNK_SIZE: usize = 16;

    #[derive(PartialEq, Eq, Debug)]
    pub struct Soln {
        nums: Vec<u8>,
        lengths: Vec<usize>,
        length_idx: usize,
        position: usize,
        skip_size: usize,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self::with_max(u8::MAX)
        }
    }
    
    impl Year2017Day10Solution for Soln {
        fn set_nums(&mut self, nums: Vec<u8>) {
            self.nums = nums;
        }

        fn get_nums(&self) -> &Vec<u8> {
            &self.nums
        }
        fn get_mut_nums(&mut self) -> &mut Vec<u8> {
            &mut self.nums
        }

        fn get_length(&self) -> usize {
            *self.lengths.get(self.length_idx).expect("Should be able to get the length at the current index.")
        }
        
        fn increment_length_idx(&mut self) {
            self.length_idx = (self.length_idx + 1) % self.lengths.len();
        }

        fn get_position(&self) -> usize {
            self.position
        }

        fn set_position(&mut self, position: usize) {
            self.position = position;
        }

        fn get_skip_size(&self) -> usize {
            self.skip_size
        }

        fn increment_skip_size(&mut self) {
            self.skip_size += 1;
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            for _round in 0..ROUNDS {
                for _length_idx in 0..self.lengths.len() {
                    utils::step(self);
                }
            }
            Answer::String(self.knot_hash())
        }
    }

    impl Soln {
        fn with_max(max: u8) -> Self {
            Soln {
                nums: utils::range_vec_max(max),
                lengths: vec![],
                length_idx:0,
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
    
        fn knot_hash(&self) -> String {
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
            let mut soln = Soln {
                nums: utils::range_vec_max(4),
                lengths: vec![0, 1, 5, 4],
                ..Soln::default()
            };
            utils::step(&mut soln);
            assert_eq!(
                soln,
                Soln {
                    nums: vec![0, 1, 2, 3, 4],
                    lengths: vec![0, 1, 5, 4],
                    length_idx: 1,
                    position: 0,
                    skip_size: 1,
                },
            );
            utils::step(&mut soln);
            assert_eq!(
                soln,
                Soln {
                    nums: vec![0, 1, 2, 3, 4],
                    lengths: vec![0, 1, 5, 4],
                    length_idx: 2,
                    position: 2,
                    skip_size: 2,
                },
            );
            utils::step(&mut soln);
            assert_eq!(
                soln,
                Soln {
                    nums: vec![3, 2, 1, 0, 4],
                    lengths: vec![0, 1, 5, 4],
                    length_idx: 3,
                    position: 4,
                    skip_size: 3,
                },
            );
            utils::step(&mut soln);
            assert_eq!(
                soln,
                Soln {
                    nums: vec![2, 3, 4, 0, 1],
                    lengths: vec![0, 1, 5, 4],
                    length_idx: 0,
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
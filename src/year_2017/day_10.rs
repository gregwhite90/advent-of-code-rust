#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 10 };

pub mod part_one {
    use crate::{utils::{io_utils, solution::{Solution, Answer}}, year_2017::utils::utils::KnotHasher};

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
    
    impl KnotHasher for Soln {
        fn set_nums(&mut self, nums: Vec<u8>) {
            self.nums = nums;
        }

        fn get_nums(&self) -> &Vec<u8> {
            &self.nums
        }
        fn get_mut_nums(&mut self) -> &mut Vec<u8> {
            &mut self.nums
        }

        fn set_lengths(&mut self, lengths: Vec<usize>) {
            self.lengths = lengths;
        }

        fn get_lengths(&self) -> &Vec<usize> {
            &self.lengths
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
                self.step();
            }
            Answer::U16(self.check())
        }
    }

    impl Soln {
        fn with_max(max: u8) -> Self {
            Soln {
                nums: Self::range_vec_max(max),
                lengths: vec![],
                length_idx:0,
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
            let mut soln = Soln::with_max(4);
            soln.set_lengths(vec![0, 1, 5, 4]);
            soln.step();
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
            soln.step();
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
            soln.step();
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
            soln.step();
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
            let mut soln = Soln::with_max(4);
            test_utils::check_example_case(
                &mut soln,
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}


pub mod part_two {
    use crate::{utils::solution::{Solution, Answer}, year_2017::utils::utils::KnotHasher};

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
    
    impl KnotHasher for Soln {
        fn set_nums(&mut self, nums: Vec<u8>) {
            self.nums = nums;
        }

        fn get_nums(&self) -> &Vec<u8> {
            &self.nums
        }
        fn get_mut_nums(&mut self) -> &mut Vec<u8> {
            &mut self.nums
        }

        fn set_lengths(&mut self, lengths: Vec<usize>) {
            self.lengths = lengths;
        }

        fn get_lengths(&self) -> &Vec<usize> {
            &self.lengths
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
            self.all_steps();
            Answer::String(self.knot_hash())
        }
    }

    impl Soln {
        fn with_max(max: u8) -> Self {
            Soln {
                nums: Self::range_vec_max(max),
                lengths: vec![],
                length_idx:0,
                position: 0,
                skip_size: 0,
            }
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
            soln.set_lengths(vec![0, 1, 5, 4]);
            soln.step();
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
            soln.step();
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
            soln.step();
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
            soln.step();
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
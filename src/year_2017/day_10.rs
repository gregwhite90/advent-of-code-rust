#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 10 };

mod utils {
    use crate::utils::{io_utils, solution::Solution};

    pub trait Year2017Day10Solution {
        fn set_lengths(&mut self, lengths: Vec<usize>);
    }

    pub fn parse_input_file<T>(
        soln: &mut T,       
        filename: &str,
    )
    where
        T: Solution + Year2017Day10Solution
    {
        soln.set_lengths(
            io_utils::file_to_string(filename)
                .split(",")
                .map(|num| {
                    num.parse::<usize>().expect("Should be able to parse input to an unsigned integer.")
                })
                .collect()
        );
    }
}

pub mod part_one {
    use crate::utils::solution::{Solution, Answer};
    use super::utils::{self, Year2017Day10Solution};

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
            utils::parse_input_file(
                self,
                filename,
            );
            for length_idx in 0..self.lengths.len() {
                self.step(*self.lengths.get(length_idx).unwrap())
            }
            Answer::U16(self.check())
        }
    }

    impl Year2017Day10Solution for Soln {
        fn set_lengths(&mut self, lengths: Vec<usize>) {
            self.lengths = lengths;
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

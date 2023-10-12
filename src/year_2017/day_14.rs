#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 14 };

const ROWS: usize = 128;

pub mod part_one {
    use crate::{utils::{solution::{Solution, Answer}, io_utils}, year_2017::utils::utils::KnotHasher};

    use super::ROWS;

    #[derive(PartialEq, Eq, Debug)]
    pub struct Hasher {
        nums: Vec<u8>,
        lengths: Vec<usize>,
        length_idx: usize,
        position: usize,
        skip_size: usize,
    }

    impl Default for Hasher {
        fn default() -> Self {
            Self::with_max(u8::MAX)
        }
    }
    
    impl KnotHasher for Hasher {
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

    impl Hasher {
        fn with_max(max: u8) -> Self {
            Hasher {
                nums: Self::range_vec_max(max),
                lengths: vec![],
                length_idx:0,
                position: 0,
                skip_size: 0,
            }
        }    
    }

    #[derive(PartialEq, Eq, Debug, Default)]
    pub struct Soln {
        ones: u32,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.ones)
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let key = io_utils::file_to_string(filename);
            self.ones = (0..ROWS)
                .map(|idx| {
                    let mut hasher = Hasher::default();
                    hasher.parse_key(&format!("{key}-{idx}"));
                    hasher.all_steps();
                    u128::from_str_radix(&hasher.knot_hash(), 16)
                        .expect("Should be able to parse base 16 string to u128.")
                        .count_ones()
                })
                .sum()
        }
    }
 
    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(8108); "example_1")]
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
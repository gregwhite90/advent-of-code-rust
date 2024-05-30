#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 16 };

mod utils {
    use itertools::Itertools;

    use crate::utils::io_utils;

    #[derive(Debug)]
    pub struct DragonChecksum {
        disk_len: usize,
        initial_state: String,
    }

    impl DragonChecksum {
        pub fn with_disk_len(disk_len: usize) -> Self {
            Self {
                disk_len,
                initial_state: String::new(),
            }
        }

        pub fn parse_input_file(&mut self, filename: &str) {
            self.initial_state = io_utils::file_to_string(filename);
        }

        pub fn generate_data(&self) -> String {
            let mut data = self.initial_state.clone();
            while data.len() < self.disk_len {
                let reversed = data.chars().rev().map(|ch| if ch == '1' { '0' } else { '1' }).collect::<String>();
                data.push('0');
                data.push_str(&reversed);
            }
            data[..self.disk_len].to_string()
        }

        pub fn checksum(&self, mut data: String) -> String {
            while data.len() % 2 == 0 {
                data = data.chars().chunks(2).into_iter().map(|mut chunk| {
                    if chunk.next().unwrap() == chunk.next().unwrap() { '1' } else { '0' }
                }).collect::<String>();
            }
            data
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::DragonChecksum;

    #[derive(Debug)]
    pub struct Soln {
        dragon_checksum: DragonChecksum,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self::with_disk_len(272)
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.dragon_checksum.parse_input_file(filename);
            Answer::String(self.dragon_checksum.checksum(self.dragon_checksum.generate_data()))
        }
    }

    impl Soln { 
        fn with_disk_len(disk_len: usize) -> Self {
            Self {
                dragon_checksum: DragonChecksum::with_disk_len(disk_len),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, 20, Answer::String("01100".to_string()); "example_1")]
        fn examples_are_correct(example_key: u8, disk_len: usize, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::with_disk_len(disk_len),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}

pub mod part_two {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::DragonChecksum;

    #[derive(Debug)]
    pub struct Soln {
        dragon_checksum: DragonChecksum,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self {
                dragon_checksum: DragonChecksum::with_disk_len(35_651_584),
            }
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.dragon_checksum.parse_input_file(filename);
            Answer::String(self.dragon_checksum.checksum(self.dragon_checksum.generate_data()))
        }
    }
}
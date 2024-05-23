#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 5 };

pub mod part_one {
    use md5::{Md5, Digest};
    use hex_literal::hex;
    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, Default)]
    pub struct Soln {
        door_id: String,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::String(self.password())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.door_id = io_utils::file_to_string(filename);
        }   

        fn password(&self) -> String {
            let mut p = String::new();
            let mut i: u32 = 0;
            while p.len() != 8 {
                let mut hasher = Md5::new();
                hasher.update(format!("{}{}", self.door_id, i).as_bytes());
                let result = hasher.finalize();
                if result[..2] == hex!("0000") && result[2] < 0x10 {
                    p.push_str(&format!("{:x}", result[2]));
                }
                i += 1;
            }
            p
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::String(String::from("18f47a30")); "example_1")]
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
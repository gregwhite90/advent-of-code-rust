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

pub mod part_two {
    use md5::{Md5, Digest};
    use hex_literal::hex;
    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, Default)]
    struct Password {
        characters: [Option<char>; 8],
        finished: bool,
    }

    impl Password {
        fn push(&mut self, position: u8, character: char) {
            if position >= 8 || self.characters[position as usize] != None { return; }
            self.characters[position as usize] = Some(character); 
            self.finished = self.characters.iter().all(|ch| *ch != None);
        }

        fn finished(&self) -> bool {
            self.finished
        }

        fn to_string(&self) -> String {
            assert!(self.finished());
            self.characters.iter().map(|ch| ch.unwrap()).collect::<String>()
        }
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        door_id: String,
        password: Password,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            self.calculate_password();
            Answer::String(self.password())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.door_id = io_utils::file_to_string(filename);
        }   

        fn calculate_password(&mut self) {
            let mut i: u32 = 0;
            while !self.password.finished() {
                let mut hasher = Md5::new();
                hasher.update(format!("{}{}", self.door_id, i).as_bytes());
                let result = hasher.finalize();
                if result[..2] == hex!("0000") && result[2] < 0x10 {
                    self.password.push(result[2], format!("{:02x}", result[3]).chars().next().unwrap());
                }
                i += 1;
            }
        }

        fn password(&self) -> String {
            self.password.to_string()
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::String(String::from("05ace8e3")); "example_1")]
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
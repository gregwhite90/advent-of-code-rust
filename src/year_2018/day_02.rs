#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 2 };

pub mod part_one {
    use std::collections::HashMap;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug)]
    pub struct BoxID {
        letter_count: HashMap<char, u32>,
    }

    impl BoxID {
        pub fn from_str(input: &str) -> Self {
            let mut letter_count = HashMap::new();
            input.chars().for_each(|ch| *letter_count.entry(ch).or_default() += 1);
            Self {
                letter_count,
            }
        }

        pub fn exact_count(&self, count: u32) -> usize {
            self.letter_count.values().filter(|c| **c == count).count()
        }
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        checksum: usize,    
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::Usize(self.checksum)
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let mut twos: usize = 0;
            let mut threes: usize = 0;
            io_utils::file_to_lines(filename)
                .map(|line| BoxID::from_str(&line))
                .for_each(|b_id| {
                    if b_id.exact_count(2) > 0 { twos += 1; }
                    if b_id.exact_count(3) > 0 { threes += 1; }
                });
            self.checksum = twos * threes;
        }   
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(12); "example_1")]
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
    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, Default)]
    pub struct Soln {
        ids: Vec<String>,    
        shared_id: String,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::String(self.shared_id.clone())
        }
    }

    fn different_char_indices(l: &str, r: &str) -> Vec<usize> {
        l.chars().zip(r.chars()).enumerate().filter(|(_idx, (l_ch, r_ch))| {
            l_ch != r_ch
        })
            .map(|(idx, (_, _))| idx)
            .collect()
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            for line in io_utils::file_to_lines(filename) {
                for prev in self.ids.iter() {
                    let diff = different_char_indices(&line, &prev);
                    if diff.len() == 1 {
                        let mut l = line.clone();
                        l.remove(diff[0]);
                        self.shared_id = l;
                        return;
                    }
                }
                self.ids.push(line);
            }
        }   
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(2, Answer::String("fgij".to_string()); "example_2")]
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
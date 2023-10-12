#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 10 };

pub mod part_one {
    use crate::{utils::{io_utils, solution::{Solution, Answer}}, year_2017::utils::knot_hasher::KnotHasher};

    #[derive(PartialEq, Eq, Debug, Default)]
    pub struct Soln {
        knot_hasher: KnotHasher,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            for _length_idx in 0..self.knot_hasher.lengths().len() {
                self.knot_hasher.step();
            }
            Answer::U16(self.check())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.knot_hasher.set_lengths(
                io_utils::file_to_string(filename)
                    .split(",")
                    .map(|num| {
                        num.parse::<usize>().expect("Should be able to parse input to an unsigned integer.")
                    })
                    .collect()
            );
        }
    
        fn check(&self) -> u16 {
            self.knot_hasher.nums()[..2].iter().fold(1, |acc, num| acc * *num as u16)
        }
    }
 
    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U16(12); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            let mut soln = Soln { knot_hasher: KnotHasher::with_max(4) };
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
    use crate::{utils::solution::{Solution, Answer}, year_2017::utils::knot_hasher::KnotHasher};

    #[derive(PartialEq, Eq, Debug, Default)]
    pub struct Soln {
        knot_hasher: KnotHasher,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.knot_hasher.parse_input_file(filename);
            self.knot_hasher.all_steps();
            Answer::String(self.knot_hasher.knot_hash())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

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
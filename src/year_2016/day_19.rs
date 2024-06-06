#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 19 };

pub mod part_one {
    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, Default)]
    pub struct Soln {
        start_num: usize,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::Usize(self.position_getting_all_presents())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.start_num = io_utils::file_to_string(filename).parse().unwrap();
        }

        fn position_getting_all_presents(&self) -> usize {
            let mut num_left = self.start_num;
            let mut start = 1;
            let mut depth = 1;
            while num_left != 1 {
                if num_left % 2 == 1 {
                    start += 2_usize.pow(depth);
                }
                num_left /= 2;
                depth += 1;
            }
            start
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(3); "example_1")]
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
        start_num: usize,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::Usize(self.position_getting_all_presents())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.start_num = io_utils::file_to_string(filename).parse().unwrap();
        }

        fn position_getting_all_presents(&self) -> usize {
            let mut indices: Vec<usize> = (1..=self.start_num).collect();
            while indices.len() > 1 {
                indices = one_round(indices);
            }
            indices[0]
        }
    }

    fn one_round(indices: Vec<usize>) -> Vec<usize> {
        // Base cases
        if indices.len() <= 1 { panic!("Length must be at least 2."); }
        if indices.len() == 2 { return vec![indices[0]]; }
        if indices.len() == 3 { return vec![indices[2]]; }

        // Recursive cases. These depend on the length of the vector,
        // and follow a pattern of how it steps that I discovered by
        // running one round on different start numbers up to 50.
        // TODO: could simplify this similar behavior into a function
        let bookend_step_by: usize = 3;
        let num_steps = indices.len() / 3 + if indices.len() % 3 != 0 { 1 } else { 0 } - 1;
        match indices.len() % 6 {
            0 | 3 => indices.into_iter().skip(2).step_by(bookend_step_by).collect(),
            1 => {
                let bookend_num_steps = (num_steps - 2) / 2;
                indices.iter().step_by(bookend_step_by).take(bookend_num_steps).copied()
                    .chain(
                        indices.iter().skip(bookend_num_steps * bookend_step_by).step_by(2).take(2).copied()
                    )
                    .chain(
                        indices.iter().skip(bookend_num_steps * bookend_step_by + 2 * 2).step_by(bookend_step_by).copied()
                    )
                    .collect()
            },
            2 => {
                let bookend_num_steps = (num_steps - 2) / 2;
                indices.iter().skip(1).step_by(bookend_step_by).take(bookend_num_steps).copied()
                    .chain(
                        indices.iter().skip(1 + bookend_num_steps * bookend_step_by).step_by(2).take(1).copied()
                    )
                    .chain(
                        indices.iter().skip(1 + bookend_num_steps * bookend_step_by + 1 * 2).step_by(bookend_step_by).copied()
                    )
                    .collect()
            },
            4 => {
                let bookend_num_steps = (num_steps - 1) / 2;
                indices.iter().step_by(bookend_step_by).take(bookend_num_steps).copied()
                    .chain(
                        indices.iter().skip(bookend_num_steps * bookend_step_by).step_by(1).take(1).copied()
                    )
                    .chain(
                        indices.iter().skip(bookend_num_steps * bookend_step_by + 1 * 1).step_by(bookend_step_by).copied()
                    )
                    .collect()                    
            },
            5 => {
                let bookend_num_steps = (num_steps - 1) / 2;
                indices.iter().skip(1).step_by(bookend_step_by).take(bookend_num_steps).copied()
                    .chain(
                        indices.iter().skip(1 + bookend_num_steps * bookend_step_by).step_by(2).take(1).copied()
                    )
                    .chain(
                        indices.iter().skip(1 + bookend_num_steps * bookend_step_by + 2 * 1).step_by(bookend_step_by).copied()
                    )
                    .collect()                    
            },
            _ => unreachable!(),
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(2, Vec::from([1]); "start_num_2")]
        #[test_case(3, Vec::from([3]); "start_num_3")]
        #[test_case(4, Vec::from([1, 2]); "start_num_4")]
        #[test_case(5, Vec::from([2, 4]); "start_num_5")]
        #[test_case(6, Vec::from([3, 6]); "start_num_6")]
        #[test_case(7, Vec::from([1, 3, 5]); "start_num_7")]
        #[test_case(8, Vec::from([2, 4, 7]); "start_num_8")]
        #[test_case(9, Vec::from([3, 6, 9]); "start_num_9")]
        #[test_case(10, Vec::from([1, 4, 5, 8]); "start_num_10")]
        #[test_case(11, Vec::from([2, 5, 7, 10]); "start_num_11")]
        #[test_case(12, Vec::from([3, 6, 9, 12]); "start_num_12")]
        #[test_case(13, Vec::from([1, 4, 6, 8, 11]); "start_num_13")]
        #[test_case(14, Vec::from([2, 5, 7, 10, 13]); "start_num_14")]
        #[test_case(15, Vec::from([3, 6, 9, 12, 15]); "start_num_15")]
        #[test_case(16, Vec::from([1, 4, 7, 8, 11, 14]); "start_num_16")]
        #[test_case(17, Vec::from([2, 5, 8, 10, 13, 16]); "start_num_17")]
        #[test_case(18, Vec::from([3, 6, 9, 12, 15, 18]); "start_num_18")]
        #[test_case(19, Vec::from([1, 4, 7, 9, 11, 14, 17]); "start_num_19")]
        #[test_case(20, Vec::from([2, 5, 8, 10, 13, 16, 19]); "start_num_20")]
        fn one_round_is_correct(start_num: usize, answer: Vec<usize>) {
            assert_eq!(one_round((1..=start_num).collect()), answer);
        }

        #[test_case(1, Answer::Usize(2); "example_1")]
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
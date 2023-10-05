#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 9 };

mod utils {
    use unicode_segmentation::UnicodeSegmentation;
    use crate::utils::{io_utils, solution::Solution};

    pub enum Mode {
        Group,
        Garbage,
        Ignore,
    }

    impl Default for Mode {
        fn default() -> Self { Mode::Group }
    }

    pub trait Year2017Day09Solution {
        fn get_mode(&self) -> &Mode;
        fn set_mode(&mut self, mode: Mode);
        fn increment_depth(&mut self);
        fn decrement_depth(&mut self);
        fn increase_sum_of_depths(&mut self);
        fn increment_garbage_count(&mut self);
    }

    pub fn parse_input_file<T>(
        soln: &mut T,       
        filename: &str,
    )
    where
        T: Solution + Year2017Day09Solution
    {
        io_utils::file_to_string(filename)
            .graphemes(true)
            .for_each(|grapheme| {
                match soln.get_mode() {
                    Mode::Group => {
                        match grapheme {
                            "<" => soln.set_mode(Mode::Garbage),
                            "{" => soln.increment_depth(),
                            "}" => {
                                soln.increase_sum_of_depths();
                                soln.decrement_depth();
                            },
                            _ => (),
                        }
                    },
                    Mode::Garbage => {
                        match grapheme {
                            ">" => soln.set_mode(Mode::Group),
                            "!" => soln.set_mode(Mode::Ignore),
                            _   => soln.increment_garbage_count(),
                        };
                    },
                    Mode::Ignore => {
                        soln.set_mode(Mode::Garbage);
                    },
                }
            });
    }
}

pub mod part_one {
    use crate::utils::solution::{Solution, Answer};
    use super::utils::{self, Mode, Year2017Day09Solution};

    #[derive(Default)]
    pub struct Soln {
        mode: Mode,
        depth: u32,
        sum_of_depths: u32,
    }
 
    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            utils::parse_input_file(
                self,
                filename,
            );
            Answer::U32(self.sum_of_depths)
        }
    }

    impl Year2017Day09Solution for Soln {
        fn get_mode(&self) -> &Mode {
            &self.mode
        }
        fn set_mode(&mut self, mode: Mode) {
            self.mode = mode;
        }
        fn decrement_depth(&mut self) {
            self.depth -= 1;
        }
        fn increment_depth(&mut self) {
            self.depth += 1;
        }
        fn increase_sum_of_depths(&mut self) {
            self.sum_of_depths += self.depth;
        }
        fn increment_garbage_count(&mut self) {}
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(1); "example_1")]
        #[test_case(2, Answer::U32(6); "example_2")]
        #[test_case(3, Answer::U32(5); "example_3")]
        #[test_case(4, Answer::U32(16); "example_4")]
        #[test_case(5, Answer::U32(1); "example_5")]
        #[test_case(6, Answer::U32(9); "example_6")]
        #[test_case(7, Answer::U32(9); "example_7")]
        #[test_case(8, Answer::U32(3); "example_8")]
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
    use crate::utils::solution::{Solution, Answer};
    use super::utils::{self, Mode, Year2017Day09Solution};

    #[derive(Default)]
    pub struct Soln {
        mode: Mode,
        depth: u32,
        sum_of_depths: u32,
        garbage_count: u32,
    }
 
    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            utils::parse_input_file(
                self,
                filename,
            );
            Answer::U32(self.garbage_count)
        }
    }

    impl Year2017Day09Solution for Soln {
        fn get_mode(&self) -> &Mode {
            &self.mode
        }
        fn set_mode(&mut self, mode: Mode) {
            self.mode = mode;
        }
        fn decrement_depth(&mut self) {
            self.depth -= 1;
        }
        fn increment_depth(&mut self) {
            self.depth += 1;
        }
        fn increase_sum_of_depths(&mut self) {
            self.sum_of_depths += self.depth;
        }
        fn increment_garbage_count(&mut self) {
            self.garbage_count += 1;
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(9, Answer::U32(0); "example_9")]
        #[test_case(10, Answer::U32(17); "example_10")]
        #[test_case(11, Answer::U32(3); "example_11")]
        #[test_case(12, Answer::U32(2); "example_12")]
        #[test_case(13, Answer::U32(0); "example_13")]
        #[test_case(14, Answer::U32(0); "example_14")]
        #[test_case(15, Answer::U32(10); "example_15")]
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

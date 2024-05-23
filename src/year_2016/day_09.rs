#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 9 };

pub mod part_one {
    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug)]
    enum Mode {
        Pushing,
        ReadingMarker,
        Repeating(usize, usize),
    }

    struct Decompressor {
        compressed: String,
    }

    impl Decompressor {
        fn new(compressed: &str) -> Self {
            Self {
                compressed: compressed.to_string(),
            }
        }

        fn decompress(&self) -> usize {
            let marker_re = Regex::new(r"\((?<len>\d+)x(?<repeats>\d+)\).*").unwrap();
            let mut idx: usize = 0;
            let mut mode = Mode::Pushing;
            let mut decompressed = String::new();
            while idx < self.compressed.len() {
                match mode {
                    Mode::Pushing => {
                        if self.compressed.as_bytes()[idx] as char == '(' { mode = Mode::ReadingMarker; }
                        else {
                            decompressed.push(self.compressed.as_bytes()[idx] as char);
                            idx += 1;
                        }
                    },
                    Mode::ReadingMarker => {
                        let marker = marker_re.captures(&self.compressed[idx..]).unwrap();
                        let len = marker.name("len").unwrap().as_str();
                        let repeats = marker.name("repeats").unwrap().as_str();
                        let marker_len = len.len() + repeats.len() + "(x)".len();
                        let len: usize = len.parse().unwrap();
                        let repeats: usize = repeats.parse().unwrap();
                        mode = Mode::Repeating(len, repeats);
                        idx += marker_len;
                    },
                    Mode::Repeating(len, repeats) => {
                        let repeated = &self.compressed[idx..idx + len];
                        for _ in 0..repeats {
                            decompressed.push_str(repeated);
                        }
                        mode = Mode::Pushing;
                        idx += len;
                    }
                }
            }
            decompressed.len()
        }
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        length: usize,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::Usize(self.length)
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let decompressor = Decompressor::new(&io_utils::file_to_string(filename));
            self.length = decompressor.decompress();
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(6); "example_1")]
        #[test_case(2, Answer::Usize(7); "example_2")]
        #[test_case(3, Answer::Usize(9); "example_3")]
        #[test_case(4, Answer::Usize(11); "example_4")]
        #[test_case(5, Answer::Usize(6); "example_5")]
        #[test_case(6, Answer::Usize(18); "example_6")]
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
    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    fn decompressed_len(re: &Regex, input: &str) -> usize {
        if let Some(captures) = re.captures(input) {
            let len = captures.name("len").unwrap();
            let before = len.start() - 1;
            let repeats = captures.name("repeats").unwrap();
            let after_idx = repeats.end() + 1;
            let len: usize = len.as_str().parse().unwrap();
            let repeats: usize = repeats.as_str().parse().unwrap();
            before + repeats * decompressed_len(re,&input[after_idx..after_idx + len]) + decompressed_len(re, &input[after_idx + len..])
        } else {
            input.len()
        }
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        length: usize,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::Usize(self.length)
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let re = Regex::new(r"\((?<len>\d+)x(?<repeats>\d+)\)").unwrap();
            self.length = decompressed_len(&re, &io_utils::file_to_string(filename));
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;
        #[test_case(3, Answer::Usize(9); "example_3")]
        #[test_case(6, Answer::Usize(20); "example_6")]
        #[test_case(7, Answer::Usize(241_920); "example_7")]
        #[test_case(8, Answer::Usize(445); "example_8")]
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
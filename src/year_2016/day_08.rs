#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 8 };

mod utils {
    use std::fmt::Display;

    use ndarray::{s, Array2, Axis};
    use regex::Regex;

    #[derive(Debug)]
    pub struct Screen {
        data: Array2<u32>,
        operation_re: Regex,
        rect_parameters_re: Regex,
        rotate_parameters_re: Regex,
    }

    impl Default for Screen {
        fn default() -> Self {
            Self {
                data: Array2::zeros((6, 50)),
                operation_re: Regex::new(r"(?<operation>(rect)|(rotate)) (?<parameters>.+)").unwrap(),
                rect_parameters_re: Regex::new(r"(?<cols>\d+)x(?<rows>\d+)").unwrap(),
                rotate_parameters_re: Regex::new(r"(?<axis>(row)|(column)) [xy]\=(?<idx>\d+) by (?<by>\d+)").unwrap(),
            }
        }
    }

    impl Display for Screen {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for row in self.data.axis_iter(Axis(0)) {
                write!(f, "{}\n", row.iter().map(|val| {
                    match val {
                        0 => ' ',
                        1 => '#',
                        _ => panic!("Unrecognized value"),
                    }
                }).collect::<String>())?;
            }
            Ok(())
        }
    }

    impl Screen {
        pub fn handle_instruction(&mut self, instruction: &str) {
            let operation = self.operation_re.captures(instruction).unwrap();
            let parameters = operation.name("parameters").unwrap().as_str();
            match operation.name("operation").unwrap().as_str() {
                "rect" => {
                    let parameters = self.rect_parameters_re.captures(parameters).unwrap();
                    let cols: usize = parameters.name("cols").unwrap().as_str().parse().unwrap();
                    let rows: usize = parameters.name("rows").unwrap().as_str().parse().unwrap();
                    self.rect(cols, rows);
                },
                "rotate" => {
                    let parameters = self.rotate_parameters_re.captures(parameters).unwrap();
                    let axis = parameters.name("axis").unwrap().as_str();
                    let idx: usize = parameters.name("idx").unwrap().as_str().parse().unwrap();
                    let by: usize = parameters.name("by").unwrap().as_str().parse().unwrap();
                    self.rotate(axis, idx, by);
                },
                _ => panic!("Unrecognized operation"),
            }
        }

        fn rect(&mut self, cols: usize, rows: usize) {
            self.data.slice_mut(s![..rows, ..cols]).fill(1);
        }

        fn rotate(&mut self, axis: &str, idx: usize, by: usize) {
            let mut slice = match axis {
                "row" => self.data.slice_mut(s![idx, ..]),
                "column" => self.data.slice_mut(s![.., idx]),
                _ => panic!("Unrecognized axis"),
            };
            let new = ndarray::concatenate(
                Axis(0),
                &[
                    slice.slice(s![slice.len() - by..]),
                    slice.slice(s![..slice.len() - by]),
                ],
            ).unwrap();
            slice.assign(&new);
        }

        pub fn lit_pixels(&self) -> u32 {
            self.data.sum()
        }
    }
}

pub mod part_one {
    use crate::utils::{io_utils, solution::{Answer, Solution}};
    use super::utils::Screen;

    #[derive(Debug, Default)]
    pub struct Soln {
        screen: Screen,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.screen.lit_pixels())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            io_utils::file_to_lines(filename).for_each(|line| self.screen.handle_instruction(&line));
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(6); "example_1")]
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
    use super::utils::Screen;

    #[derive(Debug, Default)]
    pub struct Soln {
        screen: Screen,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::String(format!("{}", self.screen))
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            io_utils::file_to_lines(filename).for_each(|line| self.screen.handle_instruction(&line));
        }
    }
}
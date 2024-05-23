#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 8 };

pub mod part_one {
    use ndarray::{s, Array2, Axis};
    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug)]
    struct Screen {
        data: Array2<u32>,
    }

    impl Default for Screen {
        fn default() -> Self {
            Self {
                data: Array2::zeros((6, 50)),
            }
        }
    }

    impl Screen {
        fn rect(&mut self, cols: usize, rows: usize) {
            self.data.slice_mut(s![..rows, ..cols]).fill(1);
        }

        fn rotate_row(&mut self, idx: usize, by: usize) {
            let mut row = self.data.slice_mut(s![idx, ..]);
            let new = ndarray::concatenate(
                Axis(0),
                &[
                    row.slice(s![row.len() - by..]),
                    row.slice(s![..row.len() - by]),
                ],
            ).unwrap();
            row.assign(&new);
        }

        fn rotate_col(&mut self, idx: usize, by: usize) {
            let mut col = self.data.slice_mut(s![.., idx]);
            let new = ndarray::concatenate(
                Axis(0),
                &[
                    col.slice(s![col.len() - by..]),
                    col.slice(s![..col.len() - by]),
                ],
            ).unwrap();
            col.assign(&new);
        }

        fn lit_pixels(&self) -> u32 {
            self.data.sum()
        }
    }

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
            let operation_re = Regex::new(r"(?<operation>(rect)|(rotate)) (?<parameters>.+)").unwrap();
            let rect_parameters_re = Regex::new(r"(?<cols>\d+)x(?<rows>\d+)").unwrap();
            let rotate_parameters_re = Regex::new(r"(?<axis>(row)|(column)) [xy]\=(?<idx>\d+) by (?<by>\d+)").unwrap();
            for line in io_utils::file_to_lines(filename) {
                let operation = operation_re.captures(&line).unwrap();
                let parameters = operation.name("parameters").unwrap().as_str();
                match operation.name("operation").unwrap().as_str() {
                    "rect" => {
                        let parameters = rect_parameters_re.captures(parameters).unwrap();
                        let cols: usize = parameters.name("cols").unwrap().as_str().parse().unwrap();
                        let rows: usize = parameters.name("rows").unwrap().as_str().parse().unwrap();
                        self.screen.rect(cols, rows);
                    },
                    "rotate" => {
                        let parameters = rotate_parameters_re.captures(parameters).unwrap();
                        let axis = parameters.name("axis").unwrap().as_str();
                        let idx: usize = parameters.name("idx").unwrap().as_str().parse().unwrap();
                        let by: usize = parameters.name("by").unwrap().as_str().parse().unwrap();
                        match axis {
                            "row" => self.screen.rotate_row(idx, by),
                            "column" => self.screen.rotate_col(idx, by),
                            _ => panic!("Unrecognized axis"),
                        }
                    },
                    _ => panic!("Unrecognized operation"),
                }
            }
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
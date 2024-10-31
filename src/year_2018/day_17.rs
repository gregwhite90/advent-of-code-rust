#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 17 };

/**
 * At each y, I need to know all the x's that are clay, approximately in order?
 * 
 * Track the horizontals and the verticals?
 * 
 * Then when hits, it goes left and right simultaneously. Once both are stopped,
 * it becomes settled water.
 * 
 * Save a map of y to all x's.
 */

mod utils {
    struct Point {
        x: usize,
        y: usize,
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    enum Orientation {
        Horizontal,
        Vertical,
    }

    struct Range {
        orientation: Orientation,
        min: usize,
        max: usize,
    }

    pub struct Reservoir {
        spring: Point,
        min_y: usize,
        max_y: usize,
    }

    // TODO: should spring be dynamic?
    impl Default for Reservoir {
        fn default() -> Self {
            Self {
                spring: Point { x: 500, y: 0 },
                min_y: 0,
                max_y: 0,
            }
        }
    }
}

pub mod part_one {
    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    use super::utils::Operation;

    #[derive(Debug, Default)]
    pub struct Soln {}

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(57); "example_1")]
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

    use super::utils::CPU;

    #[derive(Debug, Default)]
    pub struct Soln {
        cpu: CPU,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            let mut before = None;
            let mut operation: Option<Vec<usize>> = None;
            let mut after = None;
            let registers_re = Regex::new(r"(?<seq>(Before)|(After)):\s+\[(?<registers>[\d\, ]+)\]").unwrap();
            io_utils::file_to_lines(filename).for_each(|line| {
                if line.len() == 0 {
                    if let Some(af) = &after {
                        assert_ne!(before, None);
                        // NOTE: changed from part one
                        self.cpu.process_sample(
                            &before.as_ref().unwrap(),
                            operation.as_ref().unwrap()[0],
                            operation.as_ref().unwrap()[1], 
                            operation.as_ref().unwrap()[2], 
                            operation.as_ref().unwrap()[3], 
                            &af
                        );
                        before = None;
                        operation = None;
                        after = None;                        
                    }
                } else if let Some(captures) = registers_re.captures(&line) {
                    let seq = captures.name("seq").unwrap().as_str();
                    let registers = captures.name("registers").unwrap().as_str().split(", ")
                        .map(|val| val.parse().unwrap())
                        .collect();
                    match seq {
                        "Before" => before = Some(registers),
                        "After"  => after  = Some(registers),
                        _ => panic!("Unrecognized sequence"),
                    }
                } else {
                    let op = line.split(" ").map(|val| val.parse().unwrap()).collect();
                    if before == None {
                        // perform operation
                        self.cpu.perform_operation(op);
                    } else {
                        operation = Some(op);
                    }
                }
            });
            Answer::Usize(self.cpu.register_value(0))
        }
    }
}
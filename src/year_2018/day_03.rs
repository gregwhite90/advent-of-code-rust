#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 3 };

pub mod part_one {
    use std::collections::HashMap;

    use itertools::iproduct;
    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    struct Point {
        x: usize,
        y: usize,
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        claims: HashMap<Point, usize>,    
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::Usize(self.points_with_multiple_claims())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let line_re = Regex::new(r"#(?<id>\d+) \@ (?<x>\d+),(?<y>\d+)\: (?<width>\d+)x(?<height>\d+)").unwrap();
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    let captures = line_re.captures(&line).unwrap();
                    let x: usize = captures.name("x").unwrap().as_str().parse().unwrap();
                    let y: usize = captures.name("y").unwrap().as_str().parse().unwrap();
                    let width: usize = captures.name("width").unwrap().as_str().parse().unwrap();
                    let height: usize = captures.name("height").unwrap().as_str().parse().unwrap();
                    for (col, row) in iproduct!(x..x + width, y..y + height) {
                        let point = Point {x: col, y: row };
                        *self.claims.entry(point).or_default() += 1;
                    }
                });
        }   

        fn points_with_multiple_claims(&self) -> usize {
            self.claims.values().filter(|count| **count > 1).count()
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(4); "example_1")]
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
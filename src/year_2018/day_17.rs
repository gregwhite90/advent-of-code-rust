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
    use std::{cmp, collections::HashMap};

    use regex::Regex;

    use crate::utils::io_utils;

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    struct Point {
        x: usize,
        y: usize,
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    enum SquareType {
        Spring,
        Sand,
        Clay,
        FlowingWater,
        RestingWater,
    }

    #[derive(Debug)]
    pub struct Reservoir {
        spring: Point,
        min_y: usize,
        max_y: usize,
        squares: HashMap<Point, SquareType>,
    }

    impl Default for Reservoir {
        fn default() -> Self {
            let spring = Point { x: 500, y: 0 };
            Self {
                spring,
                min_y: usize::MAX,
                max_y: usize::MIN,
                squares: HashMap::from([(spring, SquareType::Spring)]),
            }
        }
    }

    impl Reservoir {
        pub fn parse_input_file(&mut self, filename: &str) {
            let line_re = Regex::new(r"(?<single_axis>[xy])=(?<single_axis_value>\d+), (?<range_axis>[xy])=(?<range_min>\d+)\.\.(?<range_max>\d+)").unwrap();
            io_utils::file_to_lines(filename).for_each(|line| {
                let caps = line_re.captures(&line).unwrap();
                let single_axis = caps.name("single_axis").unwrap().as_str();
                let single_axis_value: usize = caps.name("single_axis_value").unwrap().as_str().parse().unwrap();
                let range_axis = caps.name("range_axis").unwrap().as_str();
                let range_min: usize = caps.name("range_min").unwrap().as_str().parse().unwrap();
                let range_max: usize = caps.name("range_max").unwrap().as_str().parse().unwrap();               
                match single_axis {
                    "x" => {
                        assert_eq!(range_axis, "y");
                        self.min_y = cmp::min(self.min_y, range_min);
                        self.max_y = cmp::max(self.max_y, range_max);
                        for y in range_min..=range_max {
                            self.squares.insert(
                                Point { x: single_axis_value, y },
                                SquareType::Clay,
                            );
                        }                                
                    },
                    "y" => {
                        assert_eq!(range_axis, "x");
                        self.min_y = cmp::min(self.min_y, single_axis_value);
                        self.max_y = cmp::max(self.max_y, single_axis_value);
                        for x in range_min..=range_max {
                            self.squares.insert(
                                Point { x, y: single_axis_value },
                                SquareType::Clay,
                            );                            
                        }                                
                    },
                    _ => panic!("Unrecognized single axis,"),
                }
            });
        }
    }
}

/**
 * Add to the queue to process
 *      If sand: keep going down
 *      If clay or still water: go outward in both directions from the one above.
 *              This is a special case
 *      If 
 */

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::Reservoir;

    #[derive(Debug, Default)]
    pub struct Soln {
        reservoir: Reservoir,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.reservoir.parse_input_file(filename);
            // TODO: actually do
            Answer::Usize(0)
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
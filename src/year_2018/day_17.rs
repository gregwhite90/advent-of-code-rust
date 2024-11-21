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
    use std::{cmp, collections::{BTreeSet, HashMap, HashSet}};

    use regex::Regex;

    use crate::utils::io_utils;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    struct Point {
        x: usize,
        y: usize,
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    enum Direction {
        Left,
        Right,
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

        pub fn flow_water(&mut self) {
            let mut to_process = BTreeSet::from([self.spring]);
            while !to_process.is_empty() {
                let pt = to_process.pop_first().unwrap();
                // println!("{:?}", to_process);
                // println!("{:?}", pt);
                // println!("{}", self);
                match self.squares.get(&pt) {
                    None | Some(SquareType::FlowingWater) | Some(SquareType::Spring) => (),
                    Some(SquareType::RestingWater) => {
                        to_process.insert(Point { x: pt.x, y: pt.y - 1 });
                        continue;
                    },
                    Some(SquareType::Clay) => {
                        panic!("Should not be processing a clay square");
                    },
                    Some(SquareType::Sand) => {
                        panic!("Should not explicitly be marking squares as sand");
                    }
                }
                let down_pt = Point { x: pt.x, y: pt.y + 1 };
                match self.squares.get(&down_pt) {
                    None | Some(SquareType::FlowingWater) => {
                        self.squares.insert(down_pt, SquareType::FlowingWater);
                        if pt.y + 1 <= self.max_y {
                            to_process.insert(down_pt);
                        }
                    },
                    Some(SquareType::Clay) | Some(SquareType::RestingWater) => {
                        let (left_res, left_pts) = self.flow_water_horizontally(&pt, &Direction::Left);
                        let (right_res, right_pts) = self.flow_water_horizontally(&pt, &Direction::Right);
                        match (left_res, right_res) {
                            (None, None) => {
                                // Water has come to rest on both sides
                                for pt in left_pts.into_iter() {
                                    self.squares.insert(pt, SquareType::RestingWater);
                                }
                                for pt in right_pts.into_iter() {
                                    self.squares.insert(pt, SquareType::RestingWater);
                                }
                                // Go back up a level if this has resulted in a new layer.
                                to_process.insert(Point { x: pt.x, y: pt.y - 1 });
                            },
                            (_, _) => {
                                for pt in left_pts.into_iter() {
                                    self.squares.insert(pt, SquareType::FlowingWater);
                                }
                                for pt in right_pts.into_iter() {
                                    self.squares.insert(pt, SquareType::FlowingWater);
                                }
                                // Add the overflow points for processing
                                if let Some(left_next_pt) = left_res {
                                    to_process.insert(left_next_pt);
                                }
                                if let Some(right_next_pt) = right_res {
                                    to_process.insert(right_next_pt);
                                }
                            },
                        }
                    },
                    Some(SquareType::Sand) => {
                        panic!("Should not be explicitly storing any squares as sand");
                    },
                    Some(SquareType::Spring) => {
                        panic!("Water should not be above spring");
                    }
                }
            }
        }

        /// Returns a tuple of:
        ///     - An `Option` of a `Point``. This will be Some if the water should begin flowing
        ///         at the contained `Point``. It will be None if the water flowed to a
        ///         resting point.
        ///     - A `HashSet` of `Point`s that should become water (flowing or resting)
        ///         once both directions are processed
        fn flow_water_horizontally(
            &self,
            starting_point: &Point,
            direction: &Direction,
        ) -> (Option<Point>, HashSet<Point>) {
            let next_pt = match direction {
                Direction::Left  => Point { x: starting_point.x - 1, y: starting_point.y },
                Direction::Right => Point { x: starting_point.x + 1, y: starting_point.y },
            };
            match self.squares.get(&next_pt) {
                None | Some(SquareType::Sand) | Some(SquareType::FlowingWater) => {
                    let next_pt_support = Point { x: next_pt.x, y: next_pt.y + 1 };
                    match self.squares.get(&next_pt_support) {
                        None | Some(SquareType::FlowingWater) | Some(SquareType::Sand) => {
                            return (Some(next_pt), HashSet::from([*starting_point, next_pt]));
                        },
                        Some(SquareType::Clay) | Some(SquareType::RestingWater) => {
                            // Continue flowing.
                            let (res, mut points) = self.flow_water_horizontally(&next_pt, direction);
                            points.insert(*starting_point);
                            return (res, points);
                        },
                        Some(SquareType::Spring) => {
                            panic!("Spring should not be supporting any flowing water");
                        },
                    }
                },
                Some(SquareType::Clay) => {
                    return (None, HashSet::from([*starting_point]));
                },
                /*
                Some(SquareType::FlowingWater) => {
                    // TODO: confirm this is the desired treatment
                    let (res, mut points) = self.flow_water_horizontally(&next_pt, direction);
                    points.insert(*starting_point);
                    return (res, points);
                },*/
                Some(SquareType::RestingWater) => {
                    panic!("Should not be possible to flow horizontally into resting water");
                },
                Some(SquareType::Spring) => {
                    panic!("Should not be possible to flow horizontally into spring");
                }
            }
        }

        pub fn squares_reached_by_water(&self) -> usize {
            self.squares.iter().filter(|(pt, square_type)| {
                (**square_type == SquareType::Spring || **square_type == SquareType::FlowingWater || **square_type == SquareType::RestingWater)
                    && pt.y >= self.min_y
                    && pt.y <= self.max_y
            }).count()
        }
    }

    impl std::fmt::Display for Reservoir {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for y in 0..=250 {
                for x in 440..=560 {
                    let ch = match self.squares.get(&Point { x, y }) {
                        Some(SquareType::Spring) => '+',
                        None | Some(SquareType::Sand) => '.',
                        Some(SquareType::Clay) => '#',
                        Some(SquareType::RestingWater) => '~',
                        Some(SquareType::FlowingWater) => '|',
                    };
                    write!(f, "{}", ch)?;
                }
                write!(f, "\n")?;
            }
            Ok(())
        }
    }
}

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
            self.reservoir.flow_water();
            Answer::Usize(self.reservoir.squares_reached_by_water())
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
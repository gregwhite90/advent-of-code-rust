#[cfg(test)]
const YEAR: u32 = 2017;
#[cfg(test)]
const DAY: u8 = 3;

pub mod utils {
    use std::fs;

    pub fn parse_input_file(filename: &str) -> u32 {
        fs::read_to_string(filename)
            .expect("Should be able to read the file to a string.")
            .parse::<u32>()
            .expect("File should be a single unsigned integer.")
    }
    
    #[cfg(test)]
    mod tests {
        use std::collections::HashMap;
        use crate::utils::utils::{InputFileType, input_filename};
        use super::*;    
        use super::super::{YEAR, DAY};

        #[test]
        fn parse_input_file_is_correct() {
            let cases = HashMap::from([
                (1u8, 1u32),
                (2, 12),
                (3, 23),
                (4, 1024),
            ]);
            for (&example_key, &input_value) in &cases {
                assert_eq!(
                    parse_input_file(&input_filename(YEAR, DAY, InputFileType::Example(example_key))),
                    input_value
                );
            }
        }
    }
}

pub mod part_one {
    pub use either::*;
    use crate::utils::utils::Solution;
    use super::utils;

    #[derive(Default)]
    pub struct Soln {
        num: u32,
    }
 
    impl Solution for Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.num = utils::parse_input_file(filename);
        }

        fn solve(&mut self) -> Either<i32, &str> {
            let sqrt = (self.num as f64).sqrt().ceil() as u32;
            let shortest_distance_from_layer = sqrt / 2;
            let step_shortest_dist_multiplier = 2;
            let num_steps = 4;
            let dist_within_layer = if shortest_distance_from_layer > 0 {
                (shortest_distance_from_layer..(
                    shortest_distance_from_layer + 
                    shortest_distance_from_layer * step_shortest_dist_multiplier * (num_steps - 1) + 1
                ))
                    .step_by(
                        (shortest_distance_from_layer * step_shortest_dist_multiplier)
                            .try_into()
                            .unwrap()
                    )
                    .map(|cross_point| {
                        (cross_point as i32 - (
                            self.num - if sqrt % 2 == 0 { sqrt - 1 } else { sqrt - 2 }.pow(2)
                        ) as i32).abs() as u32
                    })
                    .min()
                    .expect("Should be at least one cross point to find the distance to.")
                } else { 
                    0
                };
            Left(
                (shortest_distance_from_layer + dist_within_layer) as i32
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use std::collections::HashMap;
        use crate::utils::utils::{InputFileType, input_filename};
        use super::*;
        use super::super::{YEAR, DAY};

        #[test]
        fn examples_are_correct() {
            let cases = HashMap::from([
                (1u8, 0),
                (2, 3),
                (3, 2),
                (4, 31),
            ]);
            for (&example_key, &answer) in &cases {
                let mut soln = Soln::default();
                soln.parse_input_file(&input_filename(YEAR, DAY, InputFileType::Example(example_key)));
                assert_eq!(
                    soln.solve().expect_left("Solution should be an integer."),
                    answer
                );
            }
        }
    }    
}
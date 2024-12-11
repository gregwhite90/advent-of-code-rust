#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2024, day: 11 };

mod utils {
    use std::collections::HashMap;

    pub fn num_stones(value: usize, blinks: usize, mut cache: &mut HashMap<(usize, usize), usize>) -> usize {
        if blinks == 0 {
            return 1;
        }
        if let Some(cached) = cache.get(&(value, blinks)) {
            return *cached;
        }
        let res = if value == 0 {
            num_stones(1, blinks - 1, &mut cache)
        } else if value.to_string().len() % 2 == 0 {
            let val_string = value.to_string();
            let left = val_string[..val_string.len() / 2].parse().unwrap();
            let right = val_string[val_string.len() / 2..].parse().unwrap();
            num_stones(left, blinks - 1, &mut cache) + num_stones(right, blinks - 1, &mut cache)
        } else {
            num_stones(value * 2024, blinks - 1, &mut cache)
        };
        cache.insert((value, blinks), res);
        res
    }
}

pub mod part_one {
    use std::collections::HashMap;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    use super::utils;

    #[derive(Debug)]
    pub struct Soln {
        stones: Vec<usize>,
        blinks: usize,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self::with_blinks(25)
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
            Answer::Usize(
                self.stones.iter()
                    .map(|value| utils::num_stones(*value, self.blinks, &mut cache))
                    .sum()
            )
        }
    }

    impl Soln {
        fn with_blinks(blinks: usize) -> Self {
            Self { 
                stones: Vec::default(),
                blinks,
            }
        }

        fn parse_input_file(&mut self, filename: &str) {
            self.stones = io_utils::file_to_string(filename)
                .split(' ')
                .map(|v| v.parse().unwrap())
                .collect()
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, 1, Answer::Usize(7); "example_1_1_blink")]
        #[test_case(2, 6, Answer::Usize(22); "example_2_6_blinks")]
        #[test_case(2, 25, Answer::Usize(55_312); "example_2_25_blinks")]
        fn examples_are_correct(example_key: u8, blinks: usize, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::with_blinks(blinks),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}

pub mod part_two {
    use std::collections::HashMap;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    use super::utils;

    #[derive(Debug)]
    pub struct Soln {
        stones: Vec<usize>,
        blinks: usize,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self::with_blinks(75)
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
            Answer::Usize(
                self.stones.iter()
                    .map(|value| utils::num_stones(*value, self.blinks, &mut cache))
                    .sum()
            )
        }
    }

    impl Soln {
        fn with_blinks(blinks: usize) -> Self {
            Self { 
                stones: Vec::default(),
                blinks,
            }
        }

        fn parse_input_file(&mut self, filename: &str) {
            self.stones = io_utils::file_to_string(filename)
                .split(' ')
                .map(|v| v.parse().unwrap())
                .collect()
        }
    }
}
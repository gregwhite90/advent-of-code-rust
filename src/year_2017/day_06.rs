#[cfg(test)]
const YEAR: u32 = 2017;
#[cfg(test)]
const DAY: u8 = 6;

pub mod part_one {
    use std::{fs, collections::HashSet};
    pub use either::*;
    use crate::utils::utils::Solution;

    #[derive(Default)]
    pub struct Soln {
        banks: Vec<u32>,
        seen: HashSet<Vec<u32>>,
    }
 
    impl Solution for Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.banks = fs::read_to_string(filename)
                .expect("Should be able to read the file to a string.")
                .split_whitespace()
                .map(|num| num.parse().expect("Each part of the input should be an unsigned integer."))
                .collect();
        }

        fn solve(&mut self) -> Either<i32, &str> {
            let mut steps = 0;
            let num_banks = self.banks.len();
            while !self.seen.contains(&self.banks) {
                self.seen.insert(self.banks.clone());
                steps += 1;
                let max = self.banks.iter().max().expect("There should be at least one bank.");
                let pos = self.banks.iter().position(|&blocks| blocks == *max).expect("The max value should be in at least one bank.");
                let blocks = self.banks[pos];
                self.banks[pos] = 0;
                for offset in 1..(blocks + 1) {
                    self.banks[(pos + offset as usize) % num_banks] += 1;
                }                
            }
            Left(steps)
        }
    }

    #[cfg(test)]
    mod tests {
        use std::collections::HashMap;
        use either::*;
        use crate::utils::test_utils;
        use super::*;
        use super::super::{YEAR, DAY};

        #[test]
        fn examples_are_correct() {
            test_utils::check_example_cases(
                &mut Soln::default(),
                &HashMap::from([
                    (1u8, Left(5)),
                ]),
                YEAR,
                DAY,
            );
        }
    }    
}

pub mod part_two {
    use std::{fs, collections::HashMap};
    pub use either::*;
    use crate::utils::utils::Solution;

    #[derive(Default)]
    pub struct Soln {
        banks: Vec<u32>,
        seen: HashMap<Vec<u32>, u32>,
    }
 
    impl Solution for Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.banks = fs::read_to_string(filename)
                .expect("Should be able to read the file to a string.")
                .split_whitespace()
                .map(|num| num.parse().expect("Each part of the input should be an unsigned integer."))
                .collect();
        }

        fn solve(&mut self) -> Either<i32, &str> {
            let mut steps = 0;
            let num_banks = self.banks.len();
            while !self.seen.contains_key(&self.banks) {
                self.seen.insert(self.banks.clone(), steps);
                steps += 1;
                let max = self.banks.iter().max().expect("There should be at least one bank.");
                let pos = self.banks.iter().position(|&blocks| blocks == *max).expect("The max value should be in at least one bank.");
                let blocks = self.banks[pos];
                self.banks[pos] = 0;
                for offset in 1..(blocks + 1) {
                    self.banks[(pos + offset as usize) % num_banks] += 1;
                }                
            }
            Left(
                (steps - self.seen.get(&self.banks).expect("Current banks should have been seen previously."))
                    .try_into()
                    .expect("Steps should be convertable to an unsigned integer.")
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use std::collections::HashMap;
        use either::*;
        use crate::utils::test_utils;
        use super::*;
        use super::super::{YEAR, DAY};

        #[test]
        fn examples_are_correct() {
            test_utils::check_example_cases(
                &mut Soln::default(),
                &HashMap::from([
                    (1u8, Left(4)),
                ]),
                YEAR,
                DAY,
            );
        }
    }    
}
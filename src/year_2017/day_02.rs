#[cfg(test)]
const YEAR: u32 = 2017;
#[cfg(test)]
const DAY: u8 = 2;

pub mod utils {
    use std::fs;

    pub fn parse_input_file(filename: &str) -> Vec<Vec<i32>> {
        fs::read_to_string(filename)
            .expect("Should be able to read the file to a string.")
            .lines()
            .map(|line| parse_line(line))
            .collect()
    }
    
    fn parse_line(line: &str) -> Vec<i32> {
        line
            .split_whitespace()
            .map(|num| num.parse::<i32>().expect("Input should be all integers."))
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use crate::utils::utils::{InputFileType, input_filename};
        use super::super::{YEAR, DAY};
        use super::*;    

        #[test]
        fn parse_line_is_correct() {
            assert_eq!(parse_line("1    2 3  4 12  \n"), vec![1, 2, 3, 4, 12]);
        }

        #[test]
        fn parse_empty_line_is_correct() {
            assert_eq!(parse_line(""), vec![]);
        }

        #[test]
        #[should_panic]
        fn parse_line_panics() {
            parse_line("1 2 a\n");
        }

        #[test]
        fn parse_input_file_is_correct() {
            assert_eq!(
                parse_input_file(&input_filename(YEAR, DAY, InputFileType::Example(1))),
                vec![
                    vec![5, 1, 9, 5],
                    vec![7, 5, 3],
                    vec![2, 4, 6, 8],
                ]
            );
        }
    }
}

pub mod part_one {
    pub use either::*;
    use crate::utils::utils::Solution;
    use crate::year_2017::day_02::utils;

    #[derive(Default)]
    pub struct Soln {
        nums: Vec<Vec<i32>>,
    }

    impl Solution for Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.nums = utils::parse_input_file(filename);
        }

        fn solve(&mut self) -> Either<i32, String> {
            Left(self.nums
                .iter()
                .map(|row| row_range(row).expect("Row should not be empty."))
                .sum()
            )
        }
    }

    fn row_range(row: &Vec<i32>) -> Option<i32> {
        let max = row.iter().max();
        let min = row.iter().min();
        match (max, min) {
            (Some(max), Some(min)) => Some(max - min),
            _ => None,
        }
    }

    #[cfg(test)]
    mod tests {
        use std::collections::HashMap;
        use either::*;
        use crate::utils::test_utils;
        use super::super::{YEAR, DAY};
        use super::*;    

        #[test]
        fn row_range_is_correct() {
            assert_eq!(row_range(&vec![1, 4, 2, 3]), Some(3));
            assert_eq!(row_range(&vec![1, 1, 1]), Some(0));
        }

        #[test]
        fn row_range_on_empty_is_correct() {
            assert_eq!(row_range(&vec![]), None);
        }
    
        #[test]
        fn example_is_correct() {
            test_utils::check_example_cases(
                &mut Soln::default(),
                &HashMap::from([
                    (1u8, Left(18)),
                ]),
                YEAR,
                DAY,
            );
        }
    }    
}

pub mod part_two {
    pub use either::*;
    use crate::utils::utils::Solution;
    use crate::year_2017::day_02::utils;

    #[derive(Default)]
    pub struct Soln {
        nums: Vec<Vec<i32>>,
    }

    impl Solution for Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.nums = utils::parse_input_file(filename);
        }

        fn solve(&mut self) -> Either<i32, String> {
            Left(self.nums
                .iter()
                .map(|row| row_division(row).expect("Row should have a divisible pair."))
                .sum()
            )
        }
    }

    fn row_division(row: &Vec<i32>) -> Option<i32> {
        for i in 0..row.len() {
            for j in (i + 1)..row.len() {
                if row[i] % row[j] == 0 {
                    return Some(row[i] / row[j])
                } else if row[j] % row[i] == 0 {
                    return Some(row[j] / row[i])
                }
            }
        }
        None
    }

    #[cfg(test)]
    mod tests {
        use std::collections::HashMap;
        use either::*;
        use crate::utils::test_utils;
        use super::*;    
        use super::super::{YEAR, DAY};

        #[test]
        fn example_is_correct() {
            test_utils::check_example_cases(
                &mut Soln::default(),
                &HashMap::from([
                    (2u8, Left(9)),
                ]),
                YEAR,
                DAY,
            );
        }
    }    
}
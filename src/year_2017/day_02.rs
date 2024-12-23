#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 2 };

mod utils {
    use crate::utils::{io_utils, solution::Solution};

    pub trait Year2017Day02Solution {
        fn set_nums(&mut self, nums: Vec<Vec<i32>>);
    }

    pub fn parse_input_file<T>(soln: &mut T, filename: &str)
    where
        T: Solution + Year2017Day02Solution 
    {
        soln.set_nums(
            io_utils::file_to_lines(filename)
                .map(|line| parse_line(&line))
                .collect()
        );
    }
    
    fn parse_line(line: &str) -> Vec<i32> {
        line
            .split_whitespace()
            .map(|num| num.parse::<i32>().expect("Input should be all integers."))
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;    

        #[test]
        fn parse_line_is_correct() {
            assert_eq!(parse_line("1    2 3  4 12  \n"), vec![1, 2, 3, 4, 12]);
        }

        #[test]
        fn parse_empty_line_is_correct() {
            assert_eq!(parse_line(""), Vec::<i32>::new());
        }

        #[test]
        #[should_panic]
        fn parse_line_panics() {
            parse_line("1 2 a\n");
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Solution, Answer};
    use super::utils::{self, Year2017Day02Solution};

    #[derive(Default)]
    pub struct Soln {
        nums: Vec<Vec<i32>>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            utils::parse_input_file(self, filename);
            Answer::I32(self.nums
                .iter()
                .map(|row| row_range(row).expect("Row should not be empty."))
                .sum()
            )
        }
    }

    impl Year2017Day02Solution for Soln {
        fn set_nums(&mut self, nums: Vec<Vec<i32>>) {
            self.nums = nums;
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
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::super::DAY;
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
    
        #[test_case(1, Answer::I32(18); "example_1")]
        fn example_is_correct(example_key: u8, answer: Answer) {
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
    use crate::utils::solution::{Solution, Answer};
    use super::utils::{self, Year2017Day02Solution};

    #[derive(Default)]
    pub struct Soln {
        nums: Vec<Vec<i32>>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            utils::parse_input_file(self, filename);
            Answer::I32(self.nums
                .iter()
                .map(|row| row_division(row).expect("Row should have a divisible pair."))
                .sum()
            )
        }
    }

    impl Year2017Day02Solution for Soln {
        fn set_nums(&mut self, nums: Vec<Vec<i32>>) {
            self.nums = nums;            
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
        use test_case::test_case;
        use crate::utils::test_utils;
        use super::*;    
        use super::super::DAY;

        #[test_case(2, Answer::I32(9); "example_2")]
        fn example_is_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::default(),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}
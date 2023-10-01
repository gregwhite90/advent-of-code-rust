pub mod part_one {
    use std::fs;
    pub use either::*;
    use crate::utils::utils::Solution;

    #[derive(Default)]
    pub struct Soln {
        nums: Vec<Vec<i32>>,
    }

    impl Solution for Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let text = fs::read_to_string(filename)
                .expect("Should be able to read the file to a string.");
            self.nums = text.lines()
                .map(|line| parse_line(line))
                .collect();
        }

        fn solve(&mut self) -> Either<i32, &str> {
            Left(self.nums
                .iter()
                .map(|row| row_range(row).expect("Row should not be empty."))
                .sum()
            )
        }
    }

    fn parse_line(line: &str) -> Vec<i32> {
        line
            .split_whitespace()
            .map(|num| num.parse::<i32>().expect("Input should be all integers."))
            .collect()
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
            let mut soln = Soln::default();
            soln.parse_input_file(
                &format!("input/year_2017/day_02/test_examples/example_1.txt")
            );
            assert_eq!(18, soln.solve().expect_left("Solution should be an integer."));    
        }
    }    
}
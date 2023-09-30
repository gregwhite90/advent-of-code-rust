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
                .map(|line| {
                    line
                        .split_whitespace()
                        .map(|num| num.parse::<i32>().expect("Input should be all integers."))
                        .collect()
                })
                .collect();
        }

        fn solve(&mut self) -> Either<i32, &str> {
            Left(self.nums
                .iter()
                .map(|row| {
                    row.iter().max().unwrap() - row.iter().min().unwrap()
                })
                .sum()
            )
        }
    }    
}

#[cfg(test)]
mod tests {
    mod part_one {
        use crate::utils::utils::Solution;    
        use crate::year_2017::day_02::part_one::Soln;
    
        #[test]
        fn example_is_correct() {
            let mut soln = Soln::default();
            soln.parse_input_file("input/year_2017/day_02/test_examples/example_1.txt");
            assert_eq!(18, soln.solve().expect_left("Solution should be an integer."));
        }
    }
}
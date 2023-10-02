pub mod utils {
    use std::fs;

    pub fn parse_input_file(filename: &str) -> i32 {
        fs::read_to_string(filename)
            .expect("Should be able to read the file to a string.")
            .parse::<i32>()
            .expect("File should be a single unsigned integer.")
    }
    
    #[cfg(test)]
    mod tests {
        use std::collections::HashMap;
        use super::*;    

        #[test]
        fn parse_input_file_is_correct() {
            let cases = HashMap::from([
                (1, 1),
                (2, 12),
                (3, 23),
                (4, 1024),
            ]);
            for (example_key, value) in &cases {
                assert_eq!(
                    parse_input_file(&format!("input/year_2017/day_03/test_examples/example_{}.txt", example_key)),
                    *value
                );
            }
        }
    }
}

pub mod part_one {
    pub use either::*;
    use crate::utils::utils::Solution;
    use crate::year_2017::day_03::utils;

    #[derive(Default)]
    pub struct Soln {
    }

    impl Solution for Soln {
        fn parse_input_file(&mut self, filename: &str) {
        }

        fn solve(&mut self) -> Either<i32, &str> {
            Left(0)
        }
    }

    #[cfg(test)]
    mod tests {
    }    
}
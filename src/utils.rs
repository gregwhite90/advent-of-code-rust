pub mod utils {
    use either::*;
    // Solution implements solve, parse input filename
    pub trait Solution {
        fn parse_input_file(&mut self, filename: &str);
        fn solve(&mut self) -> Either<i32, String>; // TODO: possibly use Box<dyn Display + PartialEq + Eq> instead of Either?
    }

    pub enum InputFileType {
        Input,
        #[allow(dead_code)] Example(u8), // constructed only in test cases
    }

    pub fn input_filename(year: u32, day: u8, input_file_type: InputFileType) -> String {
        let file = match input_file_type {
            InputFileType::Input => String::from("input.txt"),
            InputFileType::Example(example_key) => format!("test_examples/example_{example_key}.txt"),
        };
        format!("input/year_{year}/day_{day:02}/{file}")
    }
}

#[cfg(test)]
pub mod test_utils {
    use either::*;
    use super::utils::{Solution, InputFileType, input_filename};

    pub fn check_example_case<T: Solution>(
        soln: &mut T,
        example_key: u8,
        answer: Either<i32, String>,
        year: u32,
        day: u8,
    ) {
        soln.parse_input_file(&input_filename(year, day, InputFileType::Example(example_key)));
        match soln.solve() {
            Left(ans) => assert_eq!(
                ans,
                answer.expect_left("Solved answer and example answer should be the same type.")
            ),
            Right(ans) => assert_eq!(
                ans,
                answer.expect_right("Solved answer and example answer should be the same type.").clone() // TODO: confirm this works
            ),
        }
    }

}
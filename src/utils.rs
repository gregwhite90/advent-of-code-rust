pub mod utils {
    use either::*;
    // Solution implements solve, parse input filename
    pub trait Solution {
        fn parse_input_file(&mut self, filename: &str);
        fn solve(&mut self) -> Either<i32, &str>;
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
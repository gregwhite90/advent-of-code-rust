pub struct Day { 
    pub year: u32,
    pub day: u8,
}

pub mod solution {
    use std::fmt;

    #[derive(PartialEq, Eq, Debug)]
    pub enum Answer {
        String(String),
        I32(i32),
        U32(u32),
    }

    impl fmt::Display for Answer {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Answer::String(string) => write!(f, "{}", string),
                Answer::I32(num) => write!(f, "{}", num),
                Answer::U32(num) => write!(f, "{}", num),
            }
        }
    }

    pub trait Solution {
        fn parse_input_file(&mut self, filename: &str);
        fn solve(&mut self) -> Answer;
    }
}

pub mod io_utils {
    use std::fs::{self, File};
    use std::io::{self, BufRead};
    use std::path::Path;
    use super::Day;

    pub enum InputFileType {
        Input,
        #[allow(dead_code)] Example(u8),
    }

    pub fn input_filename(day: &Day, input_file_type: InputFileType) -> String {
        let file = match input_file_type {
            InputFileType::Input => String::from("input.txt"),
            InputFileType::Example(example_key) => format!("test_examples/example_{example_key}.txt"),
        };
        format!("input/year_{}/day_{:02}/{}", day.year, day.day, file)
    }

    pub fn file_to_string(filename: &str) -> String {
        fs::read_to_string(filename)
            .expect("Should be able to read the file to a string.")
    }

    // Returns an iterator of strings over lines in file.
    pub fn file_to_lines(filename: &str) -> impl Iterator<Item = String> {
        read_lines(filename).expect("Should be able to open the file.")
            .map(|line| line.expect("Should be able to read the line."))
    }

    // From [Rust by Example](https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html)
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

}

#[cfg(test)]
pub mod test_utils {
    use super::{solution::{Solution, Answer}, io_utils::{InputFileType, input_filename}, Day};

    pub fn check_example_case<T: Solution>(
        soln: &mut T,
        example_key: u8,
        answer: Answer,
        day: &Day,
    ) {
        soln.parse_input_file(&input_filename(day, InputFileType::Example(example_key)));
        assert_eq!(
            soln.solve(),
            answer,
        );
    }
}
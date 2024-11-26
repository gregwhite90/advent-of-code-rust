//! A collection of utilities used by all or nearly all solutions.

/// Specifies a day for a solution.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Day { 
    pub year: u32,
    pub day: u8,
}

pub mod solution {
    //! A collection of solution-oriented utilities.
    use std::fmt;

    /// The possible types of an answer to a solution.
    #[derive(PartialEq, Eq, Debug)]
    pub enum Answer {
        String(String),
        I32(i32),
        U32(u32),
        U16(u16),
        I64(i64),
        U64(u64),
        Usize(usize),
    }

    impl fmt::Display for Answer {
        /// Simple conversion to a display format depending on the answer type.
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Answer::String(string) => write!(f, "{}", string),
                Answer::I32(num) => write!(f, "{}", num),
                Answer::U32(num) => write!(f, "{}", num),
                Answer::U16(num) => write!(f, "{}", num),
                Answer::I64(num) => write!(f, "{}", num),
                Answer::U64(num) => write!(f, "{}", num),
                Answer::Usize(num) => write!(f, "{}", num),
            }
        }
    }

    /// Methods implemented by all solutions.
    pub trait Solution {
        /// Solves the puzzle given a path to an input file.
        fn solve(&mut self, filename: &str) -> Answer;
    }
}

pub mod io_utils {
    //! A collection of io-related utilities.
    use std::fs::{self, File};
    use std::io::{self, BufRead};
    use std::path::Path;
    use super::Day;

    /// Input files are either examples (with a known correct answer, used for testing)
    /// or inputs (the actual puzzles with no known answer). Example variations have
    /// an associated example key that is reflected in the filename.
    pub enum InputFileType {
        Input,
        #[allow(dead_code)] Example(u8),
    }

    /// Converts a day and input file type to a filename.
    pub fn input_filename(day: &Day, input_file_type: InputFileType) -> String {
        let file = match input_file_type {
            InputFileType::Input => String::from("input.txt"),
            InputFileType::Example(example_key) => format!("test_examples/example_{example_key}.txt"),
        };
        format!("input/year_{}/day_{:02}/{}", day.year, day.day, file)
    }

    /// Converts a day and filename to a fully pathed filename.
    pub fn filename(day: &Day, filename: &str) -> String {
        format!("input/year_{}/day_{:02}/{}", day.year, day.day, filename)
    }

    /// Returns a string with the entire contents of the file.
    pub fn file_to_string(filename: &str) -> String {
        fs::read_to_string(filename)
            .expect("Should be able to read the file to a string.")
    }

    /// Returns an iterator of strings over lines in file.
    pub fn file_to_lines(filename: &str) -> impl Iterator<Item = String> {
        read_lines(filename).expect("Should be able to open the file.")
            .map(|line| line.expect("Should be able to read the line."))
    }

    /// Reads the file by line. From 
    /// [Rust by Example](https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html)
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}

pub mod math_utils {
    use std::{cmp, collections::HashMap};

    use prime_factorization::Factorization;

    // TODO: make it more generic than u64?
    pub fn least_common_multiple(inputs: impl Iterator<Item = u64> + Clone) -> u64 {
        let input_factors_count = inputs.clone()
            .map(|input| {
                let factors = Factorization::run(input).factors;
                let mut factors_count: HashMap<u64, u64> = HashMap::new();
                for factor in factors {
                    factors_count.entry(factor).and_modify(|count| *count += 1).or_insert(1);
                }
                factors_count
            });
        let common_divisors = input_factors_count.reduce(|common_divisors, factors_count| {
            let mut cd = HashMap::new();
            for factor in common_divisors.keys() {
                if factors_count.contains_key(factor) {
                    cd.insert(*factor, *cmp::min(common_divisors.get(factor).unwrap(), factors_count.get(factor).unwrap()));
                }
            }
            cd
        }).unwrap();
        let greatest_common_divisor: u64 = common_divisors.iter().fold(1, |acc, (divisor, count)| acc * divisor.pow((*count).try_into().unwrap()));
        inputs.map(|period| period / greatest_common_divisor).product::<u64>() * greatest_common_divisor
    }
}

#[cfg(test)]
pub mod test_utils {
    //! A collection of testing-oriented utilities.
    use super::{solution::{Solution, Answer}, io_utils::{InputFileType, input_filename}, Day};

    /// Confirms that the solution's answer matches the example answer.
    pub fn check_example_case<T: Solution>(
        soln: &mut T,
        example_key: u8,
        answer: Answer,
        day: &Day,
    ) {
        assert_eq!(
            soln.solve(&input_filename(day, InputFileType::Example(example_key))),
            answer,
        );
    }
}
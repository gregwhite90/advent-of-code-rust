#[cfg(test)]
const YEAR: u32 = 2017;
#[cfg(test)]
const DAY: u8 = 7;

pub mod part_one {
    use std::{fs, collections::HashMap};
    use regex::Regex;
    pub use either::*;
    use crate::utils::utils::Solution;

    struct Program { // TODO: use borrowed references to strings and lifetimes to avoid unecessary copies.
        name: String,
        weight: u32,
        holding: Vec<String>,
        held_by: Option<String>,
    }

    #[derive(Default)]
    pub struct Soln {
        programs: HashMap<String, Program>,
        held_by: HashMap<String, String>,
    }
 
    impl Solution for Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let re = Regex::new(r"(?<name>[a-z]+) \((?<weight>[0-9]+)\)( -> (?<holding>[a-z ,]+))?")
                .unwrap();
            fs::read_to_string(filename)
                .expect("Should be able to read file to string.")
                .lines()
                .for_each(|line| {
                    let captures = re.captures(line)
                        .expect("Line should match regex.");
                    let name = captures.name("name").unwrap().as_str();
                    let holding = match captures.name("holding") {
                        Some(h) => h.as_str().split(", ").map(|name| String::from(name)).collect(),
                        None => vec![],
                    };
                    for program_held in holding.iter() {
                        if self.programs.contains_key(program_held) {
                            self.programs.entry(String::from(program_held))
                                .and_modify(|program| program.held_by = Some(String::from(name)));
                        } else {
                            self.held_by.insert(String::from(program_held), String::from(name));
                        }
                    }
                    self.programs.insert(
                        String::from(name),
                        Program {
                            name: String::from(name),
                            weight: captures.name("weight")
                                .unwrap()
                                .as_str()
                                .parse()
                                .expect("Weight should be convertible to an unsigned integer."),
                            holding: holding,
                            held_by: self.held_by.remove(name)
                        }
                    );
                });
        }

        fn solve(&mut self) -> Either<i32, String> {
            // TODO: implement. Filter programs for where held_by is None,
            // assert the length is 1, and return that program's name
            let base_programs: Vec<String> = self.programs
                .iter()
                .filter(|(_name, program)| program.held_by == None)
                .map(|(name, _program)| String::from(name))
                .collect();
            assert_eq!(base_programs.len(), 1);
            Right(base_programs[0].clone()) 
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
                    (1u8, Right(String::from("tknk"))),
                ]),
                YEAR,
                DAY,
            );
        }
    }    
}
#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 7};

mod utils {
    use regex::Regex;
    use std::collections::HashMap;
    use crate::utils::io_utils;

    pub struct Program { // TODO: use borrowed references to strings and lifetimes to avoid unecessary copies.
        pub name: String,
        pub individual_weight: u32,
        pub holding: Vec<String>,
        pub held_by: Option<String>,
    }

    pub fn parse_input_file(
        programs: &mut HashMap<String, Program>,
        held_by: &mut HashMap<String, String>,
        filename: &str
    ) {
        let re = Regex::new(r"(?<name>[a-z]+) \((?<weight>[0-9]+)\)( -> (?<holding>[a-z ,]+))?")
            .unwrap();
        io_utils::file_to_lines(filename).for_each(|line| {
            let captures = re.captures(&line)
                .expect("Line should match regex.");
            let name = captures.name("name").unwrap().as_str();
            let holding = match captures.name("holding") {
                Some(h) => h.as_str().split(", ").map(|name| String::from(name)).collect(),
                None => vec![],
            };
            for program_held in holding.iter() {
                if programs.contains_key(program_held) {
                    programs.entry(String::from(program_held))
                        .and_modify(|program| program.held_by = Some(String::from(name)));
                } else {
                    held_by.insert(String::from(program_held), String::from(name));
                }
            }
            programs.insert(
                String::from(name),
                Program {
                    name: String::from(name),
                    individual_weight: captures.name("weight")
                        .unwrap()
                        .as_str()
                        .parse()
                        .expect("Weight should be convertible to an unsigned integer."),
                    holding: holding,
                    held_by: held_by.remove(name)
                }
            );
        });
    }

    pub fn base_program(programs: &HashMap<String, Program>) -> &Program {
        let base_programs: Vec<&Program> = programs
            .iter()
            .filter(|(_name, program)| program.held_by == None)
            .map(|(_name, program)| program)
            .collect();
        assert_eq!(base_programs.len(), 1);
        base_programs[0]
    }

    // TODO: tests
}

pub mod part_one {
    use std::collections::HashMap;
    use crate::utils::solution::{Solution, Answer};
    use super::utils::{self, Program};

    #[derive(Default)]
    pub struct Soln {
        programs: HashMap<String, Program>,
        held_by: HashMap<String, String>,
    }
 
    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::String(utils::base_program(&self.programs).name.clone())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            utils::parse_input_file(&mut self.programs, &mut self.held_by, filename);
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::String(String::from("tknk")); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
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
    use std::collections::HashMap;
    use crate::utils::solution::{Solution, Answer};
    use super::utils::{self, Program};

    #[derive(Default)]
    pub struct Soln {
        programs: HashMap<String, Program>,
        held_by: HashMap<String, String>,
    }
 
    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            let program = utils::base_program(&self.programs);
            let mut weights_incl_holding = HashMap::new();
            self.weight_incl_holding(
                &mut weights_incl_holding,
                program,
            );
            let new_weight: u32 = self.find_misweighted_program(&weights_incl_holding, program.name.clone(), 0);
            Answer::U32(new_weight) 
        }        
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            utils::parse_input_file(&mut self.programs, &mut self.held_by, filename);
        }

        pub fn weight_incl_holding(
            &self,
            weights_including_holding: &mut HashMap<String, u32>,
            program: &Program
        ) -> u32 {
            match weights_including_holding.get(&program.name) { // TODO: figure out if we can do get_or or something here
                None => {
                    let weight_incl_holding = program.individual_weight + 
                        program.holding
                        .iter()
                        .map(|prog_name| { 
                            self.programs
                                .get(prog_name)
                                .expect("Program held by another should exist in the primary map.")
                        }).map(|prog| self.weight_incl_holding(weights_including_holding, prog))
                        .sum::<u32>();
                    weights_including_holding.insert(
                        program.name.clone(),
                        weight_incl_holding,
                    );
                },
                Some(_wt) => (),
            }
            *weights_including_holding.get(&program.name).expect("Program should have a weight including holding entry.")
        }

        // Returns the amount this program would need to change by
        fn find_misweighted_program(
            &self,
            weights_incl_holding: &HashMap<String, u32>,
            prog_name: String,
            change_amount: i32,
        ) -> u32 {
            let mut weight_to_num_holding: HashMap<u32, Vec<String>> = HashMap::new();
            let program = self.programs.get(&prog_name).expect("Program should exist in primary map.");
            program.holding
                .iter()
                .map(|prog_name| 
                    self.programs
                        .get(prog_name).expect("Program held by another should be in primary map.")
                )
                .for_each(|prog| {
                    weight_to_num_holding
                        .entry(*weights_incl_holding.get(&prog.name).expect("Program should have weight including holding already calculated."))
                        .or_insert(vec![])
                        .push(prog.name.clone());
                });
            if weight_to_num_holding.len() <= 1 {
                let wt: i32 = program.individual_weight.try_into().unwrap();
                (wt + change_amount).try_into().unwrap()
            } else {
                // Conditions that need to be true for us to uniquely determine which program is the incorrect weight.
                assert!(program.holding.len() > 2 && weight_to_num_holding.len() == 2);      

                let misweighted_program_names: Vec<String> = weight_to_num_holding
                    .iter()
                    .filter(|(_weight, prog_names)| prog_names.len() == 1)
                    .map(|(_weight, prog_names)| prog_names[0].clone())
                    .collect();
                assert_eq!(misweighted_program_names.len(), 1);
                let misweighted_program_name = misweighted_program_names[0].clone();
                let correct_weights: Vec<u32> = weight_to_num_holding
                    .iter()
                    .filter(|(_weight, prog_names)| prog_names.len() > 1)
                    .map(|(&weight, _prog_names)| weight)
                    .collect();
                assert_eq!(correct_weights.len(),1);
                let correct_weight: i32 = correct_weights[0].try_into().unwrap();
                let misweighted_amount: u32 = *weights_incl_holding.get(
                    &self.programs
                        .get(&misweighted_program_name)
                        .expect("Misweighted program should be in primary map")
                        .name
                ).expect("Misweighted program should have a weight including holding calculated.");
                let misweighted_amount: i32 = misweighted_amount.try_into().unwrap();
                self.find_misweighted_program(
                    weights_incl_holding, 
                    misweighted_program_name, 
                    correct_weight - misweighted_amount,
                )
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(60); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::default(),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}
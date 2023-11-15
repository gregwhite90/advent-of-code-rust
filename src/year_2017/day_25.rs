#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 25 };

pub mod part_one {
    use std::collections::{HashMap, HashSet};

    use regex::Regex;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    #[derive(Debug, PartialEq, Eq, Default)]
    struct Instructions {
        write_value: bool,
        move_direction: isize,
        continuation_state: char,
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    struct StateInstructions {
        zero: Instructions,
        one: Instructions,
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        state: char,
        slot: isize,
        steps: usize,
        all_instructions: HashMap<char, StateInstructions>,
        ones: HashSet<isize>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            for _ in 0..self.steps {
                self.step();
            }
            Answer::U32(self.ones.len().try_into().unwrap())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let start_state_re = Regex::new(r"Begin in state (?<state>[A-Z]).").unwrap();
            let steps_re = Regex::new(r"Perform a diagnostic checksum after (?<steps>\d+) steps.").unwrap();
            let state_re = Regex::new(r"In state (?<state>[A-Z]):").unwrap();
            let current_value_re = Regex::new(r"  If the current value is (?<val>[01]):").unwrap();
            let write_re = Regex::new(r"    - Write the value (?<val>[01]).").unwrap();
            let move_re = Regex::new(r"    - Move one slot to the (?<dir>right|left).").unwrap();
            let continuation_state_re = Regex::new(r"    - Continue with state (?<state>[A-Z]).").unwrap();
            let mut state: Option<char> = None;
            let mut current_value: Option<bool> = None;

            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    if let Some(captures) = start_state_re.captures(&line) {
                        self.state = captures.name("state").unwrap().as_str().chars().next().unwrap();
                    } else if let Some(captures) = steps_re.captures(&line) {
                        self.steps = captures.name("steps").unwrap().as_str().parse().unwrap();
                    } else if let Some(captures) = state_re.captures(&line) {
                        let st = captures.name("state").unwrap().as_str().chars().next().unwrap();
                        self.all_instructions.insert(st, StateInstructions::default());
                        state = Some(st);
                    } else if let Some(captures) = current_value_re.captures(&line) {
                        let val: u8 = captures.name("val").unwrap().as_str().parse().unwrap();
                        current_value = Some(val != 0);
                    } else if let Some(captures) = write_re.captures(&line) {
                        let val: u8 = captures.name("val").unwrap().as_str().parse().unwrap();
                        let val = val != 0;
                        self.all_instructions.entry(state.unwrap())
                            .and_modify(|state_instructions| {
                                match current_value.unwrap() {
                                    true => state_instructions.one.write_value = val,
                                    false => state_instructions.zero.write_value = val,
                                }
                            });
                    } else if let Some(captures) = move_re.captures(&line) {
                        let dir = captures.name("dir").unwrap().as_str();
                        let val: isize = match dir {
                            "right" => 1,
                            "left" => -1,
                            _ => panic!("Unknown direction."),
                        };
                        self.all_instructions.entry(state.unwrap())
                            .and_modify(|state_instructions| {
                                match current_value.unwrap() {
                                    true => state_instructions.one.move_direction = val,
                                    false => state_instructions.zero.move_direction = val,
                                }
                            });
                    } else if let Some(captures) = continuation_state_re.captures(&line) {
                        let continuation_state = captures.name("state").unwrap().as_str().chars().next().unwrap();
                        self.all_instructions.entry(state.unwrap())
                            .and_modify(|state_instructions| {
                                match current_value.unwrap() {
                                    true => state_instructions.one.continuation_state = continuation_state,
                                    false => state_instructions.zero.continuation_state = continuation_state,
                                }
                            });
                    }
                });
        }

        fn step(&mut self) {
            let state_instructions = self.all_instructions.get(&self.state).unwrap();
            let instructions = match self.ones.contains(&self.slot) {
                true => &state_instructions.one,
                false => &state_instructions.zero,
            };
            match instructions.write_value {
                true => { self.ones.insert(self.slot); },
                false => { self.ones.remove(&self.slot); },
            }
            self.slot += instructions.move_direction;
            self.state = instructions.continuation_state;
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(3); "example_1")]
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
#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 12 };

pub mod part_one {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::collections::{HashSet, HashMap};

    use regex::Regex;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    #[derive(PartialEq, Eq, Debug)]
    struct Group {
        programs: HashSet<u32>,
    }

    #[derive(PartialEq, Eq, Debug, Default)]
    pub struct Soln {
        groups: HashMap<u32, Rc<RefCell<Group>>>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.group_len(0))
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let re = Regex::new(r"(?<program>\d+) <\-> (?<pipes>[ \d,]+)").unwrap();
            io_utils::file_to_lines(filename).for_each(|line| {
                let captures = re.captures(&line)
                    .expect("Line should match regex.");
                let program: u32 = captures.name("program").unwrap().as_str().parse().unwrap();
                self.handle_pipes(
                    program,
                    captures.name("pipes")
                        .unwrap()
                        .as_str()
                        .split(", ")
                        .map(|pipe| pipe.parse::<u32>().expect("Should be able to parse to `u32`."))
                        .collect()
                );
            });
        }

        fn handle_pipes(&mut self, program: u32, pipes: Vec<u32>) {
            self.groups.insert(
                program,
                Rc::new(RefCell::new(Group { programs: HashSet::new() })),
            );
            pipes.iter().for_each(|pipe| {
                if *pipe == program {
                    self.groups.get(&program).unwrap().borrow_mut().programs.insert(*pipe);
                } else {
                    match self.groups.remove(pipe) {
                        None => {
                            self.groups.get(&program).unwrap().borrow_mut().programs.insert(*pipe);
                        },
                        Some(pipe_group) => {        
                            let program_group = self.groups.remove(&program).unwrap();
                            if program_group != pipe_group {
                                pipe_group.borrow_mut().programs.insert(program);
                                pipe_group.borrow_mut().programs.insert(*pipe);
                                pipe_group.borrow_mut().programs.extend(program_group.borrow().programs.iter());
                            }
                            self.groups.insert(program, Rc::clone(&pipe_group));
                            for prog in program_group.borrow().programs.iter() {
                                self.groups.insert(*prog, Rc::clone(&pipe_group));
                            }
                            self.groups.insert(*pipe, pipe_group);
                        },
                    }    
                }
            });
        }

        fn group_len(&self, program: u32) -> u32 {
            self.groups
                .get(&program)
                .expect("Program should exist.")
                .borrow()
                .programs
                .len()
                .try_into()
                .expect("Number of programs should fit in `u32` datatype.")
        }

        
    }
 
    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test]
        fn group_len_is_correct() {
            let group = Rc::new(RefCell::new(Group { programs: HashSet::from([0, 2, 3, 4]) }));
            let soln = Soln {
                groups: HashMap::from([
                    (1, Rc::new(RefCell::new(Group { programs: HashSet::from([1]) }))),
                    (2, Rc::clone(&group)),
                    (3, Rc::clone(&group)),
                    (4, Rc::clone(&group)),
                    (0, group),
                ])
            };
            assert_eq!(soln.group_len(0), 4);
            assert_eq!(soln.group_len(1), 1);
            assert_eq!(soln.group_len(2), 4);
            assert_eq!(soln.group_len(3), 4);
            assert_eq!(soln.group_len(4), 4);
        }

        #[test]
        fn handle_pipes_is_correct() {
            let mut soln = Soln::default();
            soln.handle_pipes(0,vec![2]);
            assert_eq!(soln.group_len(0), 1);
            soln.handle_pipes(1, vec![1]);
            assert_eq!(soln.group_len(0), 1);
            assert_eq!(soln.group_len(1), 1);
            soln.handle_pipes(2, vec![0, 3, 4]);
            assert_eq!(soln.group_len(0), 4);
        }

        #[test_case(1, Answer::U32(6); "example_1")]
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
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::collections::{HashSet, HashMap};

    use regex::Regex;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    #[derive(PartialEq, Eq, Debug)]
    struct Group {
        programs: HashSet<u32>,
    }

    #[derive(PartialEq, Eq, Debug, Default)]
    pub struct Soln {
        groups: HashMap<u32, Rc<RefCell<Group>>>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            Answer::U32(self.groups())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let re = Regex::new(r"(?<program>\d+) <\-> (?<pipes>[ \d,]+)").unwrap();
            io_utils::file_to_lines(filename).for_each(|line| {
                let captures = re.captures(&line)
                    .expect("Line should match regex.");
                let program: u32 = captures.name("program").unwrap().as_str().parse().unwrap();
                self.handle_pipes(
                    program,
                    captures.name("pipes")
                        .unwrap()
                        .as_str()
                        .split(", ")
                        .map(|pipe| pipe.parse::<u32>().expect("Should be able to parse to `u32`."))
                        .collect()
                );
            });
        }

        fn handle_pipes(&mut self, program: u32, pipes: Vec<u32>) {
            self.groups.insert(
                program,
                Rc::new(RefCell::new(Group { programs: HashSet::new() })),
            );
            pipes.iter().for_each(|pipe| {
                if *pipe == program {
                    self.groups.get(&program).unwrap().borrow_mut().programs.insert(*pipe);
                } else {
                    match self.groups.remove(pipe) {
                        None => {
                            self.groups.get(&program).unwrap().borrow_mut().programs.insert(*pipe);
                        },
                        Some(pipe_group) => {        
                            let program_group = self.groups.remove(&program).unwrap();
                            if program_group != pipe_group {
                                pipe_group.borrow_mut().programs.insert(program);
                                pipe_group.borrow_mut().programs.insert(*pipe);
                                pipe_group.borrow_mut().programs.extend(program_group.borrow().programs.iter());
                            }
                            self.groups.insert(program, Rc::clone(&pipe_group));
                            for prog in program_group.borrow().programs.iter() {
                                self.groups.insert(*prog, Rc::clone(&pipe_group));
                            }
                            self.groups.insert(*pipe, pipe_group);
                        },
                    }    
                }
            });
        }

        fn group_len(&self, program: u32) -> u32 {
            self.groups
                .get(&program)
                .expect("Program should exist.")
                .borrow()
                .programs
                .len()
                .try_into()
                .expect("Number of programs should fit in `u32` datatype.")
        }

        fn groups(&self) -> u32 {
            let mut counted: HashSet<u32> = HashSet::new();
            self.groups.values()
                .map(|group| {
                    match counted.intersection(&group.borrow().programs).count() {
                        0 => {
                            counted.extend(&group.borrow().programs);
                            1
                        },
                        x => {
                            assert_eq!(group.borrow().programs.len(), x);
                            0
                        },
                    }
                })
                .sum()
        }        
    }
 
    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test]
        fn group_len_is_correct() {
            let group = Rc::new(RefCell::new(Group { programs: HashSet::from([0, 2, 3, 4]) }));
            let soln = Soln {
                groups: HashMap::from([
                    (1, Rc::new(RefCell::new(Group { programs: HashSet::from([1]) }))),
                    (2, Rc::clone(&group)),
                    (3, Rc::clone(&group)),
                    (4, Rc::clone(&group)),
                    (0, group),
                ])
            };
            assert_eq!(soln.group_len(0), 4);
            assert_eq!(soln.group_len(1), 1);
            assert_eq!(soln.group_len(2), 4);
            assert_eq!(soln.group_len(3), 4);
            assert_eq!(soln.group_len(4), 4);
        }

        #[test]
        fn handle_pipes_is_correct() {
            let mut soln = Soln::default();
            soln.handle_pipes(0,vec![2]);
            assert_eq!(soln.group_len(0), 1);
            soln.handle_pipes(1, vec![1]);
            assert_eq!(soln.group_len(0), 1);
            assert_eq!(soln.group_len(1), 1);
            soln.handle_pipes(2, vec![0, 3, 4]);
            assert_eq!(soln.group_len(0), 4);
        }

        #[test_case(1, Answer::U32(2); "example_1")]
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
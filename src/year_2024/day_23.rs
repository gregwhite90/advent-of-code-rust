#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2024, day: 23 };

mod utils {
    use std::collections::{BTreeSet, HashMap, HashSet};

    use itertools::Itertools;
    use lazy_static::lazy_static;
    use regex::Regex;

    use crate::utils::io_utils;

    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"(?<l>\w+)\-(?<r>\w+)").unwrap();
    }

    #[derive(Debug, Default)]
    pub struct NetworkMap {
        connections: HashMap<String, HashSet<String>>,
    }

    impl NetworkMap {
        pub fn parse_input_file(&mut self, filename: &str) {
            io_utils::file_to_lines(filename).for_each(|line| {
                let captures = LINE_RE.captures(&line).unwrap();
                let l = captures.name("l").unwrap().as_str();
                let r = captures.name("r").unwrap().as_str();
                self.connections.entry(l.to_string())
                    .or_insert(HashSet::new())
                    .insert(r.to_string());
                self.connections.entry(r.to_string())
                    .or_insert(HashSet::new())
                    .insert(l.to_string());
            })
        }

        pub fn num_subsets(&self, starting_char: char) -> usize {
            let mut subsets: HashSet<BTreeSet<String>> = HashSet::new();
            for comp in self.connections.keys()
                .filter(|c| c.starts_with(starting_char)) {
                    for pair in self.connections.get(comp).unwrap().iter().combinations(2) {
                        if self.connections.get(pair[0]).unwrap().contains(pair[1]) {
                            subsets.insert(BTreeSet::from([
                                comp.clone(),
                                pair[0].clone(),
                                pair[1].clone(),
                            ]));
                        }
                    }
                }
            subsets.len()
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::NetworkMap;

    #[derive(Debug, Default)]
    pub struct Soln {
        network_map: NetworkMap,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.network_map.parse_input_file(filename);
            Answer::Usize(self.network_map.num_subsets('t'))
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(7); "example_1")]
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
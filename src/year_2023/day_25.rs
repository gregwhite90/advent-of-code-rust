#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 25 };

pub mod part_one {
    use std::collections::HashSet;

    use itertools::Itertools;
    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Edge {
        vertices: [String; 2],
    }

    impl Edge {
        fn in_and_out_of_subset(&self, subset: &HashSet<&String>) -> bool {
            self.vertices.iter().filter(|v| subset.contains(v)).count() == 1
        }
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        vertices: HashSet<String>,
        edges: HashSet<Edge>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            for subset in self.vertices.iter().powerset() {
                let subset = HashSet::from_iter(subset);
                if self.edges.iter().filter(|edge| edge.in_and_out_of_subset(&subset)).count() == 3 {
                    return Answer::Usize(subset.len() * (self.vertices.len() - subset.len()));
                }
            }
            panic!("Should have found an answer by now.");
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let re = Regex::new(r"(?<component>[a-z]+): (?<connected>[ a-z]+)").unwrap();
            io_utils::file_to_lines(filename).for_each(|line| {
                let captures = re.captures(&line).unwrap();
                let component = captures.name("component").unwrap().as_str();
                self.vertices.insert(String::from(component));
                let connected = captures.name("connected").unwrap().as_str();
                for cc in connected.split(' ') {
                    self.vertices.insert(String::from(cc));
                    self.edges.insert(Edge { vertices: [String::from(component), String::from(cc)] });
                }
            });
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(54); "example_1")]
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
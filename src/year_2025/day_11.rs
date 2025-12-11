#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2025, day: 11 };

mod utils {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        pub static ref LINE_RE: Regex = Regex::new(r"(?<device>\w+): (?<output_devices>[\w ]+)").unwrap();
    }
}

pub mod part_one {
    use std::collections::HashMap;

    use crate::utils::{io_utils, solution::{Answer, Solution}};
    use super::utils::LINE_RE;

    #[derive(Debug, Default, PartialEq, Eq)]
    struct ServerRack {
        connections: HashMap<String, Vec<String>>,
    }

    impl ServerRack {
        pub fn add_line(&mut self, line: &str) {
            let captures = LINE_RE.captures(line).unwrap();
            self.connections.insert(
                captures.name("device").unwrap().as_str().to_string(),
                captures.name("output_devices").unwrap().as_str().split(' ').map(String::from).collect(),
            );
        }

        pub fn num_paths(&self) -> usize {
            let mut cache: HashMap<String, usize> = HashMap::new();
            self.num_paths_from_device(&mut cache, "you")
        }

        fn num_paths_from_device(&self, cache: &mut HashMap<String, usize>, device: &str) -> usize {
            if device == "out" {
                1
            } else if let Some(paths) = cache.get(device) {
                *paths
            } else {
                let paths = self.connections.get(device)
                    .unwrap()
                    .iter()
                    .map(|d| {
                        self.num_paths_from_device(cache, d)
                    })
                    .sum();
                cache.insert(
                    device.to_string(),
                    paths,
                );
                paths
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        server_rack: ServerRack,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    self.server_rack.add_line(&line);
                });
            Answer::Usize(
                self.server_rack.num_paths()
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(5); "example_1")]
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
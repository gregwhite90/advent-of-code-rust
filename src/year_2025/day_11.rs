#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2025, day: 11 };

mod utils {
    use std::collections::{BTreeMap, HashMap};

    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        pub static ref LINE_RE: Regex = Regex::new(r"(?<device>\w+): (?<output_devices>[\w ]+)").unwrap();
    }

    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct ServerRack {
        connections: HashMap<String, Vec<String>>,
        required: Vec<String>,
    }

    impl ServerRack {
        pub fn with_required(
            required: Vec<String>,
        ) -> Self {
            Self {
                connections: HashMap::default(),
                required,
            }
        }

        pub fn add_line(&mut self, line: &str) {
            let captures = LINE_RE.captures(line).unwrap();
            self.connections.insert(
                captures.name("device").unwrap().as_str().to_string(),
                captures.name("output_devices").unwrap().as_str().split(' ').map(String::from).collect(),
            );
        }

        pub fn num_paths(&self, start_device: &str) -> usize {
            let mut cache: HashMap<(String, BTreeMap<String, bool>), usize> = HashMap::new();
            let visited_required: BTreeMap<String, bool> = BTreeMap::from_iter(
                self.required.iter().map(|d| {
                    (d.to_string(), false)
                })
            );
            self.num_paths_from_device(
                &mut cache,
                start_device,
                visited_required,
            )
        }

        fn num_paths_from_device(
            &self,
            cache: &mut HashMap<(String, BTreeMap<String, bool>), usize>,
            device: &str,
            visited_required: BTreeMap<String, bool>,
        ) -> usize {
            if device == "out" {
                if visited_required.values().all(|visited| *visited) {
                    1
                } else {
                    0
                }
            } else if let Some(paths) = cache.get(&(device.to_string(), visited_required.clone())) {
                *paths
            } else {
                let mut vr = visited_required.clone();
                vr.entry(device.to_string())
                    .and_modify(|v| *v = true);
                let paths = self.connections.get(device)
                    .unwrap()
                    .iter()
                    .map(|d| {
                        self.num_paths_from_device(
                            cache,
                            d,
                            vr.clone(),
                        )
                    })
                    .sum();
                cache.insert(
                    (
                        device.to_string(),
                        vr,
                    ),
                    paths,
                );
                paths
            }
        }
    }
}

pub mod part_one {
    use crate::{utils::{io_utils, solution::{Answer, Solution}}};
    use super::utils::ServerRack;

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
                self.server_rack.num_paths("you")
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

pub mod part_two {
    use crate::{utils::{io_utils, solution::{Answer, Solution}}};
    use super::utils::ServerRack;

    #[derive(Debug)]
    pub struct Soln {
        server_rack: ServerRack,
    }

    impl Default for Soln {
        fn default() -> Self {
            Self {
                server_rack: ServerRack::with_required(
                    vec![
                        "dac".to_string(),
                        "fft".to_string(),
                    ],
                )
            }
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    self.server_rack.add_line(&line);
                });
            Answer::Usize(
                self.server_rack.num_paths("svr")
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(2, Answer::Usize(2); "example_2")]
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
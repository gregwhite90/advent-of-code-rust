#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 12 };

mod utils {
    use std::collections::HashSet;

    use regex::Regex;

    use crate::utils::io_utils;

    #[derive(Debug, Default)]
    pub struct PlantPropagator {
        pots_with_plants: HashSet<i64>,
        // the sequences that yield a plant in the next generation
        // TODO: change to just a BTreeSet of indices?
        propagation_notes: HashSet<Vec<bool>>,
    }

    impl PlantPropagator {
        pub fn parse_input_file(&mut self, filename: &str) {
            let initial_re = Regex::new(r"initial state: (?<state>[#\.]+)").unwrap();
            let note_re = Regex::new(r"(?<pattern>[#\.]{5}) => (?<yield>[#\.])").unwrap();
            io_utils::file_to_lines(filename).for_each(|line| {
                if let Some(captures) = initial_re.captures(&line) {
                    let state = captures.name("state").unwrap().as_str();
                    self.pots_with_plants = state.char_indices()
                        .filter(|(_idx, ch)| *ch == '#')
                        .map(|(idx, _ch)| idx.try_into().unwrap())
                        .collect();
                } else if let Some(captures) = note_re.captures(&line) {
                    if captures.name("yield").unwrap().as_str() == "#" {
                        self.propagation_notes.insert(
                            captures.name("pattern").unwrap()
                                .as_str()
                                .chars()
                                .map(|ch| {
                                    match ch {
                                        '.' => false,
                                        '#' => true,
                                        _ => panic!("Unrecognized character in pattern"),
                                    }
                                })
                                .collect()
                        );
                    }
                }
            })
        }

        /// Note: relies on the "....." => "." note being present,
        /// otherwise, would need to be more thoughtful about the edges
        pub fn propagate(&mut self) {
            let mut new_pots_with_plants = HashSet::new();
            let min = *self.pots_with_plants.iter().min().unwrap_or(&0);
            let max = *self.pots_with_plants.iter().max().unwrap_or(&0);
            for idx in min - 2..=max + 2 {
                if self.propagation_notes.contains(
                    &vec![
                        self.pots_with_plants.contains(&(idx - 2)),
                        self.pots_with_plants.contains(&(idx - 1)),
                        self.pots_with_plants.contains(&(idx)),
                        self.pots_with_plants.contains(&(idx + 1)),
                        self.pots_with_plants.contains(&(idx + 2)),
                    ]
                ) {
                    new_pots_with_plants.insert(idx);
                }
            }            
            self.pots_with_plants = new_pots_with_plants;
        }

        pub fn sum_of_pots_with_plants(&self) -> i64 {
            self.pots_with_plants.iter().sum()
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::PlantPropagator;

    #[derive(Debug, Default)]
    pub struct Soln {
        plant_propagator: PlantPropagator,    
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.plant_propagator.parse_input_file(filename);
            for _ in 0..20 {
                self.plant_propagator.propagate();
            }
            Answer::I64(self.plant_propagator.sum_of_pots_with_plants())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::I64(325); "example_1")]
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
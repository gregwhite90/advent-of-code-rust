#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 24 };

mod utils {
    use std::{collections::BTreeSet, cmp::Ordering};

    use regex::Regex;

    use crate::utils::io_utils;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    pub struct Component {
        l: u32,
        r: u32,
    }

    impl Component {
        pub fn strength(&self) -> u32 {
            self.l + self.r
        }

        pub fn contains_port(&self, port: u32) -> bool {
            self.l == port || self.r == port
        }

        pub fn next_starting_port(&self, cur_starting_port: u32) -> u32 {
            if self.l == cur_starting_port { self.r } else { self.l }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Bridge {
        used: Vec<Component>,
        unused: BTreeSet<Component>,
    }

    impl Bridge {
        pub fn new(unused: BTreeSet<Component>) -> Self {
            Self {
                used: vec![],
                unused,
            }
        }
        pub fn strength(&self) -> u32 {
            self.used.iter().map(|component| {
                component.strength()
            }).sum()
        }
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct BridgeBuilder {
        components: BTreeSet<Component>,
    }

    pub fn superlative_bridge<F>(current_bridge: &Bridge, starting_port: u32, compare: &F) -> Bridge
    where
        F: Fn(&Bridge, &Bridge) -> Ordering
    {
        let superlative_sub_bridge: Option<Bridge> = current_bridge.unused.iter()
            .filter(|unused_component| {
                unused_component.contains_port(starting_port)
            })
            .map(|nc| {
                let mut new_bridge = current_bridge.clone();
                let c = new_bridge.unused.take(nc).unwrap();
                let next_starting_port = c.next_starting_port(starting_port);
                new_bridge.used.push(c); 
                superlative_bridge(&new_bridge, next_starting_port, compare)
            })
            .max_by(compare);
        match superlative_sub_bridge {
            None => current_bridge.clone(),
            Some(sub_bridge) => sub_bridge,
        }
    }

    impl BridgeBuilder {
        pub fn parse_input_file(&mut self, filename: &str) {
            let re = Regex::new(r"(?<l>\d+)/(?<r>\d+)").unwrap();
            io_utils::file_to_lines(filename)
                .for_each(|line| {
                    let captures = re.captures(&line)
                        .expect("Input line should match known form.");
                    let l: u32 = captures.name("l").unwrap().as_str().parse().unwrap();
                    let r: u32 = captures.name("r").unwrap().as_str().parse().unwrap();
                    if !self.components.insert(Component { l, r }) {
                        panic!("Found a duplicate component.");
                    }
                });
        }

        pub fn components(&self) -> &BTreeSet<Component> {
            &self.components
        }
    }
}

pub mod part_one {
    use std::cmp::Ordering;

    use crate::utils::solution::{Solution, Answer};

    use super::utils::{BridgeBuilder, Bridge, superlative_bridge};

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        bridge_builder: BridgeBuilder,
    }

    fn compare_by_strength(l: &Bridge, r: &Bridge) -> Ordering {
        l.strength().cmp(&r.strength())
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.bridge_builder.parse_input_file(filename);
            let bridge = Bridge::new(self.bridge_builder.components().clone());
            let strongest_bridge = superlative_bridge(&bridge, 0, &compare_by_strength);
            Answer::U32(strongest_bridge.strength())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(31); "example_1")]
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
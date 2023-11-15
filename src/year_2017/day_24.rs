#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2017, day: 24 };

pub mod part_one {
    use std::collections::BTreeSet;

    use regex::Regex;

    use crate::utils::{solution::{Solution, Answer}, io_utils};

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    struct Component {
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
    struct Bridge {
        used: Vec<Component>,
        unused: BTreeSet<Component>,
    }

    impl Bridge {
        pub fn strength(&self) -> u32 {
            self.used.iter().map(|component| {
                component.strength()
            }).sum()
        }
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    pub struct Soln {
        components: BTreeSet<Component>,
    }

    fn strongest_bridge(current_bridge: &Bridge, starting_port: u32) -> Bridge {
        let strongest_sub_bridge: Option<Bridge> = current_bridge.unused.iter()
            .filter(|unused_component| {
                unused_component.contains_port(starting_port)
            })
            .map(|nc| {
                let mut new_bridge = current_bridge.clone();
                let c = new_bridge.unused.take(nc).unwrap();
                let next_starting_port = c.next_starting_port(starting_port);
                new_bridge.used.push(c); 
                strongest_bridge(&new_bridge, next_starting_port)
            })
            .max_by_key(|bridge| bridge.strength());
        match strongest_sub_bridge {
            None => current_bridge.clone(),
            Some(sub_bridge) => sub_bridge,
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
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
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            let bridge = Bridge {
                used: vec![],
                unused: self.components.clone(),
            };
            let strongest_bridge = strongest_bridge(&bridge, 0);
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
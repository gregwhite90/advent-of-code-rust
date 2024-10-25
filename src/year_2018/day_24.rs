#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 24 };

mod utils {
    use std::collections::{HashMap, HashSet};
    
    use lazy_static::lazy_static;
    use regex::Regex;

    use crate::utils::io_utils;

    lazy_static! {
        static ref GROUP_RE: Regex = Regex::new(r"(?<units>\d+) units each with (?<hit_points>\d+) hit points (?<weaknesses_and_immunities>[\(a-z \,\;\)]+)?with an attack that does (?<attack_damage>\d+) (?<attack_type>[a-z ]+) damage at initiative (?<initiative>\d+)").unwrap();
        static ref WEAKNESS_IMMUNITY_RE: Regex = Regex::new(r"(?<modifier>weak|immune) to (?<types>[a-z \,]+)[\)\;]").unwrap();
    }

    #[derive(Debug, Default)]
    pub struct ImmuneSystem {
        immune_system: Army,
        infection: Army,
    }

    impl ImmuneSystem {
        pub fn parse_input_file(&mut self, filename: &str) {
            let mut infection_mode = false;
            let immune_system_re = Regex::new(r"Immune System:").unwrap();
            let infection_re = Regex::new(r"Infection").unwrap();
            for line in io_utils::file_to_lines(filename) {
                if line.len() == 0 || immune_system_re.is_match(&line) { continue; }
                else if infection_re.is_match(&line) { infection_mode = true; }
                else {
                    let group = Group::from_str(&line);
                    if infection_mode {
                        self.infection.add_group(group);
                    } else {
                        self.immune_system.add_group(group);
                    }
                }
            };
        }

        pub fn num_units(&self) -> usize {
            self.immune_system.num_units() + self.infection.num_units()
        }
    }

    #[derive(Debug, Default)]
    struct Army {
        groups: HashMap<usize, Group>,
    }

    impl Army {
        pub fn add_group(&mut self, group: Group) {
            self.groups.insert(self.groups.len(), group);
        }

        pub fn num_units(&self) -> usize {
            self.groups.values().map(|group| group.units).sum()
        }
    }

    #[derive(Debug)]
    struct Group {
        units: usize,
        hit_points: usize,
        attack_damage: usize,
        attack_type: String,
        initiative: usize,
        weaknesses: HashSet<String>,
        immunities: HashSet<String>,
    }

    impl Group {
        pub fn from_str(line: &str) -> Self {
            let caps = GROUP_RE.captures(line).unwrap();
            let units = caps.name("units").unwrap().as_str().parse().unwrap();
            let hit_points = caps.name("hit_points").unwrap().as_str().parse().unwrap();
            let attack_damage = caps.name("attack_damage").unwrap().as_str().parse().unwrap();
            let attack_type = caps.name("attack_type").unwrap().as_str().to_string();
            let initiative = caps.name("initiative").unwrap().as_str().parse().unwrap();
            let mut weaknesses = HashSet::new();
            let mut immunities = HashSet::new();
            if let Some(w_and_i) = caps.name("weaknesses_and_immunities") {
                for caps in WEAKNESS_IMMUNITY_RE.captures_iter(w_and_i.as_str()) {
                    let modifier = caps.name("modifier").unwrap().as_str();
                    let types = caps.name("types").unwrap().as_str().split(", ").map(|t| t.to_string()).collect();
                    if modifier == "weak" {
                        weaknesses = types;
                    } else {
                        assert!(modifier == "immune");
                        immunities = types;
                    }
                }
            }
            Self {
                units,
                hit_points,
                attack_damage,
                attack_type,
                initiative,
                weaknesses,
                immunities
            }
        }

        pub fn effective_power(&self) -> usize {
            self.units * self.attack_damage
        }
    }
}

pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    use super::utils::ImmuneSystem;

    #[derive(Debug, Default)]
    pub struct Soln {
        immune_system: ImmuneSystem,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.immune_system.parse_input_file(filename);
            Answer::Usize(self.immune_system.num_units())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(114); "example_1")]
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
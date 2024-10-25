#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2018, day: 24 };

mod utils {
    use std::{cmp, collections::{HashMap, HashSet}};
    
    use itertools::Itertools;
    use lazy_static::lazy_static;
    use regex::Regex;

    use crate::utils::io_utils;

    lazy_static! {
        static ref GROUP_RE: Regex = Regex::new(r"(?<units>\d+) units each with (?<hit_points>\d+) hit points (?<weaknesses_and_immunities>[\(a-z \,\;\)]+)?with an attack that does (?<attack_damage>\d+) (?<attack_type>[a-z ]+) damage at initiative (?<initiative>\d+)").unwrap();
        static ref WEAKNESS_IMMUNITY_RE: Regex = Regex::new(r"(?<modifier>weak|immune) to (?<types>[a-z \,]+)[\)\;]").unwrap();
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum ArmyType {
        IMMUNE_SYSTEM,
        INFECTION,
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

        pub fn fight(&mut self) {
            // Select targets
            let mut immune_system_selected_targets: HashMap<usize, usize> = HashMap::new();
            for (attacking_id, attacking_group) in self.immune_system.groups.iter().sorted_by(|a, b| {
                b.1.effective_power().cmp(&a.1.effective_power())
                    .then_with(|| b.1.initiative.cmp(&a.1.initiative))
            }) {
                if let Some((defending_id, _defending_group)) = self.infection.groups.iter()
                    .filter(|(defending_id, defending_group)| {
                        !immune_system_selected_targets.values().contains(defending_id)
                        && !defending_group.immunities.contains(&attacking_group.attack_type)
                    })
                    .max_by(|a, b| {
                        a.1.weaknesses.contains(&attacking_group.attack_type).cmp(&b.1.weaknesses.contains(&attacking_group.attack_type))
                            .then_with(|| a.1.effective_power().cmp(&b.1.effective_power()))
                            .then_with(|| a.1.initiative.cmp(&b.1.initiative))
                    }) {
                        immune_system_selected_targets.insert(*attacking_id, *defending_id);
                    }
            }
            // TODO: refactor shared functionality?
            let mut infection_selected_targets: HashMap<usize, usize> = HashMap::new();
            for (attacking_id, attacking_group) in self.infection.groups.iter().sorted_by(|a, b| {
                b.1.effective_power().cmp(&a.1.effective_power())
                    .then_with(|| b.1.initiative.cmp(&a.1.initiative))
            }) {
                if let Some((defending_id, _defending_group)) = self.immune_system.groups.iter()
                    .filter(|(defending_id, defending_group)| {
                        !infection_selected_targets.values().contains(defending_id)
                        && !defending_group.immunities.contains(&attacking_group.attack_type)
                    })
                    .max_by(|a, b| {
                        a.1.weaknesses.contains(&attacking_group.attack_type).cmp(&b.1.weaknesses.contains(&attacking_group.attack_type))
                            .then_with(|| a.1.effective_power().cmp(&b.1.effective_power()))
                            .then_with(|| a.1.initiative.cmp(&b.1.initiative))
                    }) {
                        infection_selected_targets.insert(*attacking_id, *defending_id);
                    }
            }
            // Attack
            for (army_type, id) in self.all_groups_by_initiative() {
                match army_type {
                    ArmyType::IMMUNE_SYSTEM => {
                        if let Some(defending_id) = immune_system_selected_targets.get(&id) {
                            let attacking_group = self.immune_system.groups.get(&id).unwrap();
                            let defending_group = self.infection.groups.get_mut(&defending_id).unwrap();
                            let damage = attacking_group.effective_power()
                                * if defending_group.weaknesses.contains(&attacking_group.attack_type) { 2 } else { 1 };
                            let units_killed = cmp::min(defending_group.units, damage / defending_group.hit_points);
                            defending_group.remove_units(units_killed);
                        }
                    },
                    ArmyType::INFECTION => {
                        if let Some(defending_id) = infection_selected_targets.get(&id) {
                            let attacking_group = self.infection.groups.get(&id).unwrap();
                            let defending_group = self.immune_system.groups.get_mut(&defending_id).unwrap();
                            let damage = attacking_group.effective_power()
                                * if defending_group.weaknesses.contains(&attacking_group.attack_type) { 2 } else { 1 };
                            let units_killed = cmp::min(defending_group.units, damage / defending_group.hit_points);
                            defending_group.remove_units(units_killed);
                        }                        
                    },
                }
            }
            // Remove groups with no units remaining after the fight 
            for (army_type, id) in self.all_groups_by_initiative() {
                if self.units_remaining(army_type, id) == 0 {
                    match army_type {
                        ArmyType::IMMUNE_SYSTEM => {
                            self.immune_system.groups.remove(&id);
                        },
                        ArmyType::INFECTION => {
                            self.infection.groups.remove(&id);
                        },
                    }
                }
            }
        }

        fn all_groups_by_initiative(&self) -> Vec<(ArmyType, usize)> {
            self.immune_system.groups.iter().map(|(id, group)| (ArmyType::IMMUNE_SYSTEM, id, group.initiative)).chain(
                self.infection.groups.iter().map(|(id, group)| (ArmyType::INFECTION, id, group.initiative))
            )
                .sorted_by(|a, b| b.2.cmp(&a.2))
                .map(|(army_type, id, _initiative)| (army_type, *id))
                .collect()
        }

        fn units_remaining(&self, army_type: ArmyType, id: usize) -> usize {
            match army_type {
                ArmyType::IMMUNE_SYSTEM => {
                    if let Some(group) = self.immune_system.groups.get(&id) {
                        group.units
                    } else {
                        0
                    }
                },
                ArmyType::INFECTION => {
                    if let Some(group) = self.infection.groups.get(&id) {
                        group.units
                    } else {
                        0
                    }
                }
            }
        }

        pub fn fights_completed(&self) -> bool {
            self.immune_system.groups.len() == 0 || self.infection.groups.len() == 0
        }

        pub fn add_boost(&mut self, boost: usize) {
            for (_id, group) in self.immune_system.groups.iter_mut() {
                group.add_boost(boost);
            }
        }

        pub fn winning_army(&self) -> Option<ArmyType> {
            assert!(self.fights_completed());
            if self.immune_system.groups.len() == 0 && self.infection.groups.len() == 0 {
                None
            } else if self.immune_system.groups.len() == 0 {
                Some(ArmyType::INFECTION)
            } else {
                assert!(self.infection.groups.len() == 0);
                Some(ArmyType::IMMUNE_SYSTEM)
            }
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

        pub fn remove_units(&mut self, units: usize) {
            self.units -= units;
        }

        pub fn add_boost(&mut self, boost: usize) {
            self.attack_damage += boost;
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
            while !self.immune_system.fights_completed() {
                self.immune_system.fight();
            }
            Answer::Usize(self.immune_system.num_units())
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(5_216); "example_1")]
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
    use crate::utils::solution::{Answer, Solution};

    use super::utils::{ArmyType, ImmuneSystem};

    #[derive(Debug, Default)]
    pub struct Soln {
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            let mut boost: usize = 63;
            // TODO: need to figure out why the fight is hanging/whether it's actually hanging when it gets close.
            // and whether there is a way to calculate what it should be directly vs. simulating.
            loop {
                let mut immune_system = ImmuneSystem::default();
                immune_system.parse_input_file(filename);
                immune_system.add_boost(boost);
                while !immune_system.fights_completed() {
                    immune_system.fight();
                }
                if let Some(ArmyType::INFECTION) = immune_system.winning_army() {
                    return Answer::Usize(immune_system.num_units());
                } else {
                    boost -= 1;
                }
            }
            /*
            let mut boost_difference: usize = 1 << 10;
            let mut max_losing_boost: usize = 0;
            let mut boost = max_losing_boost + boost_difference;
            loop {
                let mut immune_system = ImmuneSystem::default();
                immune_system.parse_input_file(filename);
                immune_system.add_boost(boost);
                // TODO: add a level of indirection. apply the boost. do a binary search to find the smallest boost that has immune_system win.
                while !immune_system.fights_completed() {
                    immune_system.fight();
                }
                if let Some(ArmyType::INFECTION) = immune_system.winning_army() {
                    max_losing_boost = boost;
                    boost = max_losing_boost + boost_difference;
                } else if boost_difference == 0 {
                    return Answer::Usize(immune_system.num_units());
                } else {
                    boost_difference >>= 1;
                    boost = boost - boost_difference;
                }
            }
            */
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(51); "example_1")]
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
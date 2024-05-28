#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 11 };

pub mod part_one {
    use std::{cmp::Ordering, collections::{BTreeMap, BTreeSet, BinaryHeap, HashSet}};

    use itertools::Itertools;
    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    enum Device {
        Generator(String),
        Microchip(String),
    }

    #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
    struct FloorContents {
        generators: BTreeSet<String>,
        microchips: BTreeSet<String>,
    }

    impl FloorContents {
        fn is_valid(&self) -> bool {
            self.generators.is_empty() || self.microchips.is_subset(&self.generators)
        }

        fn is_empty(&self) -> bool {
            self.generators.is_empty() && self.microchips.is_empty()
        }

        fn remove_devices(&mut self, devices: &Vec<Device>) {
            for device in devices {
                match device {
                    Device::Generator(material) => self.generators.remove(material),
                    Device::Microchip(material) => self.microchips.remove(material),
                };
            }
        }

        fn add_devices(&mut self, devices: &Vec<Device>) {
            for device in devices {
                match device {
                    Device::Generator(material) => self.generators.insert(material.clone()),
                    Device::Microchip(material) => self.microchips.insert(material.clone()),
                };
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
    struct Facility {
        bottom_floor: u8,
        top_floor: u8,
        elevator_floor: u8,
        floor_contents: BTreeMap<u8, FloorContents>,
    }

    impl Default for Facility {
        fn default() -> Self {
            Self {
                bottom_floor: 1,
                top_floor: 4,
                elevator_floor: 1,
                floor_contents: BTreeMap::new(),
            }
        }
    }

    impl Facility {
        fn is_valid_state(&self) -> bool {
            self.floor_contents.iter().all(|(_floor, contents)| {
                contents.is_valid()
            })
        }

        fn is_end_state(&self) -> bool {
            self.floor_contents.iter()
                .all(|(floor, contents)| {
                    *floor == self.top_floor || contents.is_empty()
                })
        }

        fn next_actions(&self) -> Vec<Vec<Device>> {
            let contents = self.floor_contents.get(&self.elevator_floor).unwrap();
            let mut devices: Vec<Device> = Vec::from_iter(contents.generators.iter().map(|material| Device::Generator(material.clone())));
            let mut microchips = contents.microchips.iter().map(|material| Device::Microchip(material.clone())).collect();
            devices.append(&mut microchips);
            let mut actions: Vec<Vec<Device>> = devices.iter().map(|device| Vec::from([device.clone()])).collect();
            actions.append(&mut devices.into_iter().combinations(2).collect_vec());
            actions
        }

        fn next_valid_facilities(&self, next_actions: Vec<Vec<Device>>) -> Vec<Facility> {
            let mut next_valid_facilities: Vec<Facility> = Vec::new();
            let mut next_floors = Vec::new();
            if self.elevator_floor != self.bottom_floor { next_floors.push(self.elevator_floor - 1); }
            if self.elevator_floor != self.top_floor { next_floors.push(self.elevator_floor + 1); }
            next_actions.iter().for_each(|devices| {
                let mut removed_floor_contents = self.floor_contents.clone();
                removed_floor_contents.entry(self.elevator_floor).and_modify(|contents| contents.remove_devices(devices));
                for floor in next_floors.iter() {
                    let mut floor_contents = removed_floor_contents.clone();
                    floor_contents.entry(*floor).and_modify(|contents| contents.add_devices(devices));
                    let next_facility = Facility {
                        bottom_floor: self.bottom_floor,
                        top_floor: self.top_floor,
                        elevator_floor: *floor,
                        floor_contents,
                    };
                    if next_facility.is_valid_state() {
                        next_valid_facilities.push(next_facility);
                    }
                }
            });
            next_valid_facilities
        }

        fn next_facilities(&self) -> Vec<Facility> {
            self.next_valid_facilities(self.next_actions())
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct State {
        steps: usize,
        facility: Facility,
    }

    impl State {
        fn is_end_state(&self) -> bool {
            self.facility.is_end_state()
        }
    }
    
    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.steps.cmp(&self.steps)
                .then_with(|| self.facility.cmp(&other.facility))
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    #[derive(Debug, Default)]
    pub struct Soln {
        starting_facility: Facility,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer{
            self.parse_input_file(filename);
            Answer::Usize(self.min_steps())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let line_re = Regex::new(r"The (?<floor>[a-z]+) floor contains (?<contents>.*)\.").unwrap();
            let device_re = Regex::new(r"a (?<material>[a-z]+)(?<device_type>( generator)|(\-compatible microchip))").unwrap();
            io_utils::file_to_lines(filename).for_each(|line| {
                let captures = line_re.captures(&line).unwrap();
                let floor: u8 = match captures.name("floor").unwrap().as_str() {
                    "first" => 1,
                    "second" => 2,
                    "third" => 3,
                    "fourth" => 4,
                    _ => panic!("Unrecognized floor."),
                };
                let contents = captures.name("contents").unwrap().as_str();
                if contents == "nothing relevant" { 
                    self.starting_facility.floor_contents.insert(floor, FloorContents::default());
                    return;
                }
                device_re.captures_iter(contents).for_each(|captures| {
                    let material = captures.name("material").unwrap().as_str();
                    let device_type = captures.name("device_type").unwrap().as_str();
                    let device = match device_type {
                        " generator" => Device::Generator(material.to_string()),
                        "-compatible microchip" => Device::Microchip(material.to_string()),
                        _ => panic!("Unrecognized device type."),
                    };
                    self.starting_facility
                        .floor_contents
                        .entry(floor)
                        .or_insert(FloorContents::default())
                        .add_devices(&vec![device]);

                });
            });
        }

        fn min_steps(&self) -> usize {
            let mut visited: HashSet<Facility> = HashSet::new();
            let mut pq = BinaryHeap::from([State { steps: 0, facility: self.starting_facility.clone() }]);
            while !pq.is_empty() {
                let state = pq.pop().unwrap();
                if visited.contains(&state.facility) { continue; }
                if state.is_end_state() { 
                    return state.steps;
                }
                for facility in state.facility.next_facilities() {
                    let new_state = State { steps: state.steps + 1, facility };
                    pq.push(new_state);
                }
                visited.insert(state.facility);
            }
            panic!("Explored all states without finding solution.");
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::Usize(11); "example_1")]
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
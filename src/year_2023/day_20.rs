#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 20 };

pub mod part_one {

    use std::collections::{HashMap, HashSet, VecDeque};

    use regex::Regex;

    use crate::utils::{io_utils, solution::{Answer, Solution}};

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Pulse {
        Low,
        High,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct PulseInProcess {
        pulse: Pulse,
        sender: String,
        recipient: String,
    }

    trait Module {
        fn receive(&mut self, pulse: Pulse, sender: &str) -> VecDeque<PulseInProcess>;
        fn is_original_state(&self) -> bool;
        fn destination_modules(&self) -> &Vec<String>;
        fn set_input_modules(&mut self, inputs: Vec<String>);
    }

    #[derive(Debug, PartialEq, Eq)]
    struct FlipFlopModule {
        name: String,
        on: bool,
        destination_modules: Vec<String>,
    }

    impl Module for FlipFlopModule {
        fn receive(&mut self, pulse: Pulse, _sender: &str) -> VecDeque<PulseInProcess> {
            match pulse {
                Pulse::High => VecDeque::new(),
                Pulse::Low => {
                    self.on = !self.on;
                    let p = match self.on {
                        true => Pulse::High,
                        false => Pulse::Low,
                    };
                    self.destination_modules.iter().map(|dest| {
                        PulseInProcess {
                            pulse: p,
                            sender: self.name.clone(),
                            recipient: dest.clone(),
                        }
                    }).collect()
                },
            }
        }

        fn is_original_state(&self) -> bool {
            self.on == false
        }

        fn destination_modules(&self) -> &Vec<String> {
            &self.destination_modules
        }

        fn set_input_modules(&mut self, _inputs: Vec<String>) {}
    }

    impl FlipFlopModule {
        fn new(name: &str, destination_modules: Vec<String>) -> Self {
            Self {
                name: String::from(name),
                on: false,
                destination_modules,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct ConjunctionModule {
        name: String,
        most_recent_pulses: HashMap<String, Pulse>,
        destination_modules: Vec<String>,
    }

    impl Module for ConjunctionModule {
        fn receive(&mut self, pulse: Pulse, sender: &str) -> VecDeque<PulseInProcess> {
            self.most_recent_pulses.insert(String::from(sender), pulse);
            let p = if self.most_recent_pulses.values().all(|p| *p == Pulse::High) { Pulse::Low } else { Pulse::High };
            self.destination_modules.iter().map(|dest| {
                PulseInProcess {
                    pulse: p,
                    sender: self.name.clone(),
                    recipient: String::from(dest),
                }
            }).collect()
        }

        fn is_original_state(&self) -> bool {
            self.most_recent_pulses.values().all(|p| *p == Pulse::Low)
        }

        fn destination_modules(&self) -> &Vec<String> {
            &self.destination_modules
        }

        fn set_input_modules(&mut self, inputs: Vec<String>) {
            inputs.into_iter().for_each(|input| {
                self.most_recent_pulses.insert(input, Pulse::Low);
            })            
        }
    }

    impl ConjunctionModule {
        fn new(name: &str, destination_modules: Vec<String>) -> Self {
            Self {
                name: String::from(name),
                most_recent_pulses: HashMap::new(),
                destination_modules,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct BroadcasterModule {
        name: String,
        destination_modules: Vec<String>,
    }

    impl Module for BroadcasterModule {
        fn receive(&mut self, pulse: Pulse, _sender: &str) -> VecDeque<PulseInProcess> {
            self.destination_modules.iter().map(|dest| {
                PulseInProcess {
                    pulse,
                    sender: self.name.clone(),
                    recipient: String::from(dest),
                }
            }).collect()
        }

        fn is_original_state(&self) -> bool {
            true
        }

        fn destination_modules(&self) -> &Vec<String> {
            &self.destination_modules
        }

        fn set_input_modules(&mut self, _inputs: Vec<String>) {}
    }

    impl BroadcasterModule {
        fn new(destination_modules: Vec<String>) -> Self {
            Self {
                name: String::from("broadcaster"),
                destination_modules,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct EndModule {
        name: String,
        destination_modules: Vec<String>,
    }

    impl Module for EndModule {
        fn receive(&mut self, _pulse: Pulse, _sender: &str) -> VecDeque<PulseInProcess> {
            VecDeque::new()
        }

        fn is_original_state(&self) -> bool {
            true
        }

        fn destination_modules(&self) -> &Vec<String> {
            &self.destination_modules
        }

        fn set_input_modules(&mut self, _inputs: Vec<String>) {}
    }

    impl EndModule {
        fn new(name: &str) -> Self {
            Self {
                name: String::from(name),
                destination_modules: vec![],
            }
        }
    }

    #[derive(Default)]
    pub struct Soln {
        low_pulses: u32,
        high_pulses: u32,
        pulses_in_process: VecDeque<PulseInProcess>,
        modules: HashMap<String, Box<dyn Module>>,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            for _ in 0..1000 { // TODO: could find where it's back in original state and short-circuit
                self.process_button_push();
            }
            Answer::U32(self.product_of_pulses())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            let line_re = Regex::new(r"(?<name>.+) \-> (?<dest_names>[a-z ,]+)").unwrap();
            let name_re = Regex::new(r"(?<type>[%&])(?<name>[a-z]+)").unwrap();
            let mut conjunction_modules = HashSet::new();
            let mut dest_to_inputs: HashMap<String, Vec<String>> = HashMap::new();
            io_utils::file_to_lines(filename).for_each(|line| {
                let captures = line_re.captures(&line).unwrap();
                let name = captures.name("name").unwrap().as_str();
                let dest_names: Vec<String> = captures.name("dest_names").unwrap().as_str()
                    .split(", ")
                    .map(|d| String::from(d))
                    .collect();
                match name {
                    "broadcaster" => {
                        for dest_name in dest_names.iter() {
                            dest_to_inputs.entry(String::from(dest_name)).and_modify(|inputs| inputs.push(String::from(name))).or_insert(vec![String::from(name)]);
                        }
                        let module = BroadcasterModule::new(dest_names);
                        self.modules.insert(String::from(name), Box::new(module));
                    },
                    _ => {
                        let captures = name_re.captures(name).unwrap();
                        let module_type = captures.name("type").unwrap().as_str();
                        let name = captures.name("name").unwrap().as_str();
                        for dest_name in dest_names.iter() {
                            dest_to_inputs.entry(String::from(dest_name)).and_modify(|inputs| inputs.push(String::from(name))).or_insert(vec![String::from(name)]);
                        }
                        match module_type {
                            "%" => {
                                let module = FlipFlopModule::new(name, dest_names);
                                self.modules.insert(String::from(name), Box::new(module));
                            },
                            "&" => {
                                let module = ConjunctionModule::new(name, dest_names);
                                self.modules.insert(String::from(name), Box::new(module));
                                conjunction_modules.insert(String::from(name));
                            },
                            _ => panic!("Unrecognized module type"),
                        }
                    }
                };
            });
            dest_to_inputs.keys()
                .for_each(|dest| {
                    self.modules.entry(String::from(dest)).or_insert(Box::new(EndModule::new(dest)));
                });
            dest_to_inputs.into_iter()
                .filter(|(dest, _inputs)| {
                    conjunction_modules.contains(dest)
                })
                .for_each(|(dest, inputs)| {
                    self.modules.get_mut(&dest).unwrap().as_mut().set_input_modules(inputs);
                });
        }

        fn process_button_push(&mut self) {
            self.pulses_in_process.push_back(PulseInProcess { sender: String::from("button"), pulse: Pulse::Low, recipient: String::from("broadcaster")});
            while !self.pulses_in_process.is_empty() {
                let pip = self.pulses_in_process.pop_front().unwrap();
                self.process_pip(pip);
            }
        }

        fn process_pip(&mut self, pip: PulseInProcess) {
            match pip.pulse {
                Pulse::Low => self.low_pulses += 1,
                Pulse::High => self.high_pulses += 1,
            }
            let recipient = self.modules.get_mut(&pip.recipient)
                .expect("Recipient module should exist");
            self.pulses_in_process.append(&mut recipient.receive(pip.pulse, &pip.sender));
        }

        fn product_of_pulses(&self) -> u32 {
            self.low_pulses * self.high_pulses
        }

    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::U32(32_000_000); "example_1")]
        #[test_case(2, Answer::U32(11_687_500); "example_2")]
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
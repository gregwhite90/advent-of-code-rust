#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2023, day: 20 };

mod utils {
    use std::collections::{HashMap, HashSet, VecDeque};

    use regex::Regex;

    use crate::utils::io_utils;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Pulse {
        Low,
        High,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct PulseInProcess {
        pub pulse: Pulse,
        pub sender: String,
        pub recipient: String,
    }

    pub trait Module {
        fn receive(&mut self, pulse: Pulse, sender: &str) -> VecDeque<PulseInProcess>;
        fn is_original_state(&self) -> bool;
        fn destination_modules(&self) -> &Vec<String>;
        fn set_input_modules(&mut self, inputs: Vec<String>);
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct FlipFlopModule {
        pub name: String,
        pub on: bool,
        pub destination_modules: Vec<String>,
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
        pub fn new(name: &str, destination_modules: Vec<String>) -> Self {
            Self {
                name: String::from(name),
                on: false,
                destination_modules,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct ConjunctionModule {
        pub name: String,
        pub most_recent_pulses: HashMap<String, Pulse>,
        pub destination_modules: Vec<String>,
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
        pub fn new(name: &str, destination_modules: Vec<String>) -> Self {
            Self {
                name: String::from(name),
                most_recent_pulses: HashMap::new(),
                destination_modules,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct BroadcasterModule {
        pub name: String,
        pub destination_modules: Vec<String>,
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
        pub fn new(destination_modules: Vec<String>) -> Self {
            Self {
                name: String::from("broadcaster"),
                destination_modules,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct EndModule {
        pub name: String,
        pub destination_modules: Vec<String>,
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
        pub fn new(name: &str) -> Self {
            Self {
                name: String::from(name),
                destination_modules: vec![],
            }
        }
    }

    pub fn parse_input_file(filename: &str) -> HashMap<String, Box<dyn Module>> {
        let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
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
                    modules.insert(String::from(name), Box::new(module));
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
                            modules.insert(String::from(name), Box::new(module));
                        },
                        "&" => {
                            let module = ConjunctionModule::new(name, dest_names);
                            modules.insert(String::from(name), Box::new(module));
                            conjunction_modules.insert(String::from(name));
                        },
                        _ => panic!("Unrecognized module type"),
                    }
                }
            };
        });
        dest_to_inputs.keys()
            .for_each(|dest| {
                modules.entry(String::from(dest)).or_insert(Box::new(EndModule::new(dest)));
            });
        dest_to_inputs.into_iter()
            .filter(|(dest, _inputs)| {
                conjunction_modules.contains(dest)
            })
            .for_each(|(dest, inputs)| {
                modules.get_mut(&dest).unwrap().as_mut().set_input_modules(inputs);
            });
        modules
    }
}

pub mod part_one {

    use std::collections::{HashMap, VecDeque};

    use crate::utils::solution::{Answer, Solution};
    use super::utils::{self, Module, Pulse, PulseInProcess};

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
            for _ in 0..1000 { // TODO: could try to find where it's back in original state and short-circuit
                self.process_button_push();
            }
            Answer::U32(self.product_of_pulses())
        }
    }

    impl Soln {
        fn parse_input_file(&mut self, filename: &str) {
            self.modules = utils::parse_input_file(filename);
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

/// This requires at least a partial decompilation approach, 
/// as a brute force approach sending button pushes has an 
/// infeasible runtime. There are 4 loops that operate in
/// parallel. Each loop feeds into a conjunction module
/// which feeds directly and solely into another conjunction
/// module. Those 4 conjunction modules are the sole inputs
/// to a conjunction module, which has only one destination
/// module: the one we are trying to track when it receives
/// its first low pulse.
/// 
/// If the 4 independent loops are periodic (there is a calculable
/// number of button pushes, after which the conjunction module
/// at the end of the loop sends a low pulse, on repeat), the 
/// number of button pushes after which all 4 loops will emit a
/// low pulse is the least common multiple of the periods of each
/// loop.
/// 
/// This solution actually calculates the product of the periods,
/// and will thus only work if the periods share no common denominator
/// other than 1. A more generalized improvement on this would
/// calculate the least common multiple instead of the product.
/// 
/// The diagram looks like this:
/// 
/// ```mermaid
/// flowchart TD
///     button --> broadcaster
///     broadcaster --> %kr
///     broadcaster --> %zb
///     broadcaster --> %sm
///     broadcaster --> %xd
///     %kr --> %hh
///     %kr --> &vt
///     %hh --> %dh
///     %hh --> &vt
///     %dh --> %kq
///     %kq --> %lm
///     %lm --> %hn
///     %hn --> %qk
///     %hn --> &vt
///     %qk --> %cb
///     %cb --> %hf
///     %cb --> &vt
///     %hf --> %ch
///     %hf --> &vt
///     %ch --> %kd
///     %ch --> &vt
///     %kd --> %nb
///     %kd --> &vt
///     %nb --> &vt
///     &vt --> %dh
///     &vt --> %kr
///     &vt --> %kq
///     &vt --> %lm
///     &vt --> %qk
///     &vt --> &lz
///     &lz --> &bn
///     &bn --> rx
///     %cf --> %hl
///     %cf --> &qt
///     %hm --> %jp
///     %vr --> &qt
///     %vr --> %sl
///     %gq --> %hm
///     %gq --> &nl
///     %sl --> %jx
///     %sl --> &qt
///     &pl --> &bn
///     %kx --> &dq
///     %fr --> %qf
///     %rh --> %vr
///     &dq --> &mz
///     &dq --> %ml
///     &dq --> %xd
///     &dq --> %fb
///     &dq --> %xs
///     &dq --> %rc
///     &dq --> %rt
///     %bv --> &nl
///     %jv --> %rh
///     %jv --> &qt
///     %nd --> %hp
///     %gj --> %bv
///     %gj --> &nl
///     %lv --> %xs
///     %lv --> &dq
///     %sm --> &qt
///     %sm --> %nd
///     %nt --> %jv
///     %jx --> %cf
///     %hl --> &qt
///     %hl --> %ng
///     &qt --> %sm
///     &qt --> %rh
///     &qt --> %nd
///     &qt --> %jx
///     &qt --> %nt
///     &qt --> &pl
///     %bh --> &nl
///     %bh --> %fr
///     %gx --> %mh
///     %gx --> &dq
///     %hp --> %nt
///     %hp --> &qt
///     %rc --> %lv
///     &mz --> &bn
///     %qf --> %rd
///     %qf --> &nl
///     %sk --> &nl
///     %sk --> %bh
///     %rb --> &nl
///     %rb --> %sk
///     %fb --> %rt
///     %mh --> &dq
///     %mh --> %kx
///     %rt --> %mt
///     %xd --> &dq
///     %xd --> %fb
///     %ml --> %ts
///     %mt --> %rc
///     %mt --> &dq
///     %ts --> %gx
///     %ts --> &dq
///     %rd --> &nl
///     %rd --> %gq
///     %zb --> &nl
///     %zb --> %rb
///     &nl --> %fr
///     &nl --> %zb
///     &nl --> %hm
///     &nl --> &zm
///     &zm --> &bn
///     %ng --> &qt
///     %xs --> %ml
///     %jp --> &nl
///     %jp --> %gj
pub mod part_two {

    use std::collections::{HashMap, HashSet, VecDeque};

    use crate::utils::solution::{Answer, Solution};
    use super::utils::{self, Module, Pulse, PulseInProcess};

    #[derive(Debug, PartialEq, Eq)]
    struct Period {
        module_name: String,
        period: u64,
        confirmed: bool,        
    }

    pub struct Soln {
        button_pushes: u64,
        pulses_in_process: VecDeque<PulseInProcess>,
        modules: HashMap<String, Box<dyn Module>>,
        critical_modules: HashSet<String>,
        low_pulse_periods: HashMap<String, Period>, // tracks the period of the low pulses sent from the critical modules.
    }

    impl Default for Soln {
        fn default() -> Self {
            Self::with_critical_modules(
                HashSet::from([
                    String::from("vt"),
                    String::from("qt"),
                    String::from("dq"),
                    String::from("nl"),
                ]))
        }
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.parse_input_file(filename);
            while self.low_pulse_periods.len() < self.critical_modules.len() || self.low_pulse_periods.values().any(|period| !period.confirmed) {
                self.process_button_push();
            }
            // TODO: this should actually calculate the least common multiple, not the product.
            // This will be incorrect if the periods share a greatest common denominator that is not 1.
            Answer::U64(self.low_pulse_periods.values().map(|period| period.period).product())
        }
    }

    impl Soln {
        fn with_critical_modules(critical_modules: HashSet<String>) -> Self {
            Self {
                button_pushes: 0,
                pulses_in_process: VecDeque::new(),
                modules: HashMap::new(),
                critical_modules,
                low_pulse_periods: HashMap::new(),
            }
        }
        
        fn parse_input_file(&mut self, filename: &str) {
            self.modules = utils::parse_input_file(filename);
        }

        fn process_button_push(&mut self) {
            self.button_pushes += 1;
            self.pulses_in_process.push_back(PulseInProcess { sender: String::from("button"), pulse: Pulse::Low, recipient: String::from("broadcaster")});
            while !self.pulses_in_process.is_empty() {
                let pip = self.pulses_in_process.pop_front().unwrap();
                if self.critical_modules.contains(&pip.sender) && pip.pulse == Pulse::Low {
                    self.low_pulse_periods.entry(pip.sender.clone())
                        .and_modify(|period| {
                            if self.button_pushes > period.period {
                                if self.button_pushes % period.period == 0 {
                                    period.confirmed = true;
                                } else {
                                    panic!("non-periodic critical module found.");
                                }
                            }
                        })
                        .or_insert(Period { 
                            module_name: pip.sender.clone(),
                            period: self.button_pushes,
                            confirmed: false,
                        });
                }
                self.process_pip(pip);
            }
        }

        fn process_pip(&mut self, pip: PulseInProcess) {
            let recipient = self.modules.get_mut(&pip.recipient)
                .expect("Recipient module should exist");
            self.pulses_in_process.append(&mut recipient.receive(pip.pulse, &pip.sender));
        }

    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(3, "vt", Answer::U64(4_003); "example_3")]
        #[test_case(4, "qt", Answer::U64(3_797); "example_4")]
        #[test_case(5, "dq", Answer::U64(3_881); "example_5")]
        #[test_case(6, "nl", Answer::U64(3_823); "example_6")]
        fn examples_are_correct(example_key: u8, critical_module: &str, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::with_critical_modules(HashSet::from([String::from(critical_module)])),
                example_key,
                answer,
                &DAY,
            );
        }
    }    

}